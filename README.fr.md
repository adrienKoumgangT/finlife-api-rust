# FinLife

App qui gère finance + projets + planification.


## Vision produit

Les 4 piliers finance (obligatoires)
1. **Budget**: enveloppes mensuelles + budget par projet
2. **Suivi**: toutes les opérations (dépenses/entrées/transferts) + tags + pièces jointes
3. **Objectifs**: épargne, fonds d'urgence, achat (PC, voyage), dettes, investissement
4. **Revue**: hebdo & mensuelle avec alertes ("dérive", "abonnements", "catégorie explosive")

\+ Pilier "opératif" (projets & exécution)
- Projets personnels avec : planning, tâches, jalons, budget, dépenses idées, temps passé
- Planification: "ce mois-ci je finance quoi?", "quelles tâches débloquent quoi?"
- Vue "système de vie" : priorités + charge + cashflow


## Modules principaux (structure propre)

### A. Finance

- Transactions (manuel + import)
- Catégories / sous-catégories (et mapping automatique)
- Budgets (mensuels + enveloppes + règles)
- Comptes (courant, épargne, cash, broker, dette)
- Net worth (actifs/passifs), cashflow, prévision

### B. Objectifs & plans

- Objectifs (cicle, échéance, contribution mensuelle)
- Plans mensuels (ce que l'on décide au début de mois)
- Règles automatiques (ex: "le jour de paie -> virement objectif X")

### C. Investissements

- Portefeuilles, lignes, apports (buy/sell), performance
- Allocation cible
- **Important**: on reste dans le suivi & la planification, pas du "conseil financier"

### D. Projets

- Projet = objectif + tâches + budget + timeline
- Lier des transactions à un projet ("matériel dev", "cours", "voyage Belgique")
- Indicateurs : coût réel vs prévu, reste à financer, burn rate

### E. Revue & Coaching (le vrai moteur)

- Revue hebdo : contrôle dérives + "3 actions"
- Revue mensuelle : réallocation budgets + clôture + template
- Alertes : abonnements, dépenses anormales, catégories > seuil, objectif en retard


## UX

1. Dashboard (aujourd'hui)
   - Solde dispo, reste à dépenser par enveloppe
   - Progression objectifs
   - Projets en cours + prochain action
2. Transactions
   - liste, recherche, filtres, import, catégorisation rapide
3. Budgets
   - envloppes mensuelles, règles, "move money"
4. Objectifs
   - contributions, simulation "si je metsX/mois -> date"
5. Projets
   - Kanban / timeline, budget projet, dépenses liées
6. Revue
   - hebdo + mensuel (avec checklist + notes)


## Modèle de données (noyau propre)

Entités clés (minimum solide)
- **Account** (type: checking / savings / cash / broker / loan / debt)
- **Transaction**
  - date, amount, direction, category, account, payee, note
  - tags, attachments
  - **project_id** (optionnel) + **goal_id** (optionnel)
- **Category** (+ règles d'auto-catégorisation)
- **BudgetMonth**
- **BudgetEnveloppe** (category_id, planned, actual, rollower_rule)
- **Goal**
  - type: savings / debt / invest / one-shot
  - target_amount, target_date, current_amount
- **Project**
  - status, start/end, budget_target, priority
- **ProjectTask / Milestone**
- **InvestmentPortfolio / Position / Trade**
- **ReviewSession**
  - week/month, notes, actions, décisions

Le point clé : **Transaction** doit se rattache à **Catégorie + Projet + Objectif** (optionnellement).
C'est ça qui fusionne finance et "opératif".


## Calculs & Règles

- **Disponible réel** = solde - enveloppes "bloquées" - contributions prévues
- **Rollover** : enveloppe non dépensée se reporte (oui/non/partiel)
- **Détection abonnement** : paiement récurrent même montant / même marchand
- **Anomalie** : dépense > moyenne 3 mois (z-score simple ou règle fixe)
- **Prévision cashflow** : charges fixes + contributions + projets planifiés


### Règles automatiques

- **Abonnement** : détecter et catégoriser automatiquement les paiements récurrents
- **Dépenses anormales** : détecter et signaler les dépenses écartées de la normale
- **Budget automatique** : ajuster automatiquement les budgets en fonction des dépenses réelles
- **Projet automatique** : créer automatiquement des projets pour les dépenses importantes
- **Objectif automatique** : créer automatiquement des objectifs pour les dépenses importantes


### Règles manuelles

- **Catégorisation** : catégoriser les transactions manuellement
- **Budget manuel** : définir manuellement les budgets pour les catégories
- **Projet manuel** : créer manuellement des projets pour les dépenses importantes
- **Objectif manuel** : créer manuellement des objectifs pour les dépenses importantes


### Règles "intelligentes"

#### Contrôle dépenses
- "Reste à dépenser" par enveloppe (budget)
- Détection de dérive : catégorie > budget ou > moyenne 3 mois
- Détection abonnements : payments récurrents (même marchand / périodicité)

#### Épargne
- Objectif épargne + contribution mensuelle planifiée
- Virement "virtuel" planifié (même si on exécute manuellement au début)

#### Projets
- Projet avec budget et suivi réel auto via transactions project_id
- Jalon = checkpoint + date + "reste à faire"

#### Investissement (V1)
- Suivi des apports + valeur totale (snapshot mensuel)
- Allocation et trades détaillés en V2


## Architecture

- Front : React + TypeScript
- Backend : Rust en REST
- DB : MySQL
- Cache: Redis
- Auth: JWT + chiffrement des secrets
- Import: jobs async

### Architecture Rust (Axum/Tokio)

Stack
- API: Axum
- Runtime: Tokio
- DB: MySQL + SQLx (migrations + queries)
- Auth: Argon2 (password hash) + JWT (RS256 ou HS256)
- Types monnaie : i64 en "minor units" (centimes) + currency.minor_unit
- Observabilité: tracing + tower-http + opentelemetry
- Docs API: Swagger with OpenAPI
- Tests: unitaires + integration


## Roadmap de construction

### MVP (le coeur)

1. Comptes + Transactions + Catégories
2. Budgets mensuels + enveloppes + "reste à dépenser"
3. Objectifs (épargne) + virements planifiés (manuels au début)
4. Projets (liste + budget + lien transactions)
5. Revue mensuelle (simple)

### V2 (la magie)

- Règles d'auto-catégorisation
- Import bancaire + déduplication
- Alertes + revue hebdo guidée
- Investissements (positions et performance)
- Prévision cashflow + scénarios ("si je finance X...")



