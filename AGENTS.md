# AGENTS.md - Pirates Reborn

Ce dépôt est le pilote de la méthode GPE Games. Appliquer aussi les instructions globales et de sous-répertoires applicables ; ne pas écraser de politique de sécurité ou de validation existante.

## Avant d'agir

- Lire `docs/gpe-development-method.md` pour les règles de travail communes. Lire le contrat et le plan de la phase demandée, pas tous les documents du dépôt.
- Pour P2-X3 : `docs/pirates-art-ux-v02.md` et `docs/p2-x3-execution-plan.md`. P0 est en PLAN-ONLY tant qu'une tranche d'implémentation n'est pas explicitement autorisée.
- Vérifier le statut Git, branche, SHA, pin GPE et les fichiers réellement concernés. Préserver le travail du propriétaire.
- Si la tâche dépasse une correction locale triviale, établir un plan borné avant le code. Ne pas relire tous les jeux ni lancer des agents ou recherches redondants.
- Une tranche à la fois ; aucune extension de scope, migration moteur, nouvelle dépendance non justifiée ni changement des règles économiques implicite. P2.0 et le laboratoire restent gelés.

## Validation

- Rust : fmt et Clippy strict obligatoires ; tests ciblés pendant le développement puis `bash scripts/check.sh` complet sur le candidat avant livraison. Ne jamais désactiver avertissements, tests ou CI. Natif et WASM selon le gate ; vérifier le navigateur et les entrées réelles pour un GO produit.
- Respecter les capacités effectives de GPE-Agent et FGH. Attendre la preuve terminale au SHA exact, ne pas relancer un job pour l'observer. Si blocage infrastructure, arrêter cette voie et consigner ; ne pas réparer l'infrastructure dans un chantier de jeu.
- Pour l'UX, aucune fausse interaction, aucune notice permanente, aucune maquette aplatie. Les tests verts ne prouvent pas la DA ni le plaisir de jeu.
- Deux corrections ciblées max d'un même blocage sans nouvelle décision ; ne pas gaspiller de crédits sur des essais aveugles. Rapport bref : SHA, diff, gates et preuves, limites, coût observable ou non mesuré, prochaine action.
- Aucun merge ou déploiement sans accord explicite. Ne pas présenter un objectif ou une proposition comme déjà implémenté.
