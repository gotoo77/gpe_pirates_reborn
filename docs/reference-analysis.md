# Pirates! sur Amstrad CPC — analyse P0

Recherche publique effectuée le **24 novembre 2026**, sans image disque, émulation ni téléchargement du jeu. Cette analyse porte sur le port CPC de l'original MicroProse, pas sur Pirates! Gold ou le remake 3D de 2004. [CPCWiki distingue ces éditions][CW]. Les catalogues CPC consultés indiquent 1987 ; des articles contemporains accessibles datent de 1988. Cette recherche ne tranche pas une date de commercialisation territoriale précise. [CPC-POWER][CP], [article d'avril 1988][CR].

## Niveau de preuve

- **OBSERVÉ** : information effectivement lue dans la source indiquée. « OBSERVÉ — manuel original » ne signifie pas vérification dans un exécutable CPC.
- **INFÉRÉ** : interprétation des interactions ou rapprochement avec le port CPC ; explicitement distinct d'une mécanique CPC démontrée.
- **PROPOSÉ** : choix nouveau pour Pirates Reborn, sans attribution au jeu original.

Le manuel original est accessible sous forme de [transcription hébergée dans une archive C64][M]. C'est un témoignage documentaire de première main dans une transcription tierce, dont l'intégrité et la correspondance exacte avec le livret CPC ne sont pas établies. Le [scan annoncé comme manuel CPC sur GamesDatabase][PDF] a renvoyé 404 à l'ouverture ; les extraits indexés mentionnent aussi C64/C128, ce qui interdit de le traiter comme une preuve CPC spécifique. Le [catalogue CPCRULEZ][CR] propose une notice anglaise, mais son lien de consultation a renvoyé 403. Aucune de ces deux tentatives n'est présentée comme une lecture réussie du manuel CPC.

## Cartographie des mécaniques

Les formulations ci-dessous résument les sources ; aucun chiffre d'équilibrage n'est repris pour l'implémentation.

| Système | OBSERVÉ et provenance | Limite pour le CPC |
| --- | --- | --- |
| Navigation et vent | Cap conservé sans ordre ; vent contraire lent, allure arrière oblique généralement rapide. [Manuel, « Travel by Sea »][M]. | Polaire et réglages exacts CPC non vérifiés. |
| Temps et déplacements | Voyage consomme temps et nourriture, avec pression liée aux hommes et à leur humeur. [Comparatif des versions, introduction au gameplay][V]. | Cadence, calendrier et coût exact d'une étape CPC inconnus. |
| Géographie | Aventure caribéenne, trajets entre villes ; recherche de trésors et de parents. [CPC Game Reviews][CG], [CPCWiki][CW]. | Pas de relevé exhaustif de la carte CPC ni de ses coordonnées. |
| Ports et services | Tavernes pour recruter, marchands pour échanger. [Critique CPC][CG]. Réparation et vente de navires décrites dans le [manuel][M]. | Réparation non recoupée directement sur CPC. |
| Économie et commerce | Vivres, marchandises et soute limitée ; achat, vente et réparation. [Manuel, « Trading with Merchants »][M]. | Prix, unités, stocks et variations CPC inconnus. |
| Factions | Anglais, Français, Hollandais, Espagnols. [Critique CPC][CG]. Choix d'époque et nationalité signalés par [CPC-POWER][CP]. | Pas de formule diplomatique CPC démontrée. |
| Rencontres maritimes | Attaque et pillage de navires, combat naval et abordage en duel décrits par la [presse CPC][CR]. | Fréquences et algorithmes non relevés. |
| Équipage | Recrutement en taverne [confirmé CPC][CG]. Nourriture, moral, désertions et partage du butin documentés dans l'[original][M]. | Seuils et formules du port non établis. |
| Réputation | Parts généreuses favorisent le recrutement ; actes envers une nation influencent son accueil. [Manuel, carrière et nationalité][M]. | Transposition exacte au CPC seulement inférée. |
| Progression du capitaine | Récompenses, terres, rang, fortune et famille participent au bilan. [Manuel, « Your Career on the High Seas »][M]. Objectif retraite riche et influente [signalé CPC][CW]. | Pas de modèle de compétences ou d'XP présumé. |
| Fin de carrière | Retraite après partage ; santé dégradée pouvant empêcher une nouvelle expédition. [Manuel, carrière][M]. Retraite avec argent et terres [confirmée comme objectif CPC][CP]. | Âge limite, score et conditions de défaite CPC non mesurés. |

## Interactions et boucles

**INFÉRÉ — boucle de route.** Si le cap influe sur l'allure et que le voyage coûte du temps, un détour peut devenir un bon choix. Cette lecture relie navigation et logistique ; elle ne démontre pas une optimisation précise du moteur CPC. Pour éprouver cette idée sans économie, P1 peut opposer une route directe obstruée à un passage maritime plus favorable.

**INFÉRÉ — boucle d'expédition.** Le recrutement et les ressources peuvent produire un arbitrage : préparer un voyage, chercher une occasion, décider du moment du retour. Le partage marque alors une respiration entre expéditions. Le caractère émergent provient du choix de prolonger ou d'écourter l'aventure ; aucun générateur d'histoires n'est nécessaire à cette interprétation.

**INFÉRÉ — boucle de conséquences.** Relier succès, accueil et recrutement donne une mémoire aux actes du joueur. Une récompense immédiate peut réduire les possibilités futures. C'est une hypothèse de conception tirée du rapprochement des systèmes documentés, pas la preuve d'un moteur politique particulier sur CPC.

**INFÉRÉ — horizon de carrière.** Un bilan final donne du poids aux trajets intermédiaires. Pour Pirates Reborn, la question utile sera ce que le capitaine choisit d'obtenir avant de s'arrêter, plutôt qu'une accumulation automatique de contenu. Cette conclusion ne fixe aucune durée ou règle de vieillissement.

La [critique CPC contemporaine][CR] présente elle-même le jeu comme un mélange d'aventure, de stratégie et d'arcade, avec une grande liberté. Son récit introductif est romancé : les noms, dialogues, situations et chiffres de ce récit ne sont pas des observations de partie reproductibles. Ils n'ont pas été utilisés comme données du futur jeu.

## Ce que Pirates Reborn propose d'en retenir

**PROPOSÉ — P1 :** rendre intéressante une seule traversée par la combinaison du cap, du vent et de la forme des terres. Carte et ports inventés ; pas de copie de la géographie historique du jeu. Le vent est constant pendant une session et les règles sont chiffrées dans la [spécification P1](p1-navigation-spec.md). Les tables de direction et de vitesse sont des propositions originales.

**PROPOSÉ — après P1 seulement :** examiner si des ressources limitées donnent un motif aux voyages (P2), puis si des rencontres et conséquences rendent leurs résultats mémorables (P3–P5). Il s'agit d'hypothèses, sans modèle de données anticipé en P1. Voir la [roadmap](roadmap.md).

## Incertitudes et recherches ultérieures possibles

- **OBSERVÉ :** aucune partie CPC n'a été exécutée ou visionnée image par image dans cette mission. Les témoignages et descriptions CPC ne remplacent pas un relevé instrumenté.
- **INFÉRÉ :** la parenté documentaire entre l'original et son port est plausible ; elle ne justifie pas de recopier les formules C64 comme des faits CPC. Garder en particulier ouverts la polaire, les coûts temporels, les prix, le moral, les effets diplomatiques et le bilan de retraite.
- **PROPOSÉ :** si nécessaire plus tard, consulter un livret CPC identifiable et une observation vidéo horodatée pour ces points précis. Aucun besoin de DSK pour terminer P0 ou construire la navigation originale de P1.
- Les règles de 1993/2004 ne servent pas à compléter les blancs. Aucun système de danse, de spécialistes ou d'amélioration de navire issu d'un remake n'est attribué au CPC.

## Sources consultées

| Repère | Source, nature et usage |
| --- | --- |
| M | [Transcription du manuel original MicroProse, copyright 1987][M]. Sections « Selecting a Nationality », « Trading with Merchants », « Travel by Sea », « Your Crew », « Your Career on the High Seas ». Consultée ; hébergement C64, correspondance CPC non certifiée. |
| CG | [CPC Game Reviews, Pirates!, page P/7][CG]. Critique propre au CPC ; recrutement, commerce, nationalités et liberté. Consultée. |
| CP | [CPC-POWER, fiche 1643][CP]. Identification CPC6128, crédits du port, choix de départ et objectif de retraite. Consultée. |
| CR | [CPCRULEZ, reproduction de l'article Amstrad Cent Pour Cent n°3, avril 1988][CR]. Texte consulté, récit romancé distingué de la critique. Notice liée inaccessible (403). |
| CW | [CPCWiki, Pirates!][CW]. Description CPC et distinction des versions ; extraits indexés consultés, accès direct à certains miroirs refusé (403). Confiance inférieure à une page intégralement lisible. |
| V | [FRGCB, comparaison des versions, 26 juin 2015][V]. Présentation des systèmes et différences entre portages ; aucune équivalence complète présumée. Consultée. |
| PDF | [GamesDatabase, PDF catalogué Amstrad CPC][PDF]. Recherche et extraits indexés uniquement ; ouverture 404. Source non utilisée pour établir les mécaniques CPC. |

[M]: https://www.antimon.org/dl/c64/misc/piratesdoc.html
[CG]: https://www.cpcgamereviews.com/p/7/
[CP]: https://www.cpc-power.com/index.php?num=1643&page=detail
[CR]: https://cpcrulez.fr/GamesTest/pirates.htm
[CW]: https://cpcwiki.eu/index.php/Pirates%21
[V]: https://frgcb.blogspot.com/2015/06/pirates-microprose-1987.html?m=0
[PDF]: https://www.gamesdatabase.org/Media/SYSTEM/Amstrad_CPC/manual/Formated/Sid_Meier-s_Pirates_-_1987_-_MicroProse_Ltd..pdf
