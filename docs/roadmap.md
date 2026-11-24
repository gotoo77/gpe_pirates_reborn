# Roadmap indicative

Statut au 24 novembre 2026. P0 publié sur `main` (`b0ede15`) ; P1 présent dans la branche (`84e869e`) ; **P1.1 implémenté localement, GO WITH CONDITIONS**. P2–P5 restent **PROPOSÉS**. Chaque phase demande son propre cadrage et sa validation ; aucun calendrier ou engagement de livraison n'est fixé.

| Phase | Intention | Porte de sortie envisagée |
| --- | --- | --- |
| P0 — Investigation et cadrage | Inspecter dépôts, GPE et référence CPC ; livrer README, audit, analyse, spécification et cet horizon ; ajouter `up.sh` demandé et ses protections vérifiées. | Publié ; spécification P1 autorisée par la mission du propriétaire. |
| P1 — Maritime Navigation Sandbox | Carte fixe originale, quatre îles, trois ports, navire, caméra, terres solides, vent, HUD et contrôles clavier/manette. | Séquence Brise→Phare de 903 ticks (18,06 s), tests vers Anse et retour, rendu et entrées GPE. Voir [preuves et réserves P1](p1-validation.md), dont essais humains et manette réelle encore ouverts. |
| P1.1 — Maritime Visual & Gameplay Feel | Rendu 960×540, navire détaillé sur 16 caps, océan animé/sillage, côtes organiques partagées avec les collisions, villages portuaires, HUD hiérarchisé, carte M/West et updater Windows `up.ps1`. | [Preuves P1.1](p11-validation.md) : règles et traversées P1 préservées, comparaison 1280×720, captures et mesures. Restent les essais Windows natif/PowerShell et manette physique ; aucun P2. |
| P2 — Ports et commerce | Hypothèse : services et ressources peu nombreux ; prix et trajets créent des choix. | À définir après observation de P1 ; ne pas créer de système économique en P1. |
| P3 — Rencontres maritimes | Hypothèse : événements et décisions en mer ; cadrer séparément le besoin de combat. | À définir ; une rencontre ne présuppose pas un moteur de combat ou une IA générique. |
| P4 — Factions, réputation et événements | Hypothèse : conséquences locales des actes et relations compréhensibles. | À définir ; tester la lisibilité des conséquences avant de multiplier les factions. |
| P5 — Carrière et monde évolutif | Hypothèse : expéditions, évolution du capitaine, bilan et fin de carrière. | À définir ; progression, vieillissement et persistance ne sont pas décidés. |

P2 à P5 sont **des hypothèses de travail**. L'ordre lui-même pourra changer. Aucun modèle de commerce, de diplomatie, de combat ou de sauvegarde ne doit entrer dans P1 pour préparer ces phases.

## Conditions de passage P0 → P1

1. Validation explicite du propriétaire ; P0 s'arrête ici.
2. Choix d'une révision GPE immuable après relecture de l'[audit](gpe-capability-audit.md) : le checkout actif et `origin/main` local diffèrent. Proposition : `8cbceda0c9b0cc55ee5deb7d5e173676ffc1d4f1`, effectivement inspectée, sans prétendre qu'elle est la dernière distante.
3. Initialisation Rust uniquement dans P1 autorisée ; gates natifs, cible WASM, packaging et démarrage réel à établir avant développement de la navigation. Toute capacité indispensable absente entraîne un **hard stop**, sans modification de Core.
4. Mesures de lisibilité/performance à 640 × 360 et essais clavier/manette sur les deux cibles. La validation WASM reste requise : sa disponibilité dans GPE ne prouve pas celle de ce futur jeu.

Pas de push, merge, déploiement, installation ou modification de GPE Core autorisé par cette roadmap.

## Livraison locale P1

Le SHA GPE demandé est vérifié sur le distant et verrouillé avec Cargo.lock. Le préflight minimal natif et navigateur a précédé les règles de navigation. L'implémentation se limite à P1 ; `up.sh` est inchangé. Aucun push, merge ou démarrage de P2. L'acceptation complète reste conditionnée aux essais indiqués dans le rapport P1.
