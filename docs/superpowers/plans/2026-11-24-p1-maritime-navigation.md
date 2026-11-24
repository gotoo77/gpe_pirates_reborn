# P1 Maritime Navigation Implementation Plan

> **For agentic workers:** Use superpowers:executing-plans inline. The mission authorizes implementation of the existing P0 specification; no push or merge.

**Goal:** Quitter Port-Brise, contourner les terres avec le vent et atteindre les deux autres ports.

**Architecture:** Une règle entière pure dans `src/navigation.rs`, un adaptateur temporel et d'entrées dans `src/app.rs`, un rendu GPE dans `src/draw.rs`. Entrées native et WASM partagent ces modules. Aucun changement moteur.

**Tech Stack:** Rust 2024 >=1.97.1, GPE Git `8cbceda0c9b0cc55ee5deb7d5e173676ffc1d4f1`, wasm-bindgen =0.2.127.

**Spec:** `docs/p1-navigation-spec.md`, mission P1 du propriétaire, référence `/mnt/i/pirates_target.svg`.

## Global Constraints

- Branche `feat/p1-maritime-navigation`, base `b0ede15`, suivi distant vérifié ; travailler sur place sur cette branche demandée.
- Monde 1600x1000, quatre rectangles solides et trois disques de port exactement selon la spec.
- Framebuffer 640x360 ; vue monde 640x312, HUD 48 ; fenêtre 1280x720.
- 16 caps, 1024 sous-unités, ticks 20 ms, polaire et ordre de traitement exacts.
- GPE sans feature optionnelle, aucune dépendance path, Cargo.lock versionné.
- Ne pas modifier up.sh, GPE Core ou d'autres jeux. Aucun P2, push ou merge.
- Hard stop avant gameplay si une capacité requise manque ; compilation et essai navigateur sont distincts.

## Review Focus

- Ordres reçus avant un tick et ordres opposés : conservation et consommation unique, priorité baisser.
- Pause/reset à cheval sur un tick : aucun rattrapage ou ordre résiduel.
- Contact rasant et grande étape : aucun passage à travers une terre.
- Plusieurs manettes, déconnexion, deux directions clavier : priorité stable et neutralité.
- Caméra au bord, texte et navire orienté : aucun masquage par le HUD.

### Task 1: Capability preflight

**Files:** `Cargo.toml`, `Cargo.lock`, `src/lib.rs`, `src/main.rs`, `web/index.html`, `scripts/package_web.py`.
**Interfaces:** `start() -> Result<(), EngineError>` natif ; entrée `#[wasm_bindgen(start)]` Web. GPE Game reçoit Frame.

- [ ] Vérifier distante, SHA/package (ne pas confondre tag et version), toolchain, ALSA/udev et APIs du pin.
- [ ] Initialiser un écran minimal à la résolution requise, pin Git complet et lockfile ; sans gameplay.
- [ ] Construire et démarrer natif et paquet JS/WASM/HTML réel dans Chromium WebGPU ; consigner tout blocage avant code dépendant.

### Task 2: Navigation pure et tests

**Files:** `src/navigation.rs`, `tests/navigation.rs`, `tests/fixtures/brise-phare.txt`.
**Interfaces:** `Simulation::tick(Command)`, `blocked_segment(from, to) -> bool`, `camera(position) -> (i64,i64)`, constantes partagées ; état entier comparable.

- [ ] Écrire puis exécuter en échec les tests polaire F0..3, symétrie/caps limites, déplacements/restes négatifs, rotation immédiate/répétée.
- [ ] Implémenter table, voilure, rotation, vent et intégration entière minimale ; passer les tests.
- [ ] Tester puis implémenter balayage rationnel inclusif, bornes strictes, rejet complet et dégagement.
- [ ] Tester puis implémenter arrivées sur transition extérieure/intérieure, départ/réentrée, reset exact et caméra bornée.
- [ ] Enregistrer une séquence par tick Brise→Phare <9000 ticks et vérifier l'état libre à chaque étape ; tests vers Anse et retour.
- [ ] Comparer état complet après 10000 ticks et regroupements temporels ; partager une empreinte native/WASM.

### Task 3: Adaptation et présentation jouable

**Files:** `src/app.rs`, `src/draw.rs`, `tests/adapter.rs`.
**Interfaces:** `App::advance(Duration, Controls)` ; Game::update lit GPE Input, avance puis dessine ; `draw::scene(&mut Framebuffer, &App)`.

- [ ] Tests en échec pour ordres mémorisés/uniques, opposés, pause/reset, 5 ticks et sélection stable/déconnexion manette ; implémenter et passer.
- [ ] Dessiner mer texturée originale, côtes rectangulaires, ports nommés, coque/proue/voilure dans carré ±6, indicateur vent et HUD ASCII ; caméra sans lissage.
- [ ] Tester les invariants de rendu (HUD/caméra, limites coque), puis inspecter captures ×1/×2 et redimensionnement.
- [ ] Mesurer ticks/dessin ; comparer empreinte native/WASM ; tester clavier et parcours navigateur/natif si disponibles, manette réelle séparément.

### Task 4: Gate, documentation et revue

**Files:** `scripts/check.sh`, `README.md`, `docs/roadmap.md`, `docs/p1-validation.md`, statut de la spec et complément audit.

- [ ] Gate canonique : fmt, check/test all-targets locked, Clippy strict, release, packaging Web, Bash/Python et diff-check.
- [ ] Conserver tests temporaires up.sh ; vérifier refus légitime sur arbre sale sans contourner Git.
- [ ] Mettre à jour README/roadmap selon preuves, commandes exactes et résultats PASS/FAIL/NOT RUN ; captures et limites sans confondre build et essai visuel.
- [ ] Revue finale de conformité ; mémoriser dans ICM ; livraison locale P1 sans pousser/merger.

## Exécution et revue — 24 novembre 2026

- Préflight distant, pin, toolchain, natif et navigateur réel : PASS avant navigation. Branche demandée conservée sur place ; aucune intégration Git demandée.
- Règle pure et adaptateur : tests vus en échec, puis PASS ; fixture 903 ticks / 18,06 s, routes vers Anse et retour. Calcul initial de durée de la fixture corrigé selon l'échelle 1024, sans changer la règle.
- Rendu : test de régression nord/girouette RED→GREEN après revue indépendante. Aucun autre défaut gameplay certain trouvé ; aucun mineur différé.
- Gate canonique final PASS, 17 Rust / 14 Python / 1 Node, Clippy natif et WASM strict, release et packaging.
- Vérification navigateur finale PASS : état natif/WASM exact, pause/reprise et resize ; arrivée Phare inspectée sur capture.
- Relevés CPU : 0,034 µs/tick, 0,280 ms/dessin ; rendu natif release et captures réels. Traversée X11 synthétique non concluante (focus instable), essais physiques et ressenti NOT RUN.
- Choix de livraison : conserver cette branche et ses modifications locales, sans commit/push/merge ni P2, conformément à la mission. Les conditions restantes figurent dans p1-validation.md.
- Limites de revue : matériel manette, ressenti humain, performance matérielle et CI distante non attestés par le reviewer ; évalués séparément ou marqués NOT RUN dans le rapport. Aucun comportement hors P1 ajouté.
