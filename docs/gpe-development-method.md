# GPE Games - méthode de développement v0.1

Statut : contrat de travail proposé pour le pilote Pirates Reborn. Réutilisable dans les autres jeux GPE après retour d'expérience. Ce document n'autorise aucun changement de moteur, déploiement ou merge.

## 1. Objectif

Maximiser la qualité jouable acceptée par unité de crédit IA, de temps humain et de calcul local. Un écran agréable, un parcours fonctionnel et des preuves comptent davantage que la taille du diff. Ne pas sacrifier la validation pour économiser des crédits : éliminer plutôt les explorations redondantes, les modifications inutiles et les reprises de contexte.

## 2. Trois rôles complémentaires

| Rôle | Responsabilité | Limite |
|---|---|---|
| Propriétaire / ChatGPT | Intentions, DA, arbitrages, critères, revue ciblée, plans, orchestration GitHub/FGH | Ne pas présenter un concept ou une compilation comme un playtest réussi |
| Codex | Inspection ciblée, implémentation complexe, diagnostic à partir des preuves, tests locaux, rapport de livraison | Un incrément accepté à la fois ; pas de redesign libre, ni exploration de tous les projets |
| GPE-Agent + FGH | Workspace isolé, validations effectivement disponibles, preuves et continuation terminale | Pas de shell arbitraire, pas de droits supplémentaires, pas de PASS déduit d'une admission |

GitHub conserve les branches, décisions, diffs, commits et preuves. L'agent local ne remplace pas le connecteur GitHub. Utiliser scripts déterministes, tests, compilateurs et outils de capture avant un LLM pour les opérations mécaniques. Une IA locale éventuellement disponible peut traiter des tâches bornées et vérifiables (résumé de log, classement de findings), jamais être seule garante de la validation. Ne pas supposer son installation ou sa capacité.

## 3. Cycle de travail

1. **Qualifier le besoin.** Une intention joueur, un problème observable, une hypothèse, une mesure ou un critère d'acceptation. Définir aussi ce qui ne doit pas changer. Toute idée annexe va dans un backlog, pas dans la tâche courante.
2. **PRECHECK.** Lire AGENTS.md applicable, le contrat de phase et le dernier handoff. Vérifier branche, SHA, statut Git, pin GPE, gate canonique, artefacts et blocages connus. Inspecter seulement les fichiers et API nécessaires. Stop si une capacité essentielle est inconnue : produire un spike limité ou une preuve d'absence, jamais une architecture spéculative.
3. **PLAN-ONLY si non trivial.** Un nouveau flux, une migration, une évolution UI multi-écran ou un risque moteur demande un plan avant les modifications. Plan court : état observé, cible, options comparées si utiles, fichiers autorisés, invariants, étapes et preuves, budget et arrêts. Pas de code en PLAN-ONLY ; une correction locale évidente peut suivre directement les instructions d'une tâche déjà approuvée. La décision du propriétaire ou un contrat déjà validé autorise le passage en EXECUTE ; aucune validation humaine répétée à chaque étape mécanique.
4. **EXECUTE.** Une tranche verticale cohérente et réversible. Réutiliser l'existant ; préserver contrats, dépendances, API et tests non concernés. Ne pas déduire une permission de modifier un fichier externe parce que le build bloque.
5. **VERIFY.** Après chaque petite modification, checks ciblés et fmt/clippy pertinents ; avant de livrer le lot, gate canonique complet au SHA exact. Les checks ciblés économisent des répétitions mais ne remplacent jamais les gates obligatoires. Préserver caches et worktree, éviter les builds concurrents coûteux sur la même machine. GPE-Agent pour les profils réellement supportés, local/CI pour les autres. Un échec d'infrastructure est distinct d'un échec de code. Captures et playtest pour l'UX.
6. **REVIEW + HANDOFF.** Comparer le diff au plan, consigner SHA, preuves, écarts, statut des gates, captures, blocages et prochaine action unique. Une revue adversariale indépendante est utile aux invariants risqués, aux migrations et aux changements de contrat ; pas obligatoire pour un rustfmt. Si le test humain échoue, diagnostiquer pourquoi avant d'empiler du code.

## 4. Politique de crédit et de calcul

- Dépenser le raisonnement premium sur des décisions difficiles, pas sur git, rustfmt, formatage de rapport, surveillance ou copie de résultats.
- Le mode plan n'est pas gratuit. Utiliser ChatGPT pour préparer un contrat clair, puis Codex pour une inspection locale ciblée. Éviter que deux agents redécouvrent intégralement la même architecture.
- Réglage par défaut proposé : une session Codex d'implémentation par tranche autorisée, au plus deux tentatives de correction ciblées pour un même blocage reproductible avant diagnostic et arrêt. Une nouvelle preuve discriminante peut justifier une nouvelle tâche explicitement cadrée. Ne pas confondre borne de tentatives et plafond de tokens : aucun compteur exact ne doit être inventé.
- Choisir un effort de raisonnement proportionné : élevé pour architecture, invariants ou problème inconnu ; modéré pour exécution d'un plan clair ; faible pour tâches simples lorsque disponible. Éviter les fanouts multi-agents automatiques ; revue indépendante seulement si la valeur attendue justifie le coût.
- Ne pas lire tout le dépôt par défaut, tout réécrire, scanner les dépendances sans indice, lancer tous les builds pour un changement documentaire, répéter un échec identique, ni réparer la CI par suppression de tests.
- Travailler sur un worktree propre au SHA connu ; ne pas faire cargo clean sans preuve de nécessité. Ne pas toucher les checkouts du propriétaire. Pas de tâche parallèle coûteuse sans capacité attestée.
- Si l'outil expose les crédits, conserver consommation observée avant/après le lot. Sinon écrire « non mesurée ». Mesurer aussi reprises humaines, temps de calcul, nombre de cycles de correction, résultat accepté/refusé. Optimiser sur les tranches suivantes à partir de ces données, non d'une promesse d'économie.
- Fin de session : rapport de 15 lignes environ, lien vers la preuve longue si nécessaire. Ne pas recopier tous les logs. Ne pas prolonger la session pour produire des résumés décoratifs.

## 5. Frontière jeu / moteur

Le jeu possède ses assets, règles, navigation d'écrans, textes, DA et états métier. Le moteur possède les capacités génériques effectivement nécessaires. Vérifier d'abord le pin exact, les features et les exemples. Un problème récurrent dans plusieurs usages réels peut justifier une extraction ; un blocage indispensable même dans un seul usage peut justifier une correction minimale du moteur, via chantier séparé. Pas de nouveau framework, ECS, DSL, système de layout ou asset manager universel par anticipation. Ne jamais contourner un blocage de GPE par changement implicite d'architecture, de pin, de règles de sécurité ou de test.

## 6. Qualité d'un jeu

Contrat propre à chaque jeu : intention, DA, ergonomie, contrôles et limites. Pour un écran, prouver : lecture sans manuel permanent, action évidente, focus cohérent, souris/tactile/pad/clavier selon cibles, données correctes, erreurs locales, rendu sans clipping, retours sûrs. Les captures d'un état heureux ne suffisent pas. Le playtest propriétaire ou utilisateur naïf est une gate produit distincte des gates techniques. Ne pas forcer la même DA ou la même composition sur tous les jeux.

## 7. Tickets et handoff

Ticket minimal (un seul objectif) :

- ID / projet / base SHA / branche / contrat de référence
- problème observable et résultat attendu
- IN / OUT / fichiers et invariants
- étapes, tests et critères d'arrêt
- coût prévu en tranches et niveau de risque

Handoff minimal :

- base SHA -> head SHA, état du worktree
- fait / non fait / écart justifié
- gates PASS, FAIL, BLOCKED, SKIPPED ou NOT RUN avec preuves
- résultat UX distinct, captures et test humain restant
- consommation observée ou non mesurée, tentatives et blocages
- prochaine action unique, sans relancer l'exploration

Un résultat terminal FGH doit correspondre au job, workspace, révision et opération attendus ; une admission HTTP 202 ne constitue pas une réussite. Inspecter le protocole déployé avant d'émettre une requête. Pas de jobs répétés pour observer un état existant.

## 8. Déploiement progressif de la méthode

Pilote Pirates, puis un deuxième jeu aux contraintes différentes. Conserver une version de référence commune et des AGENTS.md courts propres à chaque dépôt ; ne pas dupliquer aveuglément l'ensemble des règles dans chaque prompt. Après le pilote, revoir les métriques et formaliser l'installation locale de la méthode dans le cadre existant de Codex, sans écraser les instructions globales ou locales. Ne pas industrialiser tant que deux usages ne démontrent pas le gain.

Références Codex (documentation publique, à vérifier si interface/version change) : https://developers.openai.com/blog/run-long-horizon-tasks-with-codex ; https://developers.openai.com/cookbook/examples/codex/iterating-development-workflows-with-codex. Le mode plan natif facilite la séparation, mais le contrat PLAN-ONLY reste applicable indépendamment du client.
