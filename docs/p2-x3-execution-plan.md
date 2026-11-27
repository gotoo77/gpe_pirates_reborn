# P2-X3 - plan d'exécution du pilote UX

Statut : PLAN-ONLY. Aucun code autorisé par ce fichier seul. Base de jeu : P2-X2 `6ea1fd1c83e37a99f7ff158a051d4deb200ea496` ; branche de travail `exp/p2-x3-ux-foundation` (commits documentaires ultérieurs à vérifier). Contrats : `pirates-art-ux-v02.md` et `gpe-development-method.md`. P2-X2 est le témoin, non le produit accepté. P2.0, laboratoire économique et infrastructure GPE-Agent #1382 hors scope.

## Décisions à obtenir avant code de production

- Accepter la DA adulte maritime, la suppression des notices permanentes, les destinations Carnet/Aide, le principe de contrôles réellement interactifs.
- Décider U0 (lot conventionnel de 100 kg, charge 3 t, caisse d'outils de 100 kg brut) ou garder les lots visibles ; aucune U1 dans cette phase.
- Disposer d'un échantillon artistique (typo, matière, icône, composition, provenance) accepté. Si absent, le plan identifie précisément ce qui manque ; pas de recherche ou de fabrication d'assets interminable, pas d'image générée.
- Confirmer les besoins du spike GPE et le budget d'une première tranche. Choisir l'ordre proposé ou le corriger sur preuve, ne pas élargir le scope.

## P0 - préflight et plan Codex (lecture seule)

Inspecter statut Git, SHA, fichiers de politique, `Cargo.toml`, `scripts/check.sh`, `src/app.rs`, `src/draw.rs`, `src/economy.rs`, `src/lib.rs`, tests ciblés et API du GPE verrouillé. Établir état actuel vs prévu, preuves disponibles et lacunes. Vérifier présence/licence des assets et fonts avant de choisir une technique. Comparer au plus deux options d'intégration des contrôles (UI GPE existante vs composition locale utilisant ses primitives) sur les besoins réels, pas sur élégance abstraite.

Sortie : plan concis des tranches, fichier(s) touchés, tests, risques, coût prévu, points d'arrêt et décisions nécessaires. Aucun build lourd pour ce lot documentaire, aucune modification, aucun commit. Ne pas répéter la revue de conception déjà livrée.

## S1 - spike de capacité (autorisation distincte)

Une scène minimale reproductible, isolée du gameplay : un titre et texte accentué mesurés, un élément graphique sous licence, un vrai bouton avec focus/souris/tactile/pad/clavier selon API disponibles, capture CPU/native et browser, temps de frame comparé à conditions identiques. Vérifier feature outline-fonts, lockfile, compatibilité et coût avant décision. Pas de changement de pin, d'économie ou de navigation. Réutiliser un exemple existant si suffisant. Un seul spike, deux essais correctifs ciblés max par blocage.

Sortie : tableau des capacités PROUVÉ / BLOQUÉ / NON TESTÉ, fichiers, mesures et décision de voie. Si le besoin impose un changement du moteur, HOLD + reproduction minimale dans un ticket séparé. Ne pas construire une UI de remplacement pour contourner.

## S2 - marché vertical (autorisation après S1 et échantillon DA)

Un écran marché réel à 960 x 540, composé à partir des données, avec illustrations et typographie acceptées, contrôles utilisables, quantité, total exact, solde, charge, erreurs et retour. Pas de carnet permanent ni de mode d'emploi permanent. Le carnet et l'aide ne sont accessibles que lorsque leur destination est implémentée dans S3 ; aucun faux bouton. Préserver la transaction atomique et les règles de gel. Appliquer U0 seulement si acceptée, avec tests d'équivalence de toutes les quantités/prix/capacité. Si non acceptée, lots explicites.

Fichiers possibles : `src/draw.rs`, module de présentation dédié seulement si la séparation est justifiée, `src/app.rs`, `src/lib.rs`, assets sous licence, tests et captures, manifeste/lock uniquement pour feature prouvée et approuvée. Économie inchangée pour U0 ; aucune U1. Pas de refonte navigation. Les prix unitaires sont marginaux, le total vient du devis. Aucun clic de sélection ne confirme une transaction.

Sortie : captures de l'achat/vente et cas invalides, tests de flux et de rendu, gate intégral, preuve de comportement en natif et Web si accessible. Revue visuelle propriétaire avant S3 ; en cas d'échec, une itération ciblée ou HOLD.

## S3 - aide et carnet (autorisation après S2)

Aide contextuelle unique et carnet dédié, cotations observées déjà disponibles, âge et inconnu, retour sans perte d'état ni action accidentelle. Navigation sûre et testable ; spécifier gel et reprise si carnet disponible en mer (première version peut rester au port). Pas de nouvelles règles de prix, de recommandations de route, d'informations inventées, de port ou cale complets si non nécessaires. Tests modal, retour, action unique, conservation focus et observation. Gate complet, capture et essai humain.

## S4 - acceptation et capitalisation

Propriétaire + si possible trois utilisateurs naïfs : trouver marché, achat/vente, retrouver prix, retour, sans explication imposée. Cibles proposées 10/30/15 s du contrat UX, à confronter au test. Recueillir ce qui bloque et envie de jouer. Relever les performances avec conditions et référence ; aucun objectif chiffré annoncé comme atteint sans mesure. Gate canonique `bash scripts/check.sh` + browser réel et essais matériels ; distinguer SKIP, NOT RUN, BLOCKED. Aucun merge avant accord.

Après acceptation : identifier les conventions et composants réellement réutilisables et publier un bilan de coût (si mesurable), nombre de corrections, blocages et preuves. Piloter la méthode dans un second jeu avant standardisation GPE ; pas de changement de GPE Core dans ce plan.

## Politique de reprise

Une tranche active à la fois, plan et SHA exacts. Pas de reset, stash ou écrasement des travaux du propriétaire. Avant la livraison, comparer diff et contrat ; si dérive, arrêter et signaler. Ne jamais affaiblir fmt, Clippy, tests ou CI. Si GPE-Agent est bloqué, ne pas modifier son infrastructure : demander un contrôle local/CI strictement équivalent, noter les gates non couvertes. Un PASS d'un ancien SHA ne vaut pas pour le nouveau. En fin de lot, enregistrer la prochaine action unique.

Première action autorisée par le prompt associé : P0 seulement. Le propriétaire peut alors autoriser S1 sur un plan approuvé sans réitérer les détails de ce contrat.
