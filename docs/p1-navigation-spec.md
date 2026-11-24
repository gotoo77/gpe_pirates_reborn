# P1 — Maritime Navigation Sandbox

**Statut : implémenté localement sur autorisation de la mission P1 du 24 novembre 2026.** Le [rapport P1](p1-validation.md) donne les validations obtenues et encore ouvertes. Cadrage P0 du 24 novembre 2026. Toutes les valeurs ci-dessous sont des règles originales de Pirates Reborn, ajustables seulement avec mise à jour de la spécification et de ses tests. Elles ne décrivent pas les formules de Pirates! CPC.

## Objectif et boucle jouable

Prouver qu'un navire seul peut quitter un port, choisir son cap avec le vent, contourner une île et atteindre un autre port. Le joueur peut repartir et visiter le troisième. Une traversée doit demander une décision de route, sans combat ni économie.

Livrer seulement : carte fixe originale, quatre îles, trois ports nommés, un navire, caméra, collisions terre/bord du monde, direction et force du vent, HUD et graphismes temporaires lisibles. Hors scope : combat naval, duels, commerce, génération procédurale, politique, IA des factions, persistance, multijoueur, éditeur. Pas de système générique de navires, de scènes, d'inventaire ou de quêtes préparant ces fonctions.

## Préflight obligatoire

L'[audit GPE](gpe-capability-audit.md) vérifie les API de la révision **8cbceda0c9b0cc55ee5deb7d5e173676ffc1d4f1**, proposée comme dépendance Git verrouillée. Le checkout actif **39eec6b…** est différent. Avant de coder P1 autorisée : confirmer ce pin, établir les entrées minimales native et Web et démarrer réellement les deux à la résolution choisie. Installer la cible WASM manquante dans cette phase, sans changer de moteur.

**Capability preflight hard stop :** si une capacité indispensable ne fonctionne pas sur la cible requise, arrêter le travail dépendant et rapporter la preuve ; demander une décision sur le besoin ou le périmètre. Ne pas inventer une API, ajouter une dépendance ou corriger GPE Core implicitement. Si le pin change, refaire le contrôle des capacités utilisées.

## Carte et arrivée

Monde fixe **1600 × 1000 unités**, une unité = un pixel du monde à l'échelle nominale. Origine nord-ouest, X vers l'est, Y vers le sud. Mer libre sauf les rectangles terrestres suivants, donnés en `(x, y, largeur, hauteur)` :

| Île originale | Rectangle solide |
| --- | --- |
| Île de la Brise | `(96, 160, 192, 240)` |
| Île du Phare | `(1120, 160, 224, 256)` |
| Île de l'Anse | `(608, 672, 256, 192)` |
| Grande Dorsale | `(672, 192, 256, 320)` |

Ces formes temporaires sont assumées rectangulaires : leurs contours visibles doivent correspondre aux collisions. Ne pas dessiner de plage navigable sur un rectangle solide. Les détails décoratifs restent à l'intérieur des terres.

| Port | Centre de la zone d'arrivée, en mer |
| --- | --- |
| Port-Brise | `(320, 280)` |
| Port-Phare | `(1088, 288)` |
| Port-Anse | `(736, 640)` |

Chaque port a un disque d'arrivée de rayon **24** et un repère de quai tourné vers son île. Les zones et la coque doivent rester hors des terres. Initialement, le centre du navire est à Port-Brise, voiles baissées, cap est. Aucun service ou écran de port : entrer dans une zone depuis l'extérieur affiche `ARRIVEE PORT-...`, baisse les voiles et mémorise ce port comme dernier visité pour cette session seulement. Le départ se fait en hissant les voiles ; rester dans la zone ne redéclenche pas une arrivée. Comparaison par distance au carré, sans racine carrée.

L'île centrale coupe la route directe Brise → Phare. Un passage par le sud, approximativement `(480,560)` puis `(1024,560)`, laisse de l'eau libre. Ces points servent de repères de conception, pas de pilote automatique ou de preuve de traversée déjà exécutée. Une séquence d'entrées effective devra être enregistrée et vérifiée en P1.

## Simulation reproductible

La règle s'exécute par ticks de **20 ms (50 Hz)**. Le callback `Game::update` accumule `Frame::delta_time` en durée entière, exécute les ticks complets, conserve le reste puis dessine. GPE plafonne déjà chaque delta à 100 ms ; avec un reste inférieur à 20 ms, cela représente au plus cinq ticks par callback. Aucun calcul de déplacement directement à partir du nombre d'images, aucun hasard ni lecture d'heure système.

Séparer seulement la règle pure de simulation de l'adaptation `Frame` et du dessin pour permettre ses tests. Pas de bibliothèque de replay ou d'ordonnanceur générique. L'état de test comprend position et restes de division, cap, compteur de rotation, voilure, vent, tick et état d'entrée/sortie des ports. La caméra et le HUD ne modifient pas cet état.

Les positions sont des entiers signés à **1024 sous-unités par unité** ; intermédiaires en `i64`. Les caps sont **16 directions**, indexées dans le sens horaire : 0 est, 4 sud, 8 ouest, 12 nord. Les vecteurs approchés du premier quadrant sont `(1024,0)`, `(946,392)`, `(724,724)`, `(392,946)`, `(0,1024)` ; compléter les autres quadrants par symétrie, sans trigonométrie à l'exécution. Leur petite erreur de norme est assumée et bornée ; les tests prennent cette table comme référence.

La barre vaut -1, 0 ou +1. Un changement vers une direction non nulle tourne immédiatement d'un secteur ; maintien : un secteur supplémentaire tous les **10 ticks**, soit 112,5 degrés/s en régime établi. Relâcher remet le compteur à zéro ; directions opposées simultanées donnent zéro. Le compteur et le signe précédent font partie de l'état déterministe. Le navire peut tourner sur place, ce qui évite de rester bloqué contre la côte.

Chaque tick applique dans cet ordre : commandes ponctuelles de voilure → barre/cap → vitesse due au vent → déplacement et collision → entrée/sortie des ports → compteur temporel. Une arrivée baisse les voiles après le déplacement admis. `R` rétablit exactement l'état initial, vent compris.

### Vent et vitesse

Le vent possède un cap `w` dans la même table et une force `F` de 0 à 3. **Le cap indique où souffle le vent**, pas sa provenance météorologique. Session normale : `w = 0` (vers l'est), `F = 2`, constants jusqu'au redémarrage. Aucun cycle météo, turbulence, courant ou dérive latérale. Les autres forces/directions sont injectables dans les tests, sans créer un menu météo.

Définir `d = min((cap-w) modulo 16, (w-cap) modulo 16)`, de 0 à 8. La vitesse de base `B` vaut respectivement **0, 48, 72, 96 unités/s** pour F=0,1,2,3. La polaire vaut :

| d | Angle au vent vers lequel il souffle | Pourcentage P |
| --- | --- | --- |
| 0 | 0°, vent arrière | 85 |
| 1 | 22,5° | 95 |
| 2 | 45°, arrière oblique | 100 |
| 3 | 67,5° | 95 |
| 4 | 90°, travers | 80 |
| 5 | 112,5° | 60 |
| 6 | 135° | 40 |
| 7 | 157,5° | 25 |
| 8 | 180°, face au vent | 15 |

Voiles hautes : vitesse cible `B × P / 100`. Voiles basses : zéro. À F=2, comparer **72** à d=2, **57,6** au travers et **10,8** face au vent ; sans collision, le changement est immédiatement perceptible. Ce modèle laisse toujours avancer face au vent si F>0, pour garder tous les ports accessibles sans apprentissage du virement de bord. Pas d'inertie, d'accélération, de marche arrière ou de réglage fin de voilure en P1.

Pour chaque axe, calculer le déplacement en sous-unités à partir du vecteur entier H : `numérateur = H_axe × B × P × 20 + reste_axe`, diviser par **100000** avec la division entière Rust vers zéro, conserver le reste signé. Ainsi on ne tronque pas prématurément la vitesse et on n'accumule pas une dérive asymétrique aux caps négatifs. Au calme ou voiles baissées, déplacement nul et restes remis à zéro. Le HUD peut présenter une valeur décimale ; aucun flottant n'entre dans la règle de déplacement.

**Garantie visée :** même état initial, même table, mêmes commandes par tick et même nombre de ticks → même état entier, natif/WASM compris. Des entrées humaines reçues à des instants différents ne sont pas une même séquence de commandes. Pour tester deux cadences de rendu, fournir la même séquence indexée par tick ; ne pas prétendre que deux parties manuelles seront identiques.

### Adaptation des entrées

Lire les commandes continues une fois par callback et les maintenir pour les ticks à venir. Les ordres ponctuels `pressed()` sont mémorisés si aucun tick n'a lieu, puis consommés une seule fois au prochain tick ; plusieurs ticks dans une image ne doivent pas répéter un ordre. Des ordres de voilure simultanés donnent priorité à baisser les voiles ; une remise à zéro annule les autres commandes en attente. La pause agit sur l'adaptateur, vide son accumulateur et les ordres en attente ; elle ne produit aucun tick de simulation. Reprendre utilise les états continus courants, sans rattrapage du temps passé en pause.

La perte de focus est traitée temporellement par GPE, mais aucune API de pause automatique du jeu n'a été confirmée. Ne pas la promettre : le joueur dispose d'une pause explicite, à tester aussi dans le navigateur.

### Collision

La coque physique est un **carré aligné sur les axes de demi-largeur 6**, indépendant du cap graphique. Son dessin tient dans ce carré ; les zones de port sont sur l'eau. L'étendue du monde est une barrière solide.

Pour chaque déplacement proposé, balayer le segment du centre contre chaque rectangle terrestre élargi de 6 unités sur ses quatre côtés, en coordonnées de sous-unités. Utiliser des comparaisons entières/rationnelles, sans échantillonner uniquement la position finale. Toucher une frontière compte comme un blocage ; le centre doit aussi rester strictement dans `(6,1594) × (6,994)`. Cette convention conservatrice doit être identique dans les tests.

Si une intersection existe, refuser **tout le déplacement de ce tick**, garder la position précédente, remettre les restes de déplacement à zéro et indiquer `TERRE` dans le HUD. Pas de rebond, de dégâts ou de glissement automatique. La barre reste active ; tourner puis repartir doit permettre de se dégager. Le test de balayage est une fonction spécialisée navire/terre, pas un nouveau moteur physique.

## Contrôles fondés sur les API confirmées

| Commande | Clavier GPE | Manette GPE |
| --- | --- | --- |
| Barre gauche/droite | `Key::Left` / `Key::Right`, `held()` | `DPadLeft` / `DPadRight` ; ou stick gauche X converti en -1/0/+1 au seuil ±0,35 |
| Hisser les voiles | `Key::Up`, `pressed()` | `DPadUp`, `pressed()` |
| Baisser les voiles | `Key::Down`, `pressed()` | `DPadDown`, `pressed()` |
| Pause/reprise | `Key::Escape`, `pressed()` | `GamepadButton::Start`, `pressed()` |
| Recommencer | `Key::R`, `pressed()` | `GamepadButton::Select`, `pressed()` |

`Input::gamepad_axis(id, GamepadAxis::LeftStickX)` est confirmé à 8cbceda, ainsi que les états des boutons. Le D-pad a priorité sur le stick s'il est actionné ; ne pas appliquer les deux. Choisir une seule manette : plus petit `GamepadId::as_usize()` connecté à l'initialisation, conservé jusqu'à sa déconnexion, puis même règle ; ne pas dépendre de l'ordre d'un HashMap. Clavier directionnel prioritaire s'il est actionné. Axe inconnu/absent : D-pad ; périphérique déconnecté : commandes neutres et clavier disponible. Une déconnexion n'abaisse pas implicitement les voiles.

Le stick est volontairement numérique pour partager les règles et les tests du clavier. Il n'y a pas de déplacement absolu vers le pointeur. Les touches utilisent les flèches pour éviter de supposer ZQSD. Aucun support tactile requis en P1.

## Présentation

Vue du dessus, framebuffer **640 × 360**, fenêtre initiale **1280 × 720**. Le monde occupe les **312 premières lignes** ; HUD de 48 pixels en bas. Caméra centrée sur le navire, bornée entre `(0,0)` et `(960,688)` pour ce cadre de 640 × 312 ; origine arrondie seulement au dessin, sans lissage ou zoom. Le HUD reste en coordonnées écran. La présentation GPE conserve le ratio et emploie nearest, sans garantir un facteur entier après tout redimensionnement.

Mer, terres, limites de coque, côtes et ports doivent se distinguer par forme et contraste. Flèche de proue conforme au cap, flèche du vent accompagnée de `VENT VERS E / FORCE 2`, voilure haute/basse visible. HUD : cap, vitesse cible, état des voiles, port le plus proche avec direction approximative, arrivée ou blocage, aide concise. En cas de collision afficher vitesse effective nulle pour le tick et `TERRE`, afin de ne pas laisser croire à un mouvement réel. Éviter de faire reposer la compréhension sur la seule couleur. Libellés ASCII courts possibles pour rester dans les glyphes bitmap vérifiés.

Mesurer le coût du dessin et des ticks, vérifier au moins ×1 et ×2 et le redimensionnement ; la résolution est une proposition conditionnelle, aucune performance n'est déjà prouvée. Une réduction éventuelle demande une décision explicite et l'actualisation des critères visuels. Audio non requis.

## Critères d'acceptation et stratégie de tests

| ID | Vérification objective exigée en P1 |
| --- | --- |
| A1 — Traversée | Départ Brise, contournement de Dorsale et arrivée Phare sans téléportation ni passage terrestre ; même possibilité vers Anse et au retour. Conserver une séquence de commandes par tick pour Brise → Phare atteignant l'arrivée en moins de 180 s simulées. Compléter par parcours manuel. |
| A2 — Terres solides | Tests de face, angle, tangence, bornes du monde et segment traversant une île ; position admise toujours libre, y compris si une étape artificiellement longue est donnée au test de balayage. Tourner puis repartir après contact. |
| A3 — Vent | Tests exacts de la table, symétrie d et 16-d, caps 0/15, F=0..3 ; vitesses à F=2 ci-dessus, voiles basses immobiles. À état et durée égaux sur eau libre, changer seulement le cap change la distance conformément à la table de vecteurs et à la polaire. |
| A4 — Reproductibilité | Même état et commandes sur au moins 10000 ticks : égalité exacte de tout l'état ; comparer résultats natif/WASM avec la même séquence de référence. Vérifier restes signés, pause, reset, et regroupements du temps reçu en callbacks différents à total identique. |
| A5 — Adaptateur | Tests ciblés : ordre ponctuel reçu avant un tick puis consommé une fois ; cinq ticks sans répétition ; opposés neutralisés ; priorité baisser ; déconnexion ; aucune progression en pause. Aucun besoin de simuler un renderer GPU pour ces règles. |
| A6 — Ports et carte | Vérifier non-chevauchement des terres/arrivées, départ libre, arrivée déclenchée une seule fois, départ et réentrée, trois ports atteignables et noms distinguables. |
| A7 — Lisibilité | Capture et essai manuel à 640 × 360 et ×2 : proue, côte, vent, voiles et arrivée compris ; contrôles entièrement clavier puis manette réelle. Redimensionner et tester pause/reprise/focus, sans annoncer une auto-pause inexistante. |
| A8 — Cibles | Build locked release Linux natif et WASM ; paquet JS/WASM/HTML servi, démarrage dans un navigateur WebGPU réel, parcours complet sur les deux. Consigner OS, GPU, navigateur, manette, versions, temps de frame et éventuels blocages. Un build WASM seul ne valide pas le navigateur. |
| A9 — Qualité | Tests ciblés puis gate canonique du futur dépôt, fmt/check/test locked, **Clippy strict `-D warnings`**, diff-check ; ne pas réduire les contrôles pour obtenir un résultat vert. |

Les tests automatisés doivent établir les règles et les cas limites ; les essais manuels valident la compréhension et le ressenti. Une mauvaise lisibilité ou un vent imperceptible reste un échec d'acceptation même si les tests passent. **Aucun de ces tests gameplay n'a été écrit ou exécuté en P0.** Les tests présents concernent exclusivement `up.sh`.

## Choix et limites du cadrage

**RETENU EN P1 :** caps discrets et calcul entier, retenus ici pour rendre la règle petite et reproductible. Une rotation continue en flottants serait plus fluide mais demanderait de définir des tolérances entre plateformes ; une simulation nautique complète multiplierait les paramètres sans servir cette première boucle. La mission P1 autorise cette mise en œuvre ; le ressenti reste à valider par le propriétaire.

Le test P1 décidera si la granularité de 16 caps convient au ressenti. Aucun système de navigation ultérieur, de voile réaliste ou de combat n'est réservé dans l'architecture. L'implémentation a suivi le préflight des deux cibles et s'arrête à P1. Les capacités et limites constatées sont consignées dans le rapport P1.
