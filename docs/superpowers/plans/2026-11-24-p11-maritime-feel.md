# P1.1 — Maritime Visual & Gameplay Feel

Mission du propriétaire du 24 novembre 2026 ; exécution locale dans la branche
de travail propre `feat/p1-maritime-navigation`, base P1 `84e869e`.

## Choix et contraintes

Rendu pixel original en code, inspiré des palettes/matières des quatre images
`I:\pirates_1.png` à `pirates_4.png`, sans les copier. GPE reste exactement
`8cbceda0c9b0cc55ee5deb7d5e173676ffc1d4f1`. Aucun Core, dépendance, P2, push
ou merge. Simulation entière, polaire, tick, rotation, coque ±6 et ports conservés.

960×540 est le candidat prioritaire ; comparaison CPU et captures avec 1280×720.
640×360 reste la référence avant. Fenêtre initiale 1440×810 (1,5×), ratio/nearest
GPE conservés ; essais ×1 et ×2. Le dessin utilise les pixels supplémentaires
pour les détails et une surface de navigation plus grande, sans agrandir le monde.

Deux alternatives écartées : agrandir le framebuffer ancien sans enrichir les
motifs ; introduire un moteur de terrain ou un pipeline d'assets. Le sprite de
26 pixels est dessiné dans ses coordonnées locales puis orienté sur les 16 caps.
La caméra réserve une marge aux bords pour sa taille indépendante de la coque.

Les îles fixes sont des unions de petits polygones convexes, avec lobes et baies.
Leur géométrie est la source commune du dessin et du balayage de la coque carrée.
Les polygones restent dans les anciennes enveloppes ; accès et fixture P1 conservés.
Mer animée et sillage sont exclusivement de présentation. Les décors fixes sont
mis en cache côté jeu, sans nouvelle abstraction moteur.

M / West ouvre la carte complète et suspend les ticks ; M / West / Échap / Start
ferme la carte, en conservant une pause qui préexistait. Accumulateur et ordres
ponctuels vidés. Au retour, reprendre les ticks au cap/voilure précédents ; attendre un état physique neutre avant toute nouvelle commande nautique.

## Plan et vérifications

- [x] Lire instructions, P0/P1, Git réel, sources/API GPE et références.
- [x] Gate P1 de référence, benchmark avant, préflight réel natif/WASM 960×540.
- [x] Tests rouges : carte/pause/retour neutre, contour/collision, sprite/marges.
- [x] Géométrie fixe partagée, adapter caméra, carte et entrées sans changer les règles.
- [x] Art original : navire, eau/vagues/sillage, plages/végétation/reliefs, ports, HUD.
- [x] `up.ps1` et tests d'intégration Git temporaires, préserver intégralement Bash.
- [x] Gate canonique, release, packaging, comparaison native/WASM et navigateur.
- [x] Captures avant/après/×1/×2/carte/1280, benchmark, revue et rapport README/roadmap.

Risques à vérifier : voilier masqué aux coins ou par le HUD ; terre dessinée hors
collision ; carte ouverte avec commandes en attente ; reprise avec stick maintenu ;
retour d'erreur Git/Cargo PowerShell. Windows physique et manette restent des essais
distincts. Pas de commit nécessaire à cette livraison locale.

## Journal

Référence : `bash scripts/check.sh` PASS ; 17 tests Rust, 14 Python, 1 Node.
CPU avant : 0,022 µs/tick et 0,159 ms/dessin640×360. Chromium151 SwiftShader :
737 submits, état natif/WASM identique, aucune erreur ; cadence de scénario39,90ms
(ne constitue pas une mesure GPU isolée). Préflight960 réel natif et Web inspecté.

Revue indépendante : corrections de la pause cachée au retour, neutralité physique
avant arbitrage, contrôles Git après pull et sequencer, aide de carte déplacée au
HUD, enregistrement des arguments Cargo réels dans la fixture Windows. Tests
rouges puis verts pour les transitions Rust ; tests PowerShell NOT RUN faute de
runtime. Aucun problème important laissé ouvert après les corrections.

Le plan est exécuté dans le checkout de travail demandé, sans créer de worktree
ou commit supplémentaire. Aucun push/merge. Les captures et réserves définitives
sont consignées dans `docs/p11-validation.md`.
