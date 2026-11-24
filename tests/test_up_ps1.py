"""Real PowerShell integration checks, skipped explicitly when pwsh is absent.

Reuse the Bash Git fixtures/assertions, execute the actual .ps1, and replace
only the two fixtures that intentionally copy a standalone script. No emulation.
"""
import os
from pathlib import Path
import shutil
import subprocess
import unittest
import test_up

PWSH = shutil.which("pwsh") or shutil.which("powershell")
SCRIPT = Path(__file__).resolve().parents[1] / "up.ps1"


@unittest.skipUnless(PWSH, "PowerShell inaccessible : tests up.ps1 NOT RUN")
class PowerShellUpTests(test_up.UpTests):
    def setUp(self):
        super().setUp()
        shutil.copy2(SCRIPT, self.seed / "up.ps1")
        self.commit(self.seed, "PowerShell updater")
        self.git(self.seed, "push", "origin", "main")
        self.git(self.repo, "pull", "--ff-only")
        # .cmd recorder when these tests run on native Windows, where Git for
        # Windows must also be on PATH. On Linux pwsh invokes the shell recorder.
        if os.name == "nt":
            (self.bin / "cargo.cmd").write_text(
                '@echo off\necho %CD%>>"%UP_TEST_CARGO_LOG%"\n'
                'for %%A in (%*) do echo %%~A>>"%UP_TEST_CARGO_LOG%"\n'
                'if defined UP_TEST_CARGO_STATUS exit /b %UP_TEST_CARGO_STATUS%\nexit /b 0\n'
            )

    def run_up(self, script=None):
        # Invoke from elsewhere and assert caller location even on failure.
        # Parameters travel as an argument array, never interpolated PS code.
        wrapper = self.root / "invoke-up.ps1"
        wrapper.write_text(
            'param([string] $Updater)\n'
            '$before = (Get-Location).Path\n'
            '& $Updater\n$code = $LASTEXITCODE\n'
            'if ((Get-Location).Path -ne $before) { throw "caller location changed" }\n'
            'exit $code\n'
        )
        result = subprocess.run(
            [PWSH, "-NoLogo", "-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-File", str(wrapper),
             str(script or self.repo / "up.ps1")], cwd=self.root, env=self.env,
            text=True, capture_output=True,
        )
        # Preserve the shared fixture's French diagnostic assertions across
        # PowerShell editions/code pages; the updater itself uses ASCII.
        result.stdout = result.stdout.replace("mise a jour", "mise à jour").replace("operation Git", "opération Git")
        result.stderr = result.stderr.replace("mise a jour", "mise à jour").replace("operation Git", "opération Git").replace("depot Git", "dépôt Git")
        return result

    def test_unborn_branch_rejected(self):
        empty = self.root / "empty"
        self.git(self.root, "init", "--initial-branch=main", str(empty))
        (empty / ".gitignore").write_text("up.ps1\n.gitignore\n")
        shutil.copy2(SCRIPT, empty / "up.ps1")
        self.rejected_without_build(self.run_up(empty / "up.ps1"), "aucun commit")

    def test_script_outside_repository_rejected(self):
        standalone = self.root / "up.ps1"
        shutil.copy2(SCRIPT, standalone)
        self.rejected_without_build(self.run_up(standalone), "dépôt Git")

    def test_clean_sequencer_state_is_rejected(self):
        (self.repo / ".git" / "sequencer").mkdir()
        self.rejected_without_build(self.run_up(), "opération Git")

    def test_post_pull_local_changes_stop_before_cargo(self):
        self.advance_remote()
        hooks = self.root / "hooks"
        hooks.mkdir()
        hook = hooks / "post-merge"
        hook.write_text('#!/bin/sh\nprintf "from hook\\n" > postpull-local\n')
        hook.chmod(0o755)
        self.git(self.repo, "config", "core.hooksPath", str(hooks))
        self.rejected_without_build(self.run_up(), "modifications locales")
        self.assertTrue((self.repo / "postpull-local").exists())
