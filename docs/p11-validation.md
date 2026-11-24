# P1.1 — Maritime Visual & Gameplay Feel

24 novembre 2026 — **GO WITH CONDITIONS**. Livraison locale après P1 `84e869e`
sur `feat/p1-maritime-navigation`. Aucun commit, push ou merge du dépôt de travail,
aucun P2, aucun changement GPE Core. Les essais Windows natif, PowerShell et
manette physique restent nécessaires avant acceptation matérielle complète.

## Périmètre et préflight

État Git initial propre, P1 présent dans l'historique et dans l'upstream.
Instructions, P0/P1, tests, gate et API du pin relus. `Cargo.toml` et `Cargo.lock`
conservent exactement **`8cbceda0c9b0cc55ee5deb7d5e173676ffc1d4f1`** ; aucune
dépendance ajoutée. `up.sh` et `tests/test_up.py` sont inchangés.

Le gate P1 de référence passe avant modification. Un exemplaire temporaire de
P1 à 960×540 a ensuite réellement démarré en natif et WebGPU ; captures inspectées
avant développement du rendu. Les API `Key::M`, `GamepadButton::West`, texte
agrandi, images et pixels sont présentes dans les sources du pin. Aucune capacité
indispensable GPE manquante : le hard stop n'a pas été déclenché. La présence de
l'API manette ne vaut pas essai matériel.

Rendu original en code, inspiré des quatre références `I:\pirates_1.png` à
`pirates_4.png`, sans les copier ni les embarquer : coque de 26 pixels, deux mâts,
voiles hissées/baissées et pavillon rouge sur les 16 caps ; mer texturée et vagues
animées ; sillage limité aux déplacements acceptés, qui s'élargit et s'efface ;
plages, baies, végétation, rochers, trois petits villages et quais ; HUD hiérarchisé.

Les côtes sont des unions de huit polygones convexes fixes dans les quatre
enveloppes P1. Le même contour alimente l'image et le balayage rationnel de la
coque carrée ±6. Les plages sont solides ; écume et hauts-fonds sont de l'eau.
Les quais/bouées restent des repères sans nouvelle collision. Aucun générateur de
géographie ni moteur de terrain. Polaire, positions/restes entiers, rotation,
vitesses, tick 20 ms, rejet complet des déplacements bloqués et arrivées sont
conservés. La géométrie affinée est le seul changement des surfaces terrestres.

M / West affiche le monde entier, les îles, les trois ports et le navire orienté.
La carte suspend les ticks et les animations. M / West / Échap / Start ferme la
carte ; une pause préalable est conservée. Sinon les ticks reprennent sans
rattrapage, avec cap/voilure antérieurs ; les commandes nautiques sont ignorées
jusqu'à neutralité physique des touches, D-pad et stick. Les commandes ponctuelles
et le temps accumulé avant ouverture sont supprimés. Le HUD signale l'attente.

## Résolution et performances

**Retenu : 960×540**, 16:9, fenêtre initiale **1440×810**, HUD de 88 lignes.
La caméra garde 18 pixels de marge aux limites du monde pour montrer tout le
sprite, indépendamment de la collision physique. À ×1 les valeurs principales
du HUD font 14 pixels de haut ; les détails du navire et du terrain remplacent
les anciennes formes rudimentaires. Le monde conserve ses 1600×1000 unités.

1280×720 a été comparé avec le même dessin à sa résolution réelle : il montre
plus de mer mais ne donne pas de détails supplémentaires au voilier de 26 pixels.
À fenêtre égale les repères sont plus petits ; son coût CPU est supérieur.
640×360 n'est pas retenu : c'est la référence avant, insuffisante pour le niveau
de détail demandé. Aucun changement de simulation pour compenser la résolution.

Benchmark existant étendu à deux tailles, **`cargo run --locked --release --example measure`** :

| Mesure réellement exécutée | CPU moyen |
| --- | ---: |
| P1 avant : dessin 640×360, 1 000 images | 0,159 ms |
| P1.1 : dessin 960×540, 1 000 images, cache chaud | **0,450 ms** |
| P1.1 : dessin 1280×720, 1 000 images, cache chaud | 0,846 ms |
| P1 avant : 100 000 ticks | 0,022 µs/tick |
| P1.1 : 100 000 ticks | 0,025 µs/tick |
| Premier dessin 960×540, création du cache des îles incluse | 11,309 ms |

**Régression de coût CPU du dessin : +0,291 ms, soit environ +183 % face à P1**,
avec 2,25 fois plus de pixels et un décor plus détaillé. 1280×720 coûte environ
88 % de plus que 960×540 dans le relevé final. Une première mesure P1.1 donnait
0,485 / 0,818 ms ; les valeurs varient avec la charge. Ce benchmark mesure le CPU,
pas la présentation GPU ; moins de 1 ms CPU ne garantit pas une fréquence d'écran.

Sur WSLg, les titres de fenêtre après redimensionnement ont affiché 5,7 ms /
149 FPS à ×1 et 27,3 ms / 36 FPS à ×2. Relevés ponctuels pendant un scénario de
capture, sans identification de l'adaptateur natif : **la présentation ×2 peut
coûter sensiblement plus cher**, et le débit Windows natif reste à mesurer.
Le scénario WebGPU logiciel final relève 756 submits, intervalle moyen 46,16 ms
(référence P1 : 39,90 ms). Scénario enrichi avec carte et captures supplémentaires,
donc ce n'est pas une comparaison GPU isolée ni une preuve de régression GPU.

## Captures et essais visuels

| Vue | Preuve |
| --- | --- |
| Avant P1 | [640×360](screenshots/p1-web-1x.png), [native ×2](screenshots/p1-native-2x.png) |
| Après P1.1, framebuffer CPU | [départ 960](screenshots/p11-start-960.png), [navigation 960](screenshots/p11-sailing-960.png), [alternative 1280](screenshots/p11-sailing-1280.png) |
| Sprite original | [16 caps, voiles hissées et baissées](screenshots/p11-ship-16-headings.png) |
| Vue générale / limites | [carte CPU](screenshots/p11-chart-960.png), [bord du monde](screenshots/p11-edge-960.png) |
| Natif réel, Linux/WSLg | [fenêtre initiale](screenshots/p11-native-initial.png), [×1](screenshots/p11-native-1x.png), [×2](screenshots/p11-native-2x.png) |
| Navigateur réel | [×1](screenshots/p11-web-1x.png), [×2](screenshots/p11-web-2x.png), [redimensionnement](screenshots/p11-web-resize.png) |
| Interactions Web | [carte](screenshots/p11-web-chart.png), [pause](screenshots/p11-web-pause.png), [arrivée Phare](screenshots/p11-web-arrival.png) |

Captures inspectées : coque, voilure, côtes, trois ports et informations essentielles
lisibles à ×1 et ×2. Le HUD natif à ×2 est **pixel pour pixel égal** à son image
×1 agrandie en nearest (comparaison des 88 lignes du HUD, sans animation).
Le redimensionnement Web 970×580 conserve le ratio, avec bandes. L'arrivée
`ARRIVEE PORT-PHARE`, voiles baissées, est visible sur la capture finale de la
traversée Web commandée par touches. Ce sont des essais automatisés et une
inspection visuelle, pas une appréciation humaine du plaisir de navigation.

## Validations exécutées

Commande canonique : **`bash scripts/check.sh` — PASS**, après corrections de revue.
Rust/Cargo 1.99.0 ; Linux WSL2 ; Chromium 151.0.7922.34, WebGPU SwiftShader.
Pas de combinaison `--all-features` ajoutée : le jeu n'a aucune feature optionnelle.

| Commande / contrôle | État |
| --- | --- |
| `cargo fmt --all -- --check` | PASS |
| `cargo check --locked --all-targets` | PASS |
| `cargo test --locked --all-targets` | **PASS — 27 tests Rust** |
| `cargo clippy --locked --all-targets -- -D warnings` | **PASS — Clippy strict natif** |
| `cargo build --locked --release` | PASS |
| `cargo check --locked --all-targets --target wasm32-unknown-unknown` | PASS |
| `cargo clippy --locked --all-targets --target wasm32-unknown-unknown -- -D warnings` | **PASS — Clippy strict WASM** |
| `python3 -B scripts/package_web.py` | PASS — release locked WASM, bindgen, JS/HTML/snippets |
| `bash -n up.sh scripts/check.sh` | PASS |
| `python3 -B -m unittest discover -s tests -v` | PASS pour 14 tests Bash ; **16 tests PowerShell NOT RUN / skipped** |
| `node --test web/boot.test.mjs` | PASS — test du chargeur conservé |
| `git diff --check` | PASS |
| `cargo run --locked --release --example measure` | PASS — mesures ci-dessus |
| `cargo run --locked --release --example capture` | PASS — framebuffers PPM reproductibles |
| `scripts/verify_browser.cjs` avec Playwright/Chromium existants | PASS — carte, reprise, pause/reset, traversée, redimensionnements, zéro erreur page/console |
| Comparaison état complet natif/WASM après 10 000 commandes avec reset | PASS — état identique à la référence P1 |
| Fenêtre native et redimensionnements ×1 / ×2 | PASS — rendu réel capturé |
| Windows 11 natif : build, clavier, focus et performances | **NOT RUN** dans cette session |
| PowerShell : syntaxe et 16 protections Git en dépôts temporaires | **NOT RUN** — aucun runtime accessible |
| Manette physique native / navigateur | **NOT RUN** — aucun périphérique exposé |
| Parcours humains entre les ports et ressenti | **NOT RUN** |
| CI distante | NOT RUN — aucun push |

Les tests P1 de polaire, déplacement, rotation, balayage, ports et déterminisme
sont conservés. Seules les assertions dépendant de la résolution/caméra et de
l'ancienne taille graphique ont été adaptées. La fixture Brise→Phare reste
**903 ticks / 18,06 s**, puis les trajets vers Anse et retour Brise passent.
Les ajouts couvrent convexité/enveloppes/côtes, absence de tunneling, pixels du
décor et bâtiments sur terre, taille/16 silhouettes, visibilité complète dans les
deux vues aux bords, animations sans mutation de simulation, sillage, carte/pause,
suppression d'ordres en attente, retour sans pause cachée et neutralité brute.

Une revue indépendante a relevé la suspension prolongée par une barre maintenue,
le réarmement par des touches opposées, les contrôles Git après pull et une aide
de carte pouvant masquer le navire. Corrections effectuées ; tests Rust concernés
observés rouges puis verts. Aucun problème important connu laissé ouvert.

## Windows et réserves

`up.ps1` utilise le répertoire du script, refuse l'arbre sale y compris les non
suivis/sous-modules, detached HEAD, absence de commit/upstream, suivi local et
opération Git en cours. Il lance `git pull --ff-only --no-rebase` avec autostash
désactivé, revérifie branche/propreté, vérifie le manifeste, compile locked release,
contrôle chaque code externe et restaure le répertoire appelant via `finally`.
Sans manifeste il indique la compilation indisponible avec 0, comme Bash.

16 essais temporaires sont préparés : succès/cache/branche, arbre sale, detached,
upstream absent ou invalide, divergence, erreurs Git/Cargo, manifeste, opérations
en cours dont sequencer, contrôle après hook de pull, localisation et restauration
du répertoire appelant. **Aucun de ces essais PowerShell n'a tourné ici.**
`pwsh`/`powershell` Linux sont absents ; le fichier Windows `pwsh.exe` trouvé sous
`/mnt/c/Program Files/PowerShell/7/` renvoie `Exec format error`. Les essais ne sont
ni émulés ni déclarés PASS. Les 14 protections Bash ont réellement été exécutées.

Écarts et conditions : rendu volontairement simple et original plutôt que la
richesse illustrative des références ; comparaison 1280 par framebuffer/CPU,
pas par un essai GPU matériel distinct. Pas de validation Windows/PowerShell ou
manette réelle, ni de traversée humaine native revendiquée. Aucun blocage d'API
GPE ; ces réserves matérielles et de ressenti conditionnent le GO final.

## Lancer et reproduire

Natif : `cargo run --locked --release --bin pirates-reborn`.
Windows : `cargo build --locked --release`, puis
`.\target\release\pirates-reborn.exe`. Mise à jour d'une copie propre : `.\up.ps1`.

Web : `python3 -B scripts/package_web.py`, puis
`python3 -m http.server 8000 --bind 127.0.0.1 --directory dist`.
Ouvrir `http://127.0.0.1:8000`. Pour l'essai automatisé, servir sur 8765 et suivre
les variables Playwright/Chromium décrites dans le [rapport P1](p1-validation.md#reproduire-lessai-navigateur).
PowerShell : `py -3 -B -m unittest discover -s tests -p test_up_ps1.py -v`.

**GO WITH CONDITIONS — arrêt à P1.1.** Aucun démarrage P2 ni autorisation de push/merge.
