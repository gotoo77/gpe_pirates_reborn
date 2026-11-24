# Audit des capacités GPE — P0

Inspection des sources locales le **24 novembre 2026**. « Confirmé » signifie ici présent et lu dans les sources, pas compilé ou essayé sur le matériel. Aucun checkout, fichier GPE ou autre jeu n'a été modifié. Aucun fetch n'a été fait : les références distantes ci-dessous sont celles déjà présentes localement.

## État initial et provenance

Pirates Reborn : `/home/gduval/dev/gpe_pirates_reborn`, branche `feat/p0-project-foundation`, **aucun commit**, arbre initial vide hors `.git`, remote `https://github.com/gotoo77/gpe_pirates_reborn.git`. Aucun AGENTS.md physique trouvé dans ce dépôt ou ses répertoires ancêtres inspectés ; les instructions AGENTS fournies dans la session s'appliquent, avec RTK, rappel/stockage ICM et politique Rust stricte. Aucune convention documentaire existante à remplacer. Aucun manifeste, workspace, test gameplay ou contrôle CI initial.

Le checkout réel du moteur est `/home/gduval/dev/gotoo-pixel-engine` :

| Référence locale | Révision complète | Constat |
| --- | --- | --- |
| HEAD, `game/void-canticle` | `39eec6bc0ebe0f3e6d323154014bc6c8a30b5b69` | Checkout propre, version Cargo 0.1.0 ; upstream marqué supprimé. Source effectivement inspectée. |
| `main` | `7d081e7d15b9dfe2d7e86ed0469e4db5bf4856eb` | Manifeste 0.1.0 inspecté ; ce nom local n'est pas la base récente des autres jeux. |
| `origin/main` | `8cbceda0c9b0cc55ee5deb7d5e173676ffc1d4f1` | Version 0.2.0 ; sources inspectées par `git show` et `git archive` dans un répertoire temporaire, sans changement de branche. **Base proposée pour P1.** |

Les références techniques qui suivent portent sur **8cbceda…**, sauf indication explicite. Elles restent vérifiables dans le dépôt moteur avec `git show 8cbceda0c9b0cc55ee5deb7d5e173676ffc1d4f1:chemin`. Les liens GitHub sont fixés sur ce commit, sans dépendre du nom d'une branche. La proposition n'affirme pas que ce commit est le dernier publié ou que les builds y passent aujourd'hui.

## Capacités vérifiées

| Besoin | Fichier et symboles vérifiés | Conséquence pour P1 |
| --- | --- | --- |
| Boucle et démarrage | [`src/platform.rs`](https://github.com/gotoo77/gotoo-pixel-engine/blob/8cbceda0c9b0cc55ee5deb7d5e173676ffc1d4f1/src/platform.rs) : `run`, `EngineConfig`, `Game::update`, `GameResult`, `Frame` ; exports dans `src/lib.rs`. | Un propriétaire du jeu ; mise à jour et dessin dans le callback existant. Pas de nouveau runtime. |
| Temps | Même fichier : `Frame::delta_time`, `MAX_FRAME_DELTA`, `simulation_delta_time`, gestion de `WindowEvent::Focused`. | `Duration` variable, limitée à 100 ms ; base temporelle remise à zéro lors de transitions de focus/reprise. Ce n'est pas une simulation à pas fixe. Le jeu doit accumuler le temps reçu ; aucune avance fondée sur l'horloge civile. Pas de garantie de pause gameplay à la perte de focus. |
| Rendu logiciel | [`src/framebuffer.rs`](https://github.com/gotoo77/gotoo-pixel-engine/blob/8cbceda0c9b0cc55ee5deb7d5e173676ffc1d4f1/src/framebuffer.rs) : `Framebuffer::{clear,draw,draw_line,draw_rect,fill_rect,draw_circle,fill_circle,draw_text_scaled,text_size,draw_image_region}` ; `Pixel` dans `src/pixel.rs`. | Îles, coque orientée par segments, ports et HUD originaux possibles sans assets ni bibliothèque supplémentaire. |
| Images et sprites | `src/image.rs` : `Image::{decode_png,from_rgba8}`, `ImageRegion` ; `src/sprite.rs` : `Sprite::{new,draw,draw_centered}`. | Chargement PNG et blits confirmés. Pas de rotation arbitraire exposée par `Sprite` dans les méthodes inspectées ; P1 peut dessiner sa coque par segments selon le cap. |
| Résolution et présentation | [`src/viewport.rs`](https://github.com/gotoo77/gotoo-pixel-engine/blob/8cbceda0c9b0cc55ee5deb7d5e173676ffc1d4f1/src/viewport.rs) : `Viewport::{new,map_surface_position}`, `pixel_art_scale` ; `src/renderer.rs` : création du sampler en `FilterMode::Nearest`, `Renderer::{resize,render}`. | Ratio conservé et viewport centré. **Les échelles fractionnaires restent possibles** : le moteur ne force pas tous les agrandissements à un entier. Fenêtre initiale ×2 proposée ; tester redimensionnement et HiDPI. |
| Clavier | [`src/input.rs`](https://github.com/gotoo77/gotoo-pixel-engine/blob/8cbceda0c9b0cc55ee5deb7d5e173676ffc1d4f1/src/input.rs) : `Key`, `Input::key`, `ButtonState::{pressed,held,released}` ; `platform.rs::key_from_winit`. | Flèches, Space, Escape et R vérifiés aux deux révisions. `Key::Enter` existe à 8cbceda, pas au checkout 39eec6b. Ne pas inventer un mapping Z/Q ou une touche P. |
| Manette native et navigateur | `Input::{gamepad_ids,gamepad_info,gamepad_button,gamepad_axis,gamepad_axis_capability,gamepad_connection_events}`, `GamepadButton`, `GamepadAxis`, `GamepadCapability` ; `src/gamepad.rs`, `src/gamepad/browser.rs`, `src/gamepad_profile.rs`. | D-pad, boutons et stick gauche disponibles. À 8cbceda : axes analogiques publics et calibration ; à 39eec6b, le contrat public inspecté ne fournit que les directions/boutons. Le navigateur traite le mapping standard ; disponibilité matérielle à vérifier. |
| Liaison d'actions | `src/control.rs` : `ActionId`, `ControlMap::{bind_key,bind_gamepad,update,action}`. | Réutilisable pour des commandes numériques. Pas de framework d'input à créer. Une simple conversion de commandes vers les ticks suffit. |
| HUD et UI | [`src/ui/mod.rs`](https://github.com/gotoo77/gotoo-pixel-engine/blob/8cbceda0c9b0cc55ee5deb7d5e173676ffc1d4f1/src/ui/mod.rs) : `draw_panel`, `draw_text_centered`, exports `Ui`, `UiState`, `PauseGame`, `PauseConfig` ; texte bitmap dans `Framebuffer`. | HUD minimal possible avec les primitives et le texte existants. Ne pas migrer vers les API UI expérimentales pour P1. Tester les glyphes réellement utilisés ; libellés courts ASCII possibles. |
| Audio | `src/audio.rs` : trait `Audio`, `register_wav`, `play`, `start_loop`, `stop_loop`, `SoundBank`, `NoopAudio` ; `Frame::audio` ; backends rodio/Web Audio. | WAV et boucles présents ; le trait autorise des erreurs de capacité, ne pas déduire un succès audio universel. Aucun son requis en P1. Déverrouillage navigateur après interaction à vérifier si audio ajouté plus tard. |
| Scènes, caméra et collisions | `src/lib.rs` et exports inspectés ; `Game` ; `Rect::{contains,intersects}` dans `viewport.rs`. Aucun gestionnaire public de scènes, caméra monde ou solveur de collisions identifié. | Un état de jeu local suffit. Caméra = soustraction d'une origine bornée ; collision = règle locale navire/rectangles. `Rect::intersects` ne fournit pas un balayage continu. Aucun ECS, moteur physique ou abstraction de scènes à ajouter. |

## Jeux locaux et conventions effectivement inspectés

Les dossiers `gpe_*` accessibles ont été inventoriés ; l'étude détaillée s'est limitée aux exemples pertinents ci-dessous. Ce n'est pas une validation de tous les jeux locaux.

| Projet et HEAD | État initial / observations réutilisables |
| --- | --- |
| `gpe_druid_reborn` — `81ed6b27021c754b488faba31fc2a46533b8d176` | Propre, `main`. `Cargo.toml` : dépendance Git GPE avec `rev` immuable `ff292ec…`. `src/app.rs::AppScreen` porte les écrans côté jeu ; `src/game.rs::camera_origin` borne la caméra ; `src/visual_renderer.rs` soustrait l'origine monde. `.github/workflows/ci.yml` : fmt, tests, Clippy strict et release. Référence de méthode, sans copie du jeu. |
| `gpe_smartboyhero` — `a646c67ba98af2d81e3fca50de7b94765ebe2adf` | Branche `feat/iso-gpe-ui-presentation`, **Cargo.lock déjà modifié par l'utilisateur**. Rien touché. Pin GPE `c81955e…`, feature `outline-fonts`. `src/main.rs`, `src/bin/sbh-web.rs` : entrées native/Web séparées, même logique ; `scripts/package_web.py` : Cargo locked release, wasm-bindgen, HTML et assets dans dist. `scripts/run-ci-gate.sh` localise sa racine depuis `BASH_SOURCE`, utilise `set -euo pipefail` ; son gate n'inclut pas Clippy et ne suffit donc pas à notre politique. |
| `gpe_void-canticle` — `340cdbb0ff42f662102c870366f12586d1cadc45` | Propre, `main`. Manifeste : pin GPE `c81955e…`, diagnostics, branchements de dépendances natif/WASM. Ne pas transférer son périmètre ou ses dépendances. |
| `gpe_linkpuzzle` — `7e98905e985c4d1250454398009c3f34ef0e38cc` | Propre, `main`. Manifeste : Rust 2021 et GPE optionnel par feature `gpe`, pin `b98066d…`. Les jeux n'ont pas tous la même convention ; pas de workspace GPE à supposer. |
| `gpe_arcade` — `844645386808abf8a981c4ec55aaf07499847d9a` | Propre, branche `feat/arcade-shared-pause-settings-v0.4` en retard de 54 commits sur son suivi local. Manifeste : pins Git des jeux et GPE `cf5d279…`, feature Web séparée. Ce checkout n'est pas une base à recopier. |
| `gpe_smog_city` — `aa692ce139daa1215f7bc546f465050bd6776e74` | Propre, `feat/gameplay-events-telemetry`. Pin GPE `8cbceda…`. `src/game.rs` centralise 640 × 360 et ×2 ; `src/main.rs` adapte `Frame` au jeu. C'est une preuve de configuration dans les sources, pas une mesure de performance pour Pirates. |

Aucun `up.sh` existant trouvé dans les projets inspectés. Le nouveau script reprend la localisation de racine et l'arrêt sur erreur des scripts GPE ; il ne lance aucun script d'installation ou d'actualisation d'un autre dépôt.

## Build, tests et packaging

À 8cbceda, `Cargo.toml` indique Rust **2024**, minimum **1.97.1**, package GPE **0.2.0**, features par défaut vides ; `outline-fonts`, `diagnostics` et `diagnostics-fault-injection` sont optionnelles. **PROPOSÉ : P1 sans feature optionnelle**, dépendance Git avec révision complète et lockfile versionné, selon la convention majoritaire observée. Ne pas utiliser une dépendance `path` vers le checkout actif ancien.

Le gate canonique GPE est `python3 scripts/dev.py check` (`scripts/check.sh` est un wrapper). `command_check` exécute le contrôle de séparation, fmt, tests lib/bin/examples/intégration, **Clippy `--all-targets -- -D warnings`**, puis diff-check. Ces commandes Cargo ne sont pas toutes `--locked` dans GPE : notre politique de projet devra garder le verrouillage. Les scripts sont lus, **pas lancés pendant P0**.

La CI GPE `.github/workflows/ci.yml` confirme des jobs Linux natif et Web. Linux installe `libasound2-dev`, `libudev-dev`, `pkg-config`. `scripts/dev.py::{cargo_build_web,wasm_bindgen,command_check_web,command_check_web_browser}` construit `web_demo` pour `wasm32-unknown-unknown`, génère le glue JS avec `wasm-bindgen --target web` et prévoit des tests de démarrage Chromium. `examples/web_demo.rs::start` utilise `#[wasm_bindgen(start)]`, `EngineConfig` et `run`. `build-web` et le jeu Smart Boy Hero démontrent les conventions de paquets JS/WASM/HTML ; aucun pipeline Pirates n'existe encore.

**Cibles confirmées dans les sources : Linux natif et Web/WASM avec WebGPU.** L'existence du code WASM n'établit pas sa compatibilité avec tous les navigateurs ; aucun fallback WebGL, ni support testé Windows/macOS/mobile n'est revendiqué ici. P1 doit tester un navigateur compatible, en contexte local sûr ou HTTPS, et enregistrer sa version ainsi que l'adaptateur GPU.

## Résolution et limites

`EngineConfig` sépare taille framebuffer et fenêtre ; `validate_config` exige des dimensions non nulles, sans imposer 320 × 180. Le renderer charge une texture RGBA et demande `wgpu::Limits::default()` ; les limites matérielles et les performances ne sont pas mesurées par cet audit.

**PROPOSÉ : 640 × 360 / fenêtre 1280 × 720.** Un framebuffer RGBA à cette taille occupe 921 600 octets ; à 60 images/s, une copie complète représente environ 55,3 Mo/s, simple estimation arithmétique qui exclut les autres coûts. Le filtrage nearest existe ; à une échelle fractionnaire, les pixels peuvent avoir des tailles apparentes irrégulières. Vérifier le ×1, le ×2, le redimensionnement et les glyphes. Une caméra monde n'est pas le viewport d'affichage. Pas de système multirésolution ou projection isométrique à créer.

## Capability preflight et décision

**GO WITH CONDITIONS pour préparer P1 ; aucun feu vert d'exécution donné par P0.** Aucun manque d'API identifié n'impose une modification de Core. Le vent, la carte, le déplacement, le pas fixe et les collisions sont des règles du jeu à écrire seulement après approbation.

| Avant / pendant P1 autorisée | Condition de passage ou arrêt |
| --- | --- |
| Dépendance | Reconfirmer puis fixer le commit audité. Si une autre révision est retenue, réauditer les symboles utilisés avant code. Ne pas supposer que HEAD est équivalent. |
| Démarrage minimal natif et WASM | Installer la cible manquante dans le cadre P1 autorisé, puis construire et démarrer les deux entrées avec les API existantes, avant gameplay. Si un besoin indispensable manque : **STOP**, diagnostic factuel et décision utilisateur ; aucune correction Core implicite. |
| Rendu / input | Mesurer la lisibilité et le coût à 640 × 360 ; vérifier une manette réelle, notamment sous WSL si applicable. Absence de mesure ou périphérique inaccessible = validation encore ouverte, pas succès. |
| Gate Rust Pirates futur | Tests ciblés puis commande canonique du futur dépôt. Au minimum `cargo fmt --all -- --check`, `cargo check --locked`, `cargo test --locked`, `cargo clippy --locked --all-targets -- -D warnings`, `git diff --check`, plus build release et cible WASM. Sélectionner seulement les features supportées ; ne pas ajouter `--all-features` par défaut. |

**Validation P0 :** inspection et cohérence documentaire ; script Bash testé séparément selon README. **Cargo fmt/check/test/build : NOT RUN. Clippy: NOT RUN**, absence de manifeste/code Rust dans Pirates, aucun fichier Cargo/Rust modifié. La cible WASM n'est pas installée localement ; aucun build, benchmark, test navigateur ou essai de manette du futur jeu n'a été exécuté. Ce sont les principales limites, pas des échecs de compilation observés.
