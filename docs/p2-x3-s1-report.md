> **État courant — 2026-11-28 : S1-B implémentée au pin GPE `9ea92c068795f4e924b4cb9dd50cbb9ff13d3ef5`, gates techniques PASS ; qualification interactive native/Web incomplète, HOLD produit.** Voir la section « S1-B — reprise après GPE #117 / PR #118 » en fin de rapport. Les sections précédentes sont les preuves historiques A/B au pin ancien ; elles ne décrivent pas le candidat actuel. Aucun S2.

# P2-X3 S1 — capacity probe: HOLD

Base: `fcaace2e7200838bcb629ce042fe781d5774f9ec`, branch `exp/p2-x3-ux-foundation`.
Candidate: uncommitted working-tree diff. Source hashes: `target/s1/candidate-sha256.json`.
User authorized S1 through “Implement the plan” after P0. S2 is not started.
GPE pin remains `8cbceda0c9b0cc55ee5deb7d5e173676ffc1d4f1`.

## Implemented

One isolated scene shared by native executable and WASM cdylib examples; no gameplay source changes. Public GPE Ui/UiState button and Figtree OutlineFont, French glyph coverage and measured text, unchanged licensed PNG fitted using GPE image fitting. Local activation counter only. No replacement UI, generated art, new economic units or engine changes.

`ui-spike` is disabled by default and forwards GPE outline-fonts. Offline resolution added fontdue 0.9.4, hashbrown 0.15.5, foldhash 0.1.5 and core_maths 0.1.1. No dependency version already locked was upgraded. Notices and source attribution are in assets/ui-spike. These technical resources are not an approved maritime art kit.

## Evidence

| Capability | Result | Evidence / limit |
|---|---|---|
| PNG and Figtree load, accented text measurement | PROUVÉ | two probe tests pass; text 565 × 29 in 896 × 60 bounds |
| CPU rendering/capture | PROUVÉ | target/s1/s1-cpu.ppm, measure.log |
| Native window launch | PROUVÉ, limited | process ran until intentional 25 s timeout; audio underrun warning; human interaction not tested |
| Native window screenshot | BLOQUÉ | PIL X11 capture failed: X get_image error 8; CPU capture is not a substitute |
| Browser WebGPU | PROUVÉ | Chromium 1234 cached binary, headless SwiftShader, screenshots target/s1/browser |
| Mouse, Space, touch | PROUVÉ, simulated browser events | counter progresses 0 → 1 → 2 → 3; screenshots 01/03/04/06 |
| Release outside / touch cancel | PROUVÉ, simulated browser events | counter unchanged in screenshots 05/07 |
| Held Space | PROUVÉ, limited | 300 ms keyDown interval produced one increment; OS key-repeat not exercised |
| Pressed visual state | NON PROUVÉ | initial and pressed captures identical; response exists, visual distinction insufficient |
| Resize 640 × 360 | BLOQUÉ | screenshot 08 clips scene; CSS adjustment did not resolve fixed canvas behavior |
| Spatial focus / gamepad | NON TESTÉ | classic toolkit is ordinal Up/Down with Space activation; no public gamepad input adapter used |
| Hardware touch/pad, focus loss/resume | NON TESTÉ | no manual hardware session |
| Approved DA | NON TESTÉ | no maritime sample supplied or approved |

Browser screenshots were inspected, not treated as assertions of all behaviors. Script captures observations through CDP and requires an existing websocket-client installation. It does not install or launch a browser. No GPE-Agent/FGH job submitted.

## Measurements and limits

Same local CPU scene, 960 × 540, release build: resource load 19.701 ms; first draw 3.168 ms; 1,000 warm draws median 1.943 ms, p95 3.195 ms. Browser rAF intervals, 120 samples: median 16.7 ms, p95 50 ms. rAF is not GPU completion or input latency; SwiftShader is not representative hardware.

Observed binary sizes: native spike 16,427,704 bytes; WASM spike 3,143,696 bytes. Default game native 14,206,256 bytes; WASM 921,682 bytes. These are different programs: do not attribute the difference solely to outline-fonts. Feature-off same-scene comparison: NOT RUN. No performance threshold or regression conclusion established.

## Validation

- PASS: baseline `rtk cargo test --locked --test trade_flow --test adapter --test render` (19 tests).
- PASS: `rtk cargo test --locked --all-targets --features ui-spike`.
- PASS: `rtk cargo clippy --locked --all-targets --features ui-spike -- -D warnings`.
- PASS: same strict Clippy with `--target wasm32-unknown-unknown`.
- PASS: `rtk cargo build --locked --release --target wasm32-unknown-unknown --features ui-spike --example ui_capacity_spike_web`, then wasm-bindgen.
- PASS: `rtk bash scripts/check.sh` including rustfmt, native check/test/Clippy/release, WASM check/Clippy/package, Python and Node tests, diff check. Full output: target/s1/canonical.log.
- PASS: `rtk proxy python3 -m py_compile scripts/verify_ui_spike.py`.
- NOT RUN: original scripts/verify_browser.cjs; pre-existing expectedTrade reference remains untouched.

During development, scoped compile fixes closed Ui framebuffer borrow before drawing, restricted native capture CLI on WASM, and satisfied strict Clippy without allows. The oversized PNG was fitted with existing GPE API. Input mutators are crate-private; no engine edit to bypass that boundary. Resize remains a recorded blocker, not hidden by tests.

## Reproduction

```sh
rtk cargo run --locked --release --features ui-spike --example ui_capacity_spike -- target/s1
rtk cargo run --locked --release --features ui-spike --example ui_capacity_spike
rtk cargo build --locked --release --target wasm32-unknown-unknown --features ui-spike --example ui_capacity_spike_web
rtk wasm-bindgen --target web --out-dir target/s1/web --out-name ui_spike target/wasm32-unknown-unknown/release/examples/ui_capacity_spike_web.wasm
rtk cp web/ui-spike.html target/s1/web/
rtk proxy python3 -m http.server 18766 --bind 127.0.0.1 --directory target/s1/web
```

For automated observations, launch existing Chromium separately with CDP on 9223, an isolated profile, remote-allow-origins and appropriate local WebGPU flags, then `rtk proxy python3 scripts/verify_ui_spike.py`. The script uses localhost 18766. Native GUI and browser launch require environment permissions.

## Decision and next slice

A (classic GPE toolkit) proves basic interaction but does not yet prove the market composition, outline button labels, spatial focus, gamepad or resize behavior. B remains unimplemented: a bounded composition using existing public spatial/semantic GPE APIs, not a custom kernel. No S2 GO. Resolve essential API/viewport needs on evidence before production; a necessary engine change requires a separate ticket and HOLD.

One session, no delegated agent, no commit/push/merge/deploy, caches preserved. Credits: **non mesurée**. S2 still requires approved art sample and units decision; default remains lots. P2.0 and economic laboratory remain frozen.

Next action: review this HOLD and decide whether to authorize a bounded B capacity slice targeting spatial focus, outline controls and viewport behavior.


## S1-B — arrêt sur frontière API publique : HOLD

Reprise au même SHA de base `fcaace2e7200838bcb629ce042fe781d5774f9ec`, même branche, candidat S1-A non commité. Autorisation : mission S1-B bornée. **Aucune scène B construite : condition d'arrêt atteinte pendant l'inspection ciblée**, avant de contourner une capacité moteur manquante. S2 non lancé.

### Préservation et approche retenue

Avant modification, tous les hashes de `target/s1/candidate-sha256.json` ont été recalculés et sont identiques. Copie du rapport A avant ajout : `target/s1-b/s1-a-report.md` ; inventaire : `target/s1-b/s1-a-preservation.json`. Les sources, assets, scripts et preuves A restent inchangés. Seul ce rapport est modifié par S1-B ; les sondes discriminantes et logs sont dans `target/s1-b`, non suivis.

Approche retenue pour évaluer B : **experimental_spatial**, son `CardPainter` et son passage headless. `experimental` n'est examiné qu'à la frontière nécessaire : génération d'identités stables par clés, et vérification qu'il ne fournit pas l'alternative manquante. Pas de seconde implémentation, aucun nouvel adaptateur ou noyau.

### Capacité essentielle manquante et reproduction

Au pin exact, dans les sources GPE :

- `ui/experimental_spatial.rs:46` : `SpatialCard` expose identité, textes, image et action ; ni rectangle imposé par le jeu ni disponibilité.
- `run_card_grid` et `run_card_grid_headless` prennent des bornes, `GridSpec` et une liste de cartes. `resolve_card_grid` appelle `layout_cards` avant l'interaction. Les rectangles générés déterminent à la fois navigation et hit-test.
- `CardPainter::paint` reçoit les rectangles et états focus/survol/pression : dessin personnalisé et police outline possibles. Modifier uniquement le dessin ne déplace pas la hitbox. `SpatialOutput::layouts()` donne les géométries après interaction, sans entrée publique permettant de les remplacer.
- `ui/experimental.rs:884` : l'autre adaptateur utilise `UiNavigationPolicy::Linear`, malgré sa composition en panneaux/grilles. Il n'offre pas le passage spatial libre recherché.
- `ui/kernel.rs:36,70,128` : cibles résolues, politique spatiale et passage commun sont `pub(crate)` ; le module lui-même est privé.

Reproduction minimale, **test négatif de compilation**, pas échec du candidat :

```rust
use gotoo_pixel_engine::ui::kernel::{run_interaction_pass, UiResolvedTarget};
fn main() {}
```

Compilation réelle contre le rlib verrouillé : **E0603**, module privé et trait non réexporté. Source et diagnostic conservés dans `target/s1-b/private-api.rs` et `private-api.log`. Pour reproduire sans rebuild Cargo : `rtk rustc --edition=2024 target/s1-b/private-api.rs --extern gotoo_pixel_engine=target/debug/deps/libgotoo_pixel_engine-54746cce9cbe2b98.rlib -L dependency=target/debug/deps --out-dir /tmp` ; adapter uniquement le nom hashé du rlib si le cache change.

Besoin concret non couvert : soumettre dans **un même passage spatial** des cibles identifiées aux rectangles choisis par le jeu, avec disponibilité, pour garder dessin et interaction concordants. Exemple : commandes dans l'étal à gauche et confirmation dans un bon d'échange à droite, dimensions distinctes, sans les imposer dans des cellules identiques. Une grille peinte autrement ne prouve pas cette capacité. Plusieurs passages sur des sous-groupes ne constituent pas une preuve : le noyau répare le focus sur la liste de cibles courante. Aucun tel contournement tenté.

L'absence de champ disponibilité ne prouve pas à elle seule une impossibilité métier : un jeu peut omettre une action indisponible et en dessiner l'état. Elle ne fournit toutefois pas ici un contrat de cibles libres avec géométrie stable et disponibilité. Aucun faux contrôle désactivé ajouté pour prétendre avoir testé ce cas.

**Arrêt demandé appliqué** : exposer cette capacité nécessiterait une évolution publique du moteur. Zéro correction de ce blocage, zéro modification du moteur, pas de copie de son noyau. Cette preuve ne dit pas que GPE est incapable de dessiner une interface riche ; elle borne les adaptateurs publics inspectés au besoin de composition libre avec interaction spatiale commune.

### Clipping : diagnostic borné par couche

- **HTML/canvas** : S1-A demande une fenêtre 960 × 540. GPE `platform.rs:710–725` transmet cette taille à Winit et utilise `with_append(true)` en Web. Winit verrouillé 0.30.13, `platform_impl/web/web_sys/mod.rs:83–84`, écrit `width` et `height` CSS en pixels. Ces styles inline priment sur la règle simple `width:100vw;height:100vh` du lanceur A. Cela explique plausiblement le canvas conservé à 960 × 540 et découpé par un viewport navigateur 640 × 360. Les captures A restent la preuve visuelle ; pas de nouvelle mesure DOM, donc causalité dynamique non confirmée et aucun correctif HTML livré.
- **Viewport GPE** : la fonction publique `Viewport::new` puis `map_surface_position` est testée directement, sans jeu ni navigateur. PASS aux surfaces 960 × 540, 640 × 360 et 800 × 600 : centre projeté vers (480,270), extérieur rejeté ; letterboxing 800 × 450 à y=75 pour 800 × 600. Source et sortie dans `target/s1-b/viewport.rs` et `viewport.log`. Cela prouve le calcul géométrique testé, pas les événements DOM.
- **Composition** : A place ses éléments dans le framebuffer fixe 960 × 540. Sa capture CPU prouve cette composition au format nominal. Aucune recomposition B ou preuve de layout libre au format réduit.
- **Mapping des entrées** : GPE `platform.rs:695` dérive son mapping de `window.inner_size()` et du framebuffer configuré. Si la surface est réellement redimensionnée, la fonction testée est cohérente ; une simple transformation CSS ne prouve pas que Winit reçoit les bonnes coordonnées/taille. **Correspondance visuel/hitbox après resize : NON TESTÉE**, pas de GO déduit du test géométrique.

Aucun dépannage X11, aucune correction aveugle du canvas. L'arrêt API rend inutile de poursuivre les essais interactifs B dans ce lot.

### Preuves et validations S1-B

| Élément | Statut et nature |
|---|---|
| Préservation candidat et preuves A | PASS, comparaison automatique des hashes des sources répertoriées ; preuves A non écrasées |
| Composition libre + passage spatial public | BLOQUÉ, signatures/sources inspectées + test négatif E0603 |
| Peintre personnalisé, identité stable, états focus/survol/pression | API présente dans les sources, NON PROUVÉE par une scène B |
| Navigation spatiale, activation unique B, indisponibilité | NON TESTÉES en B après arrêt ; résultats A non réattribués à B |
| Annulation et reprise de focus B | NON TESTÉES après arrêt |
| Viewport et mapping géométrique | PASS, assertions automatisées directes sur API publique, trois tailles |
| Navigateur B / événements simulés | NOT RUN après arrêt ; aucune nouvelle session navigateur |
| Essais matériels B | NOT RUN, aucun clavier/pad/tactile matériel qualifié |

Validations relancées **par exigence explicite de livraison S1-B**, sur le candidat A préservé, sans les présenter comme preuve fonctionnelle B :

- **PASS** `rtk bash scripts/check.sh` : rustfmt, check/tests/Clippy strict natif, release, check/Clippy strict WASM, package Web, tests Python/Node, diff check ; `target/s1-b/canonical.log`.
- **PASS** `rtk cargo test --locked --all-targets --features ui-spike` ; `target/s1-b/feature-tests.log`.
- **PASS** `rtk cargo clippy --locked --all-targets --features ui-spike -- -D warnings` ; `target/s1-b/clippy-native.log`.
- **PASS** même Clippy avec `--target wasm32-unknown-unknown` ; `target/s1-b/clippy-wasm.log`.
- **PASS attendu du test négatif** : compilation refusée par E0603 ; aucun test/gate affaibli.

Aucun changement Rust/Cargo S1-B, de gameplay, économie, pin, GPE-Agent ou du script historique expectedTrade. Aucun commit/merge ni S2. Caches réutilisés, pas de benchmark des polices, pas d'images générées. Consommation de crédits : **non mesurée**.

**Verdict : HOLD sur les capacités UI nécessaires à S2 dans le contrat de composition libre demandé.** Les capacités de grille ne valent pas preuve de cibles libres ; les essais interactifs B restent à faire après résolution de la frontière publique.

**Prochaine action unique : cadrer un ticket moteur séparé pour le passage spatial public de cibles composées par le jeu, avec disponibilité, sans l'implémenter dans Pirates.**


## S1-B — reprise après GPE #117 / PR #118

### Candidat et périmètre

Mission explicite du 2026-11-28. Base et HEAD inchangés : `fcaace2e7200838bcb629ce042fe781d5774f9ec`, branche `exp/p2-x3-ux-foundation`. **Candidat non commité**, incluant les travaux locaux S1-A préexistants. Aucun commit, push, merge ou déploiement ; aucun GPE-Agent/FGH ni changement moteur. GPE #117 et sa qualification humaine native/Web sont acquis selon le propriétaire ; ils ne qualifient pas automatiquement ce consommateur Pirates.

Pin autorisé aligné dans `Cargo.toml` et `Cargo.lock` sur `9ea92c068795f4e924b4cb9dd50cbb9ff13d3ef5`. Comparé au lockfile **présent au début de cette mission**, seule la source Git GPE change. Les dépendances outline déjà ajoutées par A sont conservées ; aucun autre package mis à jour. Lecture des sources exactes `docs/ui/RESOLVED_TARGETS.md`, `examples/gpe_ui_resolved.rs` et de la surface publique `ui::resolved` du checkout Cargo à ce SHA.

Copies et empreintes avant reprise : `target/s1-b-final/before/`, `before-sha256.json`. Sources, assets, harnais HTML et script A **byte-identiques** après reprise ; le rapport reçoit uniquement un encart courant et cette annexe. Les preuves A et celles du premier arrêt B ne sont pas écrasées. Inventaire final : `target/s1-b-final/candidate-sha256.json`. Le plan borné et la revue locale sont dans le même répertoire ignoré par Git. Ces preuves locales ne sont pas des artefacts publiés.

Ajouts : deux entrées `examples/ui_capacity_spike_b{,_web}.rs`, scène partagée et tests dans `examples/support/ui_spike_b{,_tests}.rs`, harnais `web/ui-spike-b.html`, observateur CDP `scripts/verify_ui_spike_b.py`. A demeure disponible dans ses propres entrées. La feature `ui-spike` reste désactivée par défaut. Aucun fichier `src/`, aucune règle économique, unité, navigation de jeu, marché de production, laboratoire ou S2 modifié.

### Intégration et invariants

Cinq cibles stables disjointes, de tailles et positions différentes, dans le framebuffer 960 × 540 : centre `(330,220,200,70)`, haut `(360,130,150,50)`, droite `(660,210,250,80)`, bas `(300,370,260,70)`, gauche `(50,230,160,60)`. Bas initialement indisponible. Ce sont des contrôles de capacité explicitement nommés, sans transaction. Même rectangle pour dessin et soumission au **seul passage public `resolved::update`**. Aucun calcul local de hit-test, navigation, capture ou réparation de focus.

Ordre de chaque frame : `ResolvedState::synchronize_input(input)` même hors focus ; effacement du flash et suppression des transitions locales si false ; mise à jour de `ControlMap` ; transitions R/D seulement si autorisées ; puis `resolved::update` avec le snapshot sémantique même pendant le blocage pour laisser progresser sa barrière neutre. Un seul site consomme `output.activation` et incrémente le compteur. Annulation et suspension effacent le flash ; le noyau conserve le focus logique valide et annule les captures.

Flèches / D-pad : navigation ; Entrée/Espace / South : confirmation ; Échap / East : annulation ; R / Start : suspension explicite ; D / West : disponibilité du bas. Focus cyan, survol bleu, capture verte, indisponibilité grise et libellée, flash de validation orange 180 ms. Le clavier/pad ne fabrique pas de faux état de capture maintenue. Figtree outline et PNG licencié de A réutilisés ; aucune DA maritime approuvée n'est déduite de cette scène.

Le contrat moteur limite la preuve physique à ce qui reste observable après le reset OS : aucune affirmation de relâchement matériel des touches/souris/touches tactiles effacées. Le pad conservé doit redevenir neutre, axes à zéro ; une dérive peut retarder la reprise. Aucune réimplémentation de cette barrière côté jeu.

### Preuves et limites

| Capacité | Résultat actuel | Preuve / portée |
|---|---|---|
| API publique et composition libre | PASS technique | compilation native/WASM ; géométrie exacte partagée avec le peintre |
| Quatre allers-retours spatiaux | PASS sémantique | géométrie B exacte, bas activé pour le test, retour centre, compteur inchangé |
| Indisponibilité et conflit navigation/confirmation | PASS sémantique | tests du consommateur, aucun incrément interdit |
| Capture bords demi-ouverts, maintien, sortie | PASS sémantique | bords exacts dessinés, une activation, release répétée sans incrément |
| Annulation, suspension, barrière neutre | PASS sémantique | capture abandonnée, confirm retenu rejeté, fresh press après neutralité |
| Touch cancel/sortie, fins simultanées ambiguës | PASS sémantique | aucun dispatch ; ce ne sont pas des essais tactiles matériels |
| Synchronisation du focus OS | PASS test de snapshot public | `Input::set_window_focused`, perte/gain entre frames et frames séparées, flash/capture effacés ; pas un Alt+Tab humain |
| États visuels | PASS rendu CPU | assertions sur pixels réellement peints : repos, focus, hover, capture, indisponible, flash ; distinction pression A non réattribuée |
| Natif : fenêtre et capture | PASS limité | fenêtre réelle 960 × 540, capture directe X11 `native-window.png`, CPU `s1-b-cpu.ppm` ; pas de qualification humaine |
| Natif : scénario XTest | FAIL du scénario, qualification incomplète | premier ID de fenêtre expiré (`BadWindow`), relance puis assertion d'état initial/navigation échouée ; arrêt, aucune conclusion de bug moteur |
| Web : build et wasm-bindgen | PASS | deux entrées A/B release, binding Web généré |
| Resize réel HTML/surface B | PASS DOM | avant CSS : viewport640×360 mais DOM/surface960×540 ; après CSS : DOM/surface640×360 ; JSON `browser-before/observations.json` et `browser-dom-after/observations.json` |
| Web : rendu/interaction | BLOQUÉ environnement | headless blanc puis noir, assertion initiale de pixels échouée ; aucune suite interactive exécutée jusqu'au bout |
| Web visible | BLOQUÉ environnement | première page `ERR_CONNECTION_REFUSED` (serveur fermé), retest serveur relancé puis CDP9224 inaccessible ; arrêt de cette voie |
| Hitboxes après resize natif/Web | NON QUALIFIÉ | calcul moteur/API et dimensions DOM ne suffisent pas à prouver le mapping réel des clics |
| Pad/tactile matériels, Alt+Tab humain, agrément/DA | NOT RUN | retest propriétaire requis, aucune assimilation avec la qualification moteur |

Correction HTML/CSS **une seule, sur le nouveau harnais B uniquement** : `width:100vw!important;height:100vh!important` prime sur la taille CSS inline Winit. Le ResizeObserver met alors effectivement à jour la surface. Pas de transformation CSS ni nouveau mapping d'entrée. Le harnais A reste identique. La correspondance visuel/hitbox après resize n'a pas pu être validée dans cette session.

Bornes respectées : une correction CSS étayée ; headless initial puis un ajustement Vulkan/SwiftShader, arrêt sur images inutilisables ; un essai visible puis un retest après fermeture du serveur, arrêt sur CDP indisponible ; aucune investigation GPU/WSL/X11 ni réparation d'infrastructure. Capture bureau X11 toujours en échec `X get_image error 8`, capture directe fenêtre réussie. La tentative XTest n'est pas présentée comme un playtest réussi. Aucun fanout.

### Gates réellement exécutés sur le candidat

| Commande | Résultat | Preuve locale |
|---|---|---|
| `rtk cargo fmt --all -- --check` | **PASS** | `target/s1-b-final/fmt.log` |
| `rtk cargo test --locked --all-targets --features ui-spike` | **PASS** : 59 tests Rust / 15 suites, dont 8 tests B par entrée et 2 tests A par entrée | `feature-tests.log` |
| `rtk cargo clippy --locked --all-targets --features ui-spike -- -D warnings` | **PASS** | `clippy-native.log` |
| `rtk cargo clippy --locked --all-targets --target wasm32-unknown-unknown --features ui-spike -- -D warnings` | **PASS** | `clippy-wasm.log` |
| `rtk bash scripts/check.sh` | **PASS** : fmt, check/test/Clippy strict/release natif, check/Clippy strict/package WASM, 39 tests Rust, 30 Python, 1 Node, diff check | `canonical.log` |
| Builds release A/B natif et Web + wasm-bindgen 0.2.127 | **PASS** | `build-native.log`, `build-web.log`, `web/` |
| `rtk proxy python3 -m py_compile scripts/verify_ui_spike_b.py` et `--help` | **PASS** | exécution locale ; dépendances optionnelles déjà présentes : websocket-client, Pillow |
| `rtk git diff --check` | **PASS** | contrôle relancé après rapport |
| Scénarios d'interaction navigateur et XTest | **FAIL / BLOQUÉ**, détails ci-dessus | captures/observations et `runtime-results.txt` |
| `scripts/verify_browser.cjs` | **NOT RUN** | défaut historique `expectedTrade` hors scope, script inchangé |
| CI distante / GPE-Agent / FGH | **NOT RUN** | aucune soumission |

Le gate canonique désactive la feature par défaut ; les deux Clippy et les tests avec `ui-spike` ci-dessus sont donc nécessaires et ont été exécutés séparément. Aucun avertissement masqué, aucun test ou gate détendu. Revue locale par l'auteur, sans agent de revue indépendant (mission sans fanout) : invariants/API/périmètre vérifiés, qualification runtime toujours bloquante. Crédits : **non mesurée** ; aucune nouvelle mesure de performance représentative ni benchmark attribué au changement de pin.

### Retest natif et Web

Depuis la racine, avec Rust/WASM et wasm-bindgen 0.2.127 installés :

```sh
# A préservée
rtk cargo run --locked --release --features ui-spike --example ui_capacity_spike
# B native ; ajouter target/s1-b-final pour la capture CPU sans fenêtre
rtk cargo run --locked --release --features ui-spike --example ui_capacity_spike_b
rtk cargo run --locked --release --features ui-spike --example ui_capacity_spike_b -- target/s1-b-final

# B Web ; chemin distinct des preuves A historiques
rtk cargo build --locked --release --target wasm32-unknown-unknown --features ui-spike --example ui_capacity_spike_b_web
rtk wasm-bindgen --target web --out-dir target/s1-b-final/web --out-name ui_spike_b target/wasm32-unknown-unknown/release/examples/ui_capacity_spike_b_web.wasm
rtk cp web/ui-spike-b.html target/s1-b-final/web/ui-spike-b.html
rtk proxy python3 -m http.server 18767 --bind 127.0.0.1 --directory target/s1-b-final/web
```

Garder le serveur vivant dans son terminal ; ouvrir `http://127.0.0.1:18767/ui-spike-b.html` dans un navigateur WebGPU réel. Pour l'observateur optionnel, lancer séparément un Chromium déjà installé avec `--remote-debugging-port=9224 --remote-allow-origins=http://localhost:9224` et un profil isolé, puis `rtk proxy python3 scripts/verify_ui_spike_b.py --out target/s1-b-final/retest-browser`. `--dom-only` ne fait que relever le DOM à 960×540 et 640×360 ; sa réussite ne valide pas les interactions. Ne pas reprendre d'investigation headless sur ces échecs historiques.

Parcours humain court à faire **dans les deux runtimes**, pas encore coché :

1. Repartir de B neuf : compteur0, centre focalisé, bas gris indisponible. Souris puis tactile : hover/capture distincts, click/end = +1 ; sortie/cancel = +0.
2. Depuis centre, droite/gauche, haut/bas, gauche/droite ; D active bas, bas/haut. Chaque retour revient au centre, aucun mouvement n'active. D-pad/South/West/East/Start sur matériel disponible, sinon noter NOT RUN.
3. Entrée et Espace : +1 par pression ; maintenir et répéter ne doit pas doubler. Flash orange court. Bas indisponible ne peut pas s'activer.
4. Capture souris/tactile puis Échap ou R : +0, capture abandonnée, neutralité puis nouvelle pression nécessaire. Capturer puis Alt+Tab, relâcher hors fenêtre et revenir : +0, aucun flash/capture hérité, focus logique conservé ; clic neuf et Entrée neuf fonctionnent. Vérifier aussi perte/gain rapide et maintien d'une entrée au retour sans supposer un relâchement matériel invisible au moteur.
5. Redimensionner réellement la fenêtre/surface à640×360 puis800×600 : tous les contrôles/textes restent visibles ; vérifier le centre ET une commande éloignée, les bords et les zones hors cible. En800×600, tester aussi les bandes hors contenu : +0. Aucune validation du mapping déduite des seules dimensions DOM.

**Verdict : implémentation S1-B livrée pour revue, HOLD de qualification produit.** Le blocage d'API publique est levé ; les preuves sémantiques et de rendu CPU sont acquises. Le test interactif humain natif/Web de ce spike, particulièrement focus OS et hitboxes après resize, reste l'action suivante unique. Aucun GO S2 ni GO artistique déduit des gates techniques.

## Qualification humaine finale S1-B

Pin GPE : 9ea92c068795f4e924b4cb9dd50cbb9ff13d3ef5.

| Test | Natif | Web |
|---|---|---|
| Affichage complet et lisible | PASS | PASS |
| Navigation et allers-retours | PASS | PASS |
| Activation unique | PASS | PASS |
| Relâchement hors cible | PASS | PASS |
| Perte de focus et reprise | NOT RUN | PASS |
| Resize 640x360 et hitboxes | PASS | PASS |
| Resize 800x600 et hitboxes | NOT RUN | NOT RUN |

Bilan : 11 PASS, 0 FAIL, 3 NOT RUN.
Manette et tactile physiques non testés.

Décision : capacité S1-B acceptée avec réserves.
Aucun GO artistique ni GO S2 implicite.
