# Pirates Reborn

Jeu indépendant en Rust sur [gotoo-pixel-engine (GPE)](https://github.com/gotoo77/gotoo-pixel-engine), héritier spirituel de **Pirates! sur Amstrad CPC**. Liberté de navigation, exploration maritime et aventure émergente constituent le cœur du projet. À terme, commerce, ressources, ports, factions, réputation, diplomatie, rencontres et carrière du capitaine pourraient se nourrir mutuellement : la profondeur doit venir des interactions entre systèmes simples.

Carte, graphismes, textes et données seront originaux. Aucun code ou asset propriétaire n'est repris ; une image DSK n'est pas nécessaire.

## Statut

**P0 : investigation et cadrage, 24 novembre 2026. Aucun gameplay implémenté.** Le dépôt était vide, sans commit, sur `feat/p0-project-foundation`, avec `origin` configuré vers `gotoo77/gpe_pirates_reborn`. Il ne contient encore ni `Cargo.toml`, ni workspace, ni lockfile, ni CI Rust. Les seuls éléments exécutables P0 sont l'utilitaire `up.sh` et ses tests.

La décision de faisabilité est **GO WITH CONDITIONS** : approuver P1, fixer une révision GPE auditée, puis valider réellement le démarrage natif et navigateur. Cette décision n'autorise pas l'implémentation. Aucun changement de GPE Core, des autres jeux, aucune nouvelle dépendance, aucun push ou merge n'est inclus dans P0.

## Documents

- [Analyse du jeu de référence](docs/reference-analysis.md) : faits documentés, déductions, propositions et limites des sources CPC.
- [Audit GPE](docs/gpe-capability-audit.md) : révisions, API et fichiers inspectés, conventions, contraintes et conditions de démarrage.
- [Spécification P1](docs/p1-navigation-spec.md) : petite traversée jouable, vent, contrôles, collisions et critères mesurables.
- [Roadmap](docs/roadmap.md) : P0 à P5 ; P2 et suivantes restent des hypothèses.

## Première tranche envisagée

**P1 — Maritime Navigation Sandbox** : une carte originale fixe, plusieurs îles, trois ports, un navire, une caméra, des collisions, un vent lisible et un HUD minimal. Boucle : partir d'un port → choisir un cap → contourner les terres → atteindre un autre port.

Le cadrage propose une vue du dessus à **640 × 360**, fenêtre initiale **1280 × 720**, à confirmer par une validation visuelle et de performance. Les ports sont des repères d'arrivée, sans services. Combat naval, duels, commerce, génération procédurale, politique, IA des factions, persistance, multijoueur et éditeur sont hors P1. Aucune abstraction n'est prévue pour les anticiper.

## Prérequis

Pour utiliser `up.sh` : Bash et Git, un clone avec au moins un commit, une branche courante avec une branche distante suivie existante, un arbre propre et un accès au distant.

Pour les tests du script : Python 3, bibliothèque standard uniquement, Bash et Git. Les tests n'utilisent que des dépôts temporaires locaux et un substitut de Cargo qui enregistre les arguments. Ils ne compilent aucun code Rust et ne contactent pas GitHub.

Pour la future P1, après approbation : Rust compatible avec le minimum GPE **1.97.1**, Cargo, rustfmt et Clippy ; sous Linux, `pkg-config`, ALSA et libudev de développement. La cible Web confirmée dans les sources GPE est `wasm32-unknown-unknown`, avec `wasm-bindgen` **0.2.127**, navigateur compatible WebGPU et serveur HTTP local / HTTPS. Les installations et le packaging du jeu restent à effectuer en P1.

Sur la machine inspectée : Rust/Cargo 1.99.0, `wasm-bindgen` 0.2.127 et bibliothèques ALSA/libudev détectés ; **cible WASM absente**. Aucune compilation native ou WASM de Pirates Reborn n'est possible sans manifeste.

## Actualiser et compiler

Depuis la racine :

```bash
./up.sh
```

Depuis un autre répertoire :

```bash
/home/gduval/dev/gpe_pirates_reborn/up.sh
```

Le script se place dans son propre répertoire, inspecte Git avant toute mise à jour et refuse les modifications suivies, indexées, non suivies ou dans les sous-modules. Les fichiers ignorés, dont `target/`, ne rendent pas le dépôt sale. Il refuse aussi HEAD détachée, dépôt sans commit, suivi absent ou local, et opérations Git en cours.

Il conserve la branche et lance `git pull --ff-only --no-rebase`, en désactivant l'autostash pour cette commande. Une divergence ou une erreur réseau interrompt le script. Il vérifie à nouveau la branche et l'état local avant de compiler.

Si `Cargo.toml` existe après la mise à jour, il exécute **`cargo build --locked --release`** et propage toute erreur. Le cache Cargo est conservé ; pas de `cargo clean`. Sans manifeste, il affiche **« compilation indisponible »** et termine avec le code 0 pour la seule actualisation Git réussie. Il ne crée aucun projet ni lockfile. Il ne pousse rien, ne change pas de branche et n'effectue ni stash, ni reset, ni rebase, ni commit de merge. Une compilation échouée ne défait pas un fast-forward déjà effectué.

Dans l'état de livraison P0, les nouveaux fichiers non commités font légitimement refuser le script avant le pull ; le dépôt initial n'avait par ailleurs ni commit ni upstream. Leur publication ou la configuration d'un suivi devront être décidées explicitement. Ne pas lancer le script en même temps qu'une autre opération modifiant le dépôt.

`up.sh` sert à actualiser et compiler ; il ne remplace pas les tests et Clippy.

## Validation P0

```bash
bash -n up.sh
test -x up.sh
python3 -B -m unittest discover -s tests -v
git diff --check
```

Les règles de navigation et leur stratégie de tests sont spécifiées, sans code gameplay. Cargo fmt/check/test/build et **Clippy: NOT RUN** : aucun manifeste ni code Rust, aucune modification Cargo. Les validations GPE ont été lues, pas exécutées ; aucune CI verte n'est revendiquée. L'audit décrit les contrôles obligatoires de P1, dont Clippy strict avec `-D warnings`.
