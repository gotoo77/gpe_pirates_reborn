"""Integration checks for up.sh using only temporary, local Git repositories.

Cargo is replaced by a recorder: these checks do not compile Rust.
Run: python3 -m unittest discover -s tests -v
"""

import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest


SCRIPT = Path(__file__).resolve().parents[1] / "up.sh"


class UpTests(unittest.TestCase):
    def setUp(self):
        self.assertTrue(SCRIPT.is_file(), "up.sh must exist before the workflow can run")
        self.temp = tempfile.TemporaryDirectory(prefix="pirates-up-")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.env = os.environ.copy()
        # Keep user Git configuration and hooks out of the temporary fixtures.
        self.env.update(GIT_CONFIG_NOSYSTEM="1", GIT_CONFIG_GLOBAL=os.devnull)
        for key in list(self.env):
            if key.startswith("GIT_") and key not in {"GIT_CONFIG_NOSYSTEM", "GIT_CONFIG_GLOBAL"}:
                del self.env[key]
        self.remote = self.root / "remote.git"
        self.seed = self.root / "seed"
        self.repo = self.root / "checkout with spaces"
        self.git(self.root, "init", "--bare", "--initial-branch=main", str(self.remote))
        self.git(self.root, "init", "--initial-branch=main", str(self.seed))
        self.identity(self.seed)
        shutil.copy2(SCRIPT, self.seed / "up.sh")
        (self.seed / ".gitignore").write_text("/target/\n")
        (self.seed / "Cargo.toml").write_text('[package]\nname = "fixture"\nversion = "0.1.0"\n')
        (self.seed / "Cargo.lock").write_text("# test fixture; Cargo is a recorder\n")
        self.commit(self.seed, "initial")
        self.git(self.seed, "remote", "add", "origin", str(self.remote))
        self.git(self.seed, "push", "-u", "origin", "main")
        self.git(self.root, "clone", str(self.remote), str(self.repo))
        self.identity(self.repo)
        self.bin = self.root / "bin"
        self.bin.mkdir()
        self.log = self.root / "cargo.log"
        cargo = self.bin / "cargo"
        cargo.write_text('#!/usr/bin/env bash\nprintf "%s\\n" "$PWD" "$@" >> "$UP_TEST_CARGO_LOG"\nexit "${UP_TEST_CARGO_STATUS:-0}"\n')
        cargo.chmod(0o755)
        self.env.update(PATH=str(self.bin) + os.pathsep + self.env["PATH"], UP_TEST_CARGO_LOG=str(self.log))

    def git(self, cwd, *args):
        result = subprocess.run(["git", *args], cwd=cwd, env=self.env, text=True, capture_output=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        return result.stdout.strip()

    def identity(self, repo):
        self.git(repo, "config", "user.name", "Temporary test")
        self.git(repo, "config", "user.email", "test@example.invalid")
        self.git(repo, "config", "core.hooksPath", str(self.root / "no-hooks"))

    def commit(self, repo, message):
        self.git(repo, "add", "--all")
        self.git(repo, "commit", "-m", message)

    def advance_remote(self):
        (self.seed / "remote-change").write_text("new remote revision\n")
        self.commit(self.seed, "remote advance")
        self.git(self.seed, "push", "origin", "main")
        return self.git(self.seed, "rev-parse", "HEAD")

    def run_up(self, script=None):
        return subprocess.run([str(script or self.repo / "up.sh")], cwd=self.root, env=self.env,
                              text=True, capture_output=True)

    def rejected_without_build(self, result, diagnostic):
        self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn(diagnostic, result.stdout + result.stderr)
        self.assertFalse(self.log.exists(), "Cargo must not run after rejection")

    def test_fast_forward_from_other_directory_preserves_branch_and_cache(self):
        expected = self.advance_remote()
        cache = self.repo / "target" / "release" / "cache-sentinel"
        cache.parent.mkdir(parents=True)
        cache.write_text("keep me")
        # A local pull.rebase preference must not rebase local commits.
        self.git(self.repo, "config", "pull.rebase", "true")
        result = self.run_up()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(self.git(self.repo, "rev-parse", "HEAD"), expected)
        self.assertEqual(self.git(self.repo, "branch", "--show-current"), "main")
        self.assertEqual(cache.read_text(), "keep me")
        self.assertEqual(self.log.read_text().splitlines(), [str(self.repo), "build", "--locked", "--release"])

    def test_dirty_states_reject_before_fetch(self):
        self.advance_remote()
        old_head = self.git(self.repo, "rev-parse", "HEAD")
        old_remote = self.git(self.repo, "rev-parse", "origin/main")
        for state in ("untracked", "unstaged", "staged"):
            with self.subTest(state=state):
                path = self.repo / ("new-file" if state == "untracked" else "Cargo.toml")
                original = path.read_text() if path.exists() else None
                path.write_text("local edit\n")
                if state == "staged":
                    self.git(self.repo, "add", "Cargo.toml")
                self.rejected_without_build(self.run_up(), "modifications locales")
                self.assertEqual(path.read_text(), "local edit\n")
                self.assertEqual(self.git(self.repo, "rev-parse", "HEAD"), old_head)
                self.assertEqual(self.git(self.repo, "rev-parse", "origin/main"), old_remote)
                if original is None:
                    path.unlink()
                else:
                    path.write_text(original)
                    self.git(self.repo, "add", "Cargo.toml")

    def test_detached_head_rejected(self):
        self.git(self.repo, "checkout", "--detach")
        self.rejected_without_build(self.run_up(), "branche courante")

    def test_no_upstream_rejected(self):
        self.git(self.repo, "branch", "--unset-upstream")
        self.rejected_without_build(self.run_up(), "branche distante suivie")

    def test_missing_tracking_ref_rejected(self):
        self.git(self.repo, "update-ref", "-d", "refs/remotes/origin/main")
        self.rejected_without_build(self.run_up(), "branche distante suivie")

    def test_local_tracking_branch_rejected(self):
        self.git(self.repo, "branch", "local-base")
        self.git(self.repo, "branch", "--set-upstream-to=local-base")
        self.rejected_without_build(self.run_up(), "distant")

    def test_divergence_rejected_without_rebase_or_merge(self):
        self.advance_remote()
        (self.repo / "local-change").write_text("local commit\n")
        self.commit(self.repo, "local advance")
        head = self.git(self.repo, "rev-parse", "HEAD")
        self.git(self.repo, "config", "pull.rebase", "true")
        self.rejected_without_build(self.run_up(), "mise à jour Git")
        self.assertEqual(self.git(self.repo, "rev-parse", "HEAD"), head)
        self.assertEqual(self.git(self.repo, "status", "--porcelain"), "")

    def test_no_manifest_reports_build_unavailable(self):
        self.git(self.repo, "rm", "Cargo.toml", "Cargo.lock")
        self.commit(self.repo, "documentation only")
        result = self.run_up()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("compilation indisponible", result.stdout)
        self.assertFalse(self.log.exists())

    def test_manifest_added_by_pull_is_used(self):
        self.git(self.repo, "rm", "Cargo.toml", "Cargo.lock")
        self.commit(self.repo, "remove manifests")
        self.git(self.repo, "push", "origin", "main")
        self.git(self.seed, "pull", "--ff-only")
        (self.seed / "Cargo.toml").write_text('[package]\nname = "fixture"\nversion = "0.1.0"\n')
        self.commit(self.seed, "add manifest fixture")
        self.git(self.seed, "push", "origin", "main")
        result = self.run_up()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertTrue(self.log.exists())

    def test_cargo_failure_propagated(self):
        self.env["UP_TEST_CARGO_STATUS"] = "42"
        result = self.run_up()
        self.assertEqual(result.returncode, 42)
        self.assertIn("compilation Rust", result.stderr)

    def test_unreachable_remote_stops_before_build(self):
        self.git(self.repo, "remote", "set-url", "origin", str(self.root / "absent.git"))
        self.rejected_without_build(self.run_up(), "mise à jour Git")

    def test_in_progress_git_operation_rejected(self):
        for marker in ("MERGE_HEAD", "CHERRY_PICK_HEAD", "REVERT_HEAD", "BISECT_LOG", "rebase-merge", "rebase-apply"):
            with self.subTest(marker=marker):
                path = self.repo / ".git" / marker
                path.write_text(self.git(self.repo, "rev-parse", "HEAD") + "\n")
                self.rejected_without_build(self.run_up(), "opération Git")
                path.unlink()

    def test_unborn_branch_rejected(self):
        empty = self.root / "empty"
        self.git(self.root, "init", "--initial-branch=main", str(empty))
        (empty / ".gitignore").write_text("up.sh\n.gitignore\n")
        shutil.copy2(SCRIPT, empty / "up.sh")
        self.rejected_without_build(self.run_up(empty / "up.sh"), "aucun commit")

    def test_script_outside_repository_rejected(self):
        standalone = self.root / "up.sh"
        shutil.copy2(SCRIPT, standalone)
        self.rejected_without_build(self.run_up(standalone), "dépôt Git")


if __name__ == "__main__":
    unittest.main()
