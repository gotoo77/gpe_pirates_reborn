# Pirates Reborn - Direction artistique & UX v0.2

Statut : proposition révisée par auto-revue adversariale le 27 novembre 2026. Contrat de conception pour le pilote P2-X3, à accepter avant implémentation. Le résultat artistique reste à démontrer. Remplace la proposition v0.1 ; aucune nouvelle image dans cette phase.

## Revue adversariale de v0.1

| Finding | Gravité | Traitement |
|---|---|---|
| D1 mélange style, assets, entrées et plusieurs écrans | Haute | Tranches distinctes avec sortie observable et arrêt |
| Palette et mots d'ambiance ne font pas un kit artistique | Haute | Contrat d'assets, typographie et composition, échantillon approuvé avant intégration |
| Boutons dessinés sans hitboxes réelles | Haute | Contrôle sémantique unique, états testés, pas de faux bouton |
| Unité 100 kg et charge 3 t hypothétiques | Haute | Convention bijective proposée, décision explicite ; masses différentes hors scope |
| Capacités UI GPE listées sans preuve dans Pirates | Haute | Spike exact au pin, fonts/licences/input/captures/perfs natif et WASM |
| Carnet annoncé séparé mais D1 trop large | Moyenne | Déplacer seulement les cotations existantes dans une destination dédiée ; pas de nouvelles données |
| Cible 60 FPS et délais UX non établis | Moyenne | Comparaison mesurée à conditions identiques, seuils proposés, pas de garantie |
| Acceptation trop dépendante des tests techniques | Haute | Gate produit séparée : art, lisibilité, découverte des actions et envie de poursuivre |
| Risque de framework et de dépenses IA récurrentes | Haute | Méthode GPE commune et extraction différée, budget par tranche |

Verdict v0.1 : orientation pertinente, insuffisante pour une exécution autonome en un seul lot. v0.2 : cadrage révisé ; pas d'approbation indépendante. Les décisions de design et le budget restent à confirmer par le propriétaire.

## État observé

P2-X2 témoin : `6ea1fd1c83e37a99f7ff158a051d4deb200ea496`, branche `exp/p2-x2-market-ux`. Capture native du propriétaire : cartes, icônes sommaires, 30 cases de cale, carnet et explications permanentes. Le résultat a été refusé sur le plan artistique. Lancement natif observé ; aucun résultat navigateur ni rapport complet P2-X2 disponible ici. Économie : 3 biens, lots entiers, 30 unités de capacité, devis exact, prix marginaux, observation datée, simulation gelée dans le marché. GPE pin `8cbceda0c9b0cc55ee5deb7d5e173676ffc1d4f1`, framebuffer 960 x 540. `src/draw.rs` rendu, `src/app.rs` entrées/état, `src/economy.rs` règles. P2.0 reste NO-GO ; laboratoire économique gelé.

## Direction : aventure maritime illustrée

Crédible, adulte, romanesque ; pas de reconstitution historique obligatoire. L'interface doit évoquer l'objet ou le lieu : comptoir, carnet, carte, cale, chantier naval. Pas de caddie moderne, d'emoji, de mascotte caricaturale, de texture décorative derrière toutes les données, de contour néon ou d'accumulation de symboles pirates. Les lieux et personnages sont fictifs.

Palette de départ : encre #14262D, mer #254D5B, papier #E3D0AA, bois #493227, laiton #BC9354, bordeaux #773C37, vert ardoise #426F60. Cette palette est une proposition, pas une règle imposée à tous les jeux. Titres à caractère d'époque, corps lisible et nombres stables, accents français, casse normale. Le framebuffer n'impose pas une police pixel. Aucun effet graphique ne remplace la lisibilité.

Avant production : approuver une composition annotée (zones, dimensions, hiérarchie et marges), un petit échantillon de matières/typo/iconographie et une liste d'assets nécessaires, avec provenance et licences. Assets originaux ou compatibles, sélectionnés ou produits manuellement dans ce chantier ; pas de génération d'image sans nouvelle demande. Pas de screenshot aplati contenant des valeurs et boutons. Aucun asset supposé existant. Si aucun kit crédible n'est disponible, stopper l'intégration artistique et consigner le besoin. Ne pas faire payer à Codex une génération procédurale de décor de fortune.

## Grammaire de tous les écrans

Une intention principale, des zones calmes, peu de destinations explicites. Actions métier courtes et incarnées, retour cohérent, aide contextuelle unique (icône ?). Les instructions de manipulation vont dans l'aide ; aucun bandeau manuel permanent. Les erreurs métier et états nécessaires restent visibles. L'aide peut afficher les commandes réellement actives, adaptées au périphérique ; retour conserve sélection et devis ou le recalcule explicitement si état changé.

Souris et tactile doivent activer les vrais contrôles visibles ; clavier et manette les mêmes actions via focus et navigation spatiale. Focus visible distinct du survol et de la sélection. Contrôles actifs, pressés, indisponibles et validés. Aucun clic traversant, aucune transaction sur mouvement de focus, un seul échange par confirmation, neutralisation à la reprise, pas de reset accidentel. Un bouton apparent est activable ou clairement désactivé avec raison ; pas de bouton vers une fonction future.

## Marché de référence

Choisir une cargaison puis conclure un échange. Pas de carnet permanent ni de chiffres historiques répétés. En-tête léger : port, marché, bourse, retour, aide et accès carnet/cale si disponible. Grande zone d'étal : trois illustrations de biens, nom, disponible, prix de l'opération active avec unité. Bon d'échange distinct : Acheter/Vendre, quantité, montant exact, solde après, action principale. Cale résumée par charge et capacité, pas par un tableau de cellules. Une indication courte de la cause d'une action impossible près du contrôle ; feedback de succès non bloquant. Pas de son obligatoire pour comprendre. Ne pas fabriquer de prix de lot constant à partir du prix marginal.

## Carnet et cale

Carnet = destination distincte, pas un widget de marché. Les observations existantes : port, bien, dernière cotation achat/vente et ancienneté ; inconnue si non visitée. Consultation et retour sans mutation, gel du temps à définir précisément dans le plan, aucun calcul de profit garanti ni découverte inventée. Première version limitée aux données existantes. La carte et les autres renseignements sont des possibilités ultérieures. Cale = destination dédiée à la cargaison si nécessaire ; charge totale et par bien, unités cohérentes. Aucun autre service portuaire fictivement activable.

## Unités : transition et vraie physique

Proposition U0 : conserver tous les lots entiers actuels, présenter sucre et bois par lots de 100 kg brut, outils par caisse conventionnelle de 100 kg brut, capacité 3 000 kg = 30 lots. Stock 32 -> 3,2 t, 8 lots -> 800 kg, 6 or/lot -> 6 or/100 kg. Convention de jeu à valider, pas un modèle historique. Même quantité discrète, même prix, même capacité, même transaction, même résultat ; aucune conversion arrondie qui modifie les règles. Afficher prix et dénominateur partout, y compris carnet et stock ; ne pas masquer qu'une caisse compte comme un lot de masse donnée. Si U0 n'est pas validée, afficher lots, pas des tonnes fictives.

U1 séparée (non autorisée) : masses par conditionnement réellement distinctes, unités de volume pour les liquides, masse brute calculée avec densité conventionnelle et contenant. Ne jamais additionner kg et litres. Entiers pour comptabilité et masse, conversions vérifiées, aucune seconde contrainte de volume sans besoin observé. Toute modification de capacité ou de prix est un changement économique et exige contrat et validation propres. Ne pas relancer P2.0 implicitement.

## Carte des écrans

Navigation : choisir un cap et voyager (existe) ; port : choisir un service (proposition) ; marché : échange (cible du pilote) ; carnet : information connue (tranche minimale) ; cale : chargement (résumé d'abord, écran ensuite si utile) ; carte : préparer un trajet (existe) ; navire : comprendre/aménager (vision) ; relations : alliances et correspondances (vision) ; aide/pause : compréhension et retour (pilote). Aucun chantier combat, missions, factions ou carrière dans P2-X3.

## GPE : capacités vérifiées et limites

Au pin cité : PNG et régions d'images (`Image::decode_png`, `Framebuffer::draw_image_region`), bitmap personnalisée, mesure texte, `outline-fonts` optionnelle non activée dans Pirates, `Input` souris/tactile/pad/clavier, `Ui`, `UiState`, styles et répétition, audio disponible. Vérifier les signatures et comportement exacts avant usage. Les capacités annoncées ne sont pas une preuve d'intégration de Pirates. Sources : https://github.com/gotoo77/gotoo-pixel-engine/tree/8cbceda0c9b0cc55ee5deb7d5e173676ffc1d4f1/src .

Typo et assets à tester dans le framebuffer existant d'abord ; comparer 1280 x 720 seulement si 960 x 540 est insuffisant, même poste et conditions. Mesurer coûts du rendu et temps de frame, pas seulement FPS affichés. Privilégier les fonctions GPE existantes ; vérifier focus/états/hit-test en particulier. Si une lacune avérée bloque la qualité, documenter reproduction et ouvrir un chantier moteur distinct, sans migration implicite. Pas de nouveau framework UI, layout universel ou gestionnaire d'assets spéculatif.

## Acceptation produit

Captures à états réels, pas seulement écran vide : achat, vente, stock épuisé, charge pleine, manque d'or, carnet connu/inconnu, aide/retour. Contrôles opérables et textes non coupés au format de référence et redimensionnement. Tester au moins trois personnes naïves si possible : marché trouvé à quai en 10 s, achat et vente en 30 s chacun, cotation passée retrouvée en 15 s, sans aide imposée. Ce sont des cibles à confirmer, non des mesures acquises ; noter le nombre de sujets et échecs. Demander perception de l'univers et envie de tenter une traversée. Gate propriétaire : « je veux jouer à cet écran » ; des tests techniques verts ne donnent pas un GO artistique. Si échec, une correction ciblée sur le problème observé ou un HOLD ; ne pas embellir indéfiniment.

Qualification : gate canonique inchangé, tests de transaction et de navigation conservés, tests supplémentaires de conversion, focus/entrée, action unique, retour et absence de fuite. Natif et navigateur réel, contrôles matériels selon disponibilités, captures CPU reproductibles et mesures comparables. Incident GPE-Agent #1382 hors scope. Aucun merge avant acceptation explicite.

Voir `p2-x3-execution-plan.md` pour les tranches et le point d'arrêt, et `gpe-development-method.md` pour les règles communes.
