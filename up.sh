#!/usr/bin/env bash
set -Eeuo pipefail

step="initialisation"
trap 'code=$?; printf "[up] ERREUR pendant %s (code %s). Arrêt.\n" "$step" "$code" >&2; exit "$code"' ERR

die() {
    printf '[up] ERREUR : %s\n' "$1" >&2
    exit 1
}

step="localisation du projet"
project_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
cd -- "$project_root"

step="vérification du dépôt Git"
git_root="$(git rev-parse --show-toplevel 2>/dev/null)" || die "aucun dépôt Git à cet emplacement."
[[ "$(cd -- "$git_root" && pwd -P)" == "$project_root" ]] || die "up.sh doit être à la racine du dépôt Git."

step="vérification des modifications locales"
status="$(git status --porcelain=v1 --untracked-files=all --ignore-submodules=none)"
if [[ -n "$status" ]]; then
    printf '%s\n' "$status" >&2
    die "modifications locales présentes (index, fichiers suivis ou non suivis). Aucune mise à jour."
fi

step="vérification de la branche courante"
branch="$(git symbolic-ref --quiet --short HEAD)" || die "aucune branche courante : HEAD détachée."
git rev-parse --verify HEAD >/dev/null 2>&1 || die "aucun commit : mise à jour indisponible."
for marker in MERGE_HEAD CHERRY_PICK_HEAD REVERT_HEAD BISECT_LOG rebase-merge rebase-apply; do
    [[ ! -e "$(git rev-parse --git-path "$marker")" ]] || die "opération Git en cours ($marker) : terminez-la avant de relancer."
done

step="vérification de la branche distante suivie"
upstream="$(git rev-parse --abbrev-ref --symbolic-full-name '@{upstream}' 2>/dev/null)" \
    || die "aucune branche distante suivie disponible pour $branch. Configurez-la explicitement."
git rev-parse --verify '@{upstream}' >/dev/null 2>&1 \
    || die "la branche distante suivie $upstream n'existe pas localement."
remote="$(git config --get "branch.$branch.remote")" || die "distant non configuré pour $branch."
[[ "$remote" != "." ]] || die "le suivi doit désigner un distant, pas une branche locale."
git remote get-url "$remote" >/dev/null || die "distant $remote introuvable."

printf '[up] Projet : %s\n[up] Branche conservée : %s ; suivi : %s\n' "$project_root" "$branch" "$upstream"
step="mise à jour Git (fast-forward uniquement)"
printf '[up] git pull --ff-only --no-rebase\n'
# Override user preferences that could otherwise select rebase or autostash.
git -c merge.autoStash=false -c rebase.autoStash=false pull --ff-only --no-rebase

step="vérification après mise à jour"
[[ "$(git symbolic-ref --quiet --short HEAD)" == "$branch" ]] || die "la branche courante a changé pendant la mise à jour."
[[ -z "$(git status --porcelain=v1 --untracked-files=all --ignore-submodules=none)" ]] \
    || die "modifications locales détectées après la mise à jour. Compilation annulée."

if [[ ! -f Cargo.toml ]]; then
    printf '[up] Git à jour ; compilation indisponible : Cargo.toml absent, projet Rust non initialisé.\n'
    exit 0
fi

step="compilation Rust en release"
command -v cargo >/dev/null 2>&1 || die "Cargo introuvable dans PATH."
printf '[up] cargo build --locked --release (cache Cargo conservé)\n'
cargo build --locked --release
printf '[up] Compilation release réussie.\n'
