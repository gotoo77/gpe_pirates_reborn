# P1 — preuves et limites de validation

24 novembre 2026. Livraison locale sur `feat/p1-maritime-navigation` ; aucun commit, push, merge ou P2 dans cette mission. Le ressenti humain et la manette réelle restent des conditions d'acceptation.

## Provenance et préflight

- Départ propre : HEAD `b0ede154eb13ee3a6c98faf508a9b59100365371`, identique à `main`, `origin/main` et `origin/feat/p1-maritime-navigation`. Branche attendue et upstream vérifiés avant modification, puis confirmés avec `git ls-remote origin refs/heads/main refs/heads/feat/p1-maritime-navigation`.
- `git ls-remote https://github.com/gotoo77/gotoo-pixel-engine.git refs/heads/main` confirme **`8cbceda0c9b0cc55ee5deb7d5e173676ffc1d4f1`**. Dépendance Git `rev` complète et Cargo.lock, aucune dépendance `path`. Son manifeste déclare 0.2.0 ; **le tag Git `v0.2.0` se résout à `a345f709500aa755666fd193e755fa34a6b48c93`**, ce n'est pas notre pin.
- Sources du pin relues : `EngineConfig`, `Game::update`, `Frame::delta_time`, primitives `Framebuffer`, clavier, `gamepad_ids`, `GamepadId`, `gamepad_axis`, boutons, présentation ratio/nearest. Aucun manque d'API nécessaire identifié. Caméra, collisions et ticks restent locaux au jeu.
- Rust/Cargo 1.99.0, minimum déclaré 1.97.1, édition 2024 ; ALSA 1.2.6.1 et libudev 249 détectés par pkg-config. Cibles `x86_64-unknown-linux-gnu` et `wasm32-unknown-unknown` ; cette dernière a été installée dans le cadre de la mission.
- Lockfile résolu : wasm-bindgen **0.2.127**, CLI **0.2.127**, js-sys/web-sys 0.3.104, wasm-bindgen-futures 0.4.77. Aucune feature GPE optionnelle utilisée ; pas de combinaison `--all-features` inventée.
- Écran minimal natif 640×360 / fenêtre 1280×720 ouvert avant gameplay. Paquet minimal JS/WASM/HTML servi et rendu réellement visible dans Chromium WebGPU avant implémentation de la navigation. Le checkout moteur `game/void-canticle` est resté inchangé.

Le préflight Web a révélé deux problèmes d'environnement/intégration corrigés sans toucher Core : le chargeur interprétait comme fatale l'exception de transfert de contrôle documentée de winit 0.30 ; il reconnaît maintenant uniquement ses deux marqueurs, en gardant les autres erreurs fatales (test Node). Le Chromium logiciel exige ici la combinaison `--enable-unsafe-webgpu --use-angle=swiftshader --use-vulkan=swiftshader --enable-features=Vulkan` pour un rendu visible. Un adaptateur/device et des submits sans pixels visibles n'ont pas été considérés comme suffisants.

## Périmètre livré

`Cargo.toml`, Cargo.lock et entrées native/Web ; `src/navigation.rs`, `src/app.rs`, `src/draw.rs` ; tests Rust et séquence `tests/fixtures/brise-phare.txt` ; scripts de gate, packaging et vérification navigateur ; exemples de mesure et de comparaison native/WASM ; HTML/chargeur ; README, statut spec/roadmap, complément d'audit et captures. `.gitignore` ajoute seulement `dist/`. `up.sh` et ses tests existants sont inchangés.

Règles : carte et rectangles P0, trois ports, coque carrée ±6, 16 caps, positions/restes entiers, tick 20 ms, rotation immédiate puis tous les dix ticks, vent vers E force 2 et polaire P0. Collision rationnelle inclusive sur tout le segment, bornes strictes et rejet complet. Arrivée uniquement sur transition extérieur→disque, voiles abaissées, dernier port mémorisé en session. Pause explicite, reset exact, ordres ponctuels conservés/consommés une fois, sélection de manette stable et priorités prévues. HUD : cap/vitesse effective/cible/voiles, vent, port proche et direction/distance, arrivée ou terre, aide.

Le graphisme suit les couleurs et repères du SVG utilisateur. La géométrie reste celle de la spec : quatre îles rectangulaires et coque petite, sans la géométrie illustrative ou la grande coque du SVG. Le bandeau supérieur décoratif a été retiré après revue : il masquait le navire au bord nord. Le navire est dessiné après la girouette ; test de régression passé de rouge à vert.

## Contrôles automatisés

Commande canonique : **`bash scripts/check.sh`**. Exécution finale complète : **PASS**.

| Commande réellement exécutée | Résultat |
| --- | --- |
| `cargo fmt --all -- --check` | PASS |
| `cargo check --locked --all-targets` | PASS |
| `cargo test --locked --all-targets` | PASS — 17 tests Rust, dont la référence native |
| `cargo clippy --locked --all-targets -- -D warnings` | **PASS — Clippy strict** |
| `cargo build --locked --release` | PASS |
| `cargo check --locked --all-targets --target wasm32-unknown-unknown` | PASS — compilation, pas un essai navigateur |
| `cargo clippy --locked --all-targets --target wasm32-unknown-unknown -- -D warnings` | PASS |
| `python3 -B scripts/package_web.py` | PASS — build release locked, bindgen et HTML/JS/WASM/snippets |
| `bash -n up.sh scripts/check.sh` | PASS |
| `python3 -B -m unittest discover -s tests -v` | PASS — 14 tests temporaires locaux de up.sh |
| `node --test web/boot.test.mjs` | PASS — transfert winit et erreurs réelles |
| `git diff --check` | PASS |
| `bash up.sh` sur cet arbre modifié | PASS du refus attendu, code 1 avant fetch ; aucun contournement |
| up.sh end-to-end avec le vrai Cargo sur branche publiée propre | NOT RUN — publication non autorisée ; tests temporaires conservés |
| CI distante | NOT RUN — aucun push |

Tests ciblés : table de vent sur F0..3 et toutes directions, vitesses 72/57,6/10,8, déplacements et restes négatifs, calme/voiles basses, barre/répétition/wrap, balayage long/tangence/corner/bords, rejet et dégagement, caméra sous-unités et bornes, ports sur l'eau/entrée/départ/réentrée. Adaptateur : attente d'un tick, cinq ticks sans répétition, priorité baisser, opposés, pause/reset, sélection/déconnexion. Rendu : coque dans son carré, voilure visible, HUD et visibilité aux bords.

La fixture Brise→Phare compte **903 ticks, soit 18,06 s simulées**, sans téléportation, collision ou passage terrestre. Le test poursuit vers Anse puis revient à Brise. Reproductibilité testée sur 10 000 ticks et avec plusieurs regroupements de callbacks de même durée. La même séquence de 10 000 commandes avec reset intermédiaire donne exactement le même état complet natif/WASM :

```text
Simulation { position: [573114, 28058], remainder: [-69600, 32800], heading: 7, rudder: -1, turn_ticks: 3, sails: true, wind: Wind { heading: 0, force: 2 }, ticks: 5000, ports_inside: [false, false, false], last_port: 0, arrival: None, blocked: false, effective_speed: 1800 }
```

La remise à zéro à mi-séquence explique `ticks: 5000`. Cette égalité porte sur les commandes par tick, pas sur deux parties humaines jouées à des moments différents.

## Rendu et essais d'exécution

Environnement : Linux 6.18.33.2-microsoft-standard-WSL2, x86_64, WSLg/X11 ; Chromium **151.0.7922.34**, WebGPU logiciel SwiftShader. Aucun périphérique `/dev/input` n'est exposé. L'adaptateur natif exact n'a pas été identifié par ces tests.

| Essai | État |
| --- | --- |
| Fenêtre native release 1280×720, rendu réel | PASS ; [capture native](screenshots/p1-native-2x.png) |
| Browser : paquet HTTP, canvas WebGPU, clavier, pause/reprise/reset | PASS automatisé ; capture inspectée |
| Browser : Brise→Phare par touches, sans appel direct à la simulation | PASS automatisé ; arrivée et voiles basses visibles sur la capture finale inspectée |
| ×1, ×2 et lisibilité du HUD | PASS par inspection des captures ; ressenti humain restant |
| Redimensionnement non proportionnel | PASS automatisé à 970×580 ; ratio conservé, bandes et HUD lisibles ([capture](screenshots/p1-web-resize.png)) |
| Traversée native par injection X11 | FAIL de cet essai synthétique : focus WSLg instable (parfois 0), impulsions de barre non fiabilisées ; une tentative atteint la Dorsale et y est correctement bloquée. Aucune traversée native réussie revendiquée. |
| Traversée native au clavier physique | NOT RUN — validation humaine nécessaire |
| Manette réelle native et navigateur | NOT RUN — périphérique absent ; sélection/arbitrage couverts seulement par tests |
| Parcours humains, confort des 16 caps et retour des trois ports | NOT RUN — nécessitent un joueur |
| Focus/reprise avec interactions réelles du bureau | NOT RUN ; aucune auto-pause revendiquée |

Captures : [Web ×1](screenshots/p1-web-1x.png), [Web ×2](screenshots/p1-web-2x.png), [pause](screenshots/p1-web-pause.png), [arrivée](screenshots/p1-web-arrival.png). Une capture n'est pas un test matériel de manette.

Mesure CPU par **`cargo run --locked --release --example measure`** : 100 000 ticks, moyenne **0,034 µs/tick** ; 1 000 dessins à 640×360, moyenne **0,280 ms/dessin**. Ce microbenchmark ne mesure pas le GPU. La fenêtre native release a affiché **13,6 ms / 82 FPS** lors d'un relevé hors compilation ; sous compilation concurrente, les relevés étaient sensiblement plus lents. Ce ne sont pas des garanties de performance sur d'autres machines.

Le script `scripts/verify_browser.cjs` a terminé avec le code 0 : Chromium 151.0.7922.34, vendor `google`, architecture `swiftshader`, état natif/WASM identique, 570 submits et aucune erreur console/page. L'intervalle moyen de submit était 51,35 ms sur le scénario complet (captures, pause, redimensionnements et essai natif concurrent) ; ce chiffre n'est pas un benchmark de débit GPU isolé. Une première relance avait rencontré un serveur HTTP arrêté ; le serveur a été redémarré et l'essai complet a ensuite passé.

## Reproduire l'essai navigateur

Le script optionnel ne fait pas partie du gate sans navigateur. Il utilise une installation Playwright/Chromium existante ; aucune dépendance npm n'est ajoutée au jeu.

```bash
python3 -B scripts/package_web.py
python3 -m http.server 8765 --bind 127.0.0.1 --directory dist
# Autre terminal, en adaptant les deux chemins :
PLAYWRIGHT_MODULE=/chemin/node_modules/playwright \
CHROMIUM_EXECUTABLE=/chemin/chromium \
SOFTWARE_WEBGPU=1 node scripts/verify_browser.cjs
```

Sans `SOFTWARE_WEBGPU=1`, le script utilise les paramètres GPU par défaut. `HEADED=1` ouvre une fenêtre. Il compare l'état natif/WASM, vérifie des frames soumises et la pause, presse les touches pour la traversée et produit des captures dans `target/browser-check/`. **Inspecter l'arrivée sur la capture** : la présence de cette capture seule n'est pas une assertion automatique de l'arrivée. La commande requiert un serveur vivant pendant l'essai.

## Décision

**GO WITH CONDITIONS.** Prototype local limité à P1, contrôles techniques réussis. Restent l'essai humain de navigation, le clavier natif physique, une manette réelle sur les deux cibles et le contrôle de focus et de redimensionnement en usage humain. Aucun blocage d'API moteur ; aucun changement Core, autre dépôt, P2, push ou merge.
