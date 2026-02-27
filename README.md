# FinLife

App that manages finances + projects + planning.


## Product vision

The 4 finance pillars (required)
1. **Budget**: monthly envelopes + budget per project
2. **Tracking**: all transactions (expenses/income/transfers) + tags + attachments
3. **Goals**: savings, emergency fund, purchases (PC, travel), debt, investment
4. **Review**: weekly & monthly with alerts ("drift", "subscriptions", "exploding category")

\+ "Operational" pillar (projects & execution)
- Personal projects with: planning, tasks, milestones, budget, idea expenses, time spent
- Planning: "what am I funding this month?", "which tasks unblock what?"
- "Life system" view: priorities + workload + cashflow


## Main modules (clean structure)

### A. Finance

- Transactions (manual + import)
- Categories / subcategories (and automatic mapping)
- Budgets (monthly + envelopes + rules)
- Accounts (checking, savings, cash, broker, debt)
- Net worth (assets/liabilities), cashflow, forecast

### B. Goals & plans

- Goals (cycle, deadline, monthly contribution)
- Monthly plans (what is decided at the beginning of the month)
- Automatic rules (e.g. "payday -> transfer to goal X")

### C. Investments

- Portfolios, positions, contributions (buy/sell), performance
- Target allocation
- **Important**: stay focused on tracking & planning, not "financial advice"

### D. Projects

- Project = goal + tasks + budget + timeline
- Link transactions to a project ("dev hardware", "courses", "Belgium trip")
- Metrics: actual cost vs planned, remaining to fund, burn rate

### E. Review & Coaching (the real engine)

- Weekly review: control drift + "3 actions"
- Monthly review: reallocate budgets + closeout + template
- Alerts: subscriptions, unusual expenses, categories > threshold, late goal


## UX

1. Dashboard (today)
   - Available balance, remaining to spend by envelope
   - Goal progress
   - Active projects + next action
2. Transactions
   - list, search, filters, import, quick categorization
3. Budgets
   - monthly envelopes, rules, "move money"
4. Goals
   - contributions, simulation "if I put X/month -> date"
5. Projects
   - Kanban / timeline, project budget, linked expenses
6. Review
   - weekly + monthly (with checklist + notes)


## Data model (clean core)

Key entities (solid minimum)
- **Account** (type: checking / savings / cash / broker / loan / debt)
- **Transaction**
  - date, amount, direction, category, account, payee, note
  - tags, attachments
  - **project_id** (optional) + **goal_id** (optional)
- **Category** (+ auto-categorization rules)
- **BudgetMonth**
- **BudgetEnvelope** (category_id, planned, actual, rollover_rule)
- **Goal**
  - type: savings / debt / invest / one-shot
  - target_amount, target_date, current_amount
- **Project**
  - status, start/end, budget_target, priority
- **ProjectTask / Milestone**
- **InvestmentPortfolio / Position / Trade**
- **ReviewSession**
  - week/month, notes, actions, decisions

The key point: **Transaction** must be attachable to **Category + Project + Goal** (optionally).
That is what merges finance and "operational".


## Calculations & Rules

- **Real available** = balance - "locked" envelopes - planned contributions
- **Rollover**: unspent envelope amount carries over (yes/no/partial)
- **Subscription detection**: recurring payment with same amount / same merchant
- **Anomaly**: expense > 3-month average (simple z-score or fixed rule)
- **Cashflow forecast**: fixed charges + contributions + planned projects


### Automatic rules

- **Subscription**: detect and automatically categorize recurring payments
- **Unusual expenses**: detect and flag expenses that deviate from normal
- **Automatic budget**: automatically adjust budgets based on actual spending
- **Automatic project**: automatically create projects for large expenses
- **Automatic goal**: automatically create goals for large expenses


### Manual rules

- **Categorization**: categorize transactions manually
- **Manual budget**: manually define category budgets
- **Manual project**: manually create projects for large expenses
- **Manual goal**: manually create goals for large expenses


### "Smart" rules

#### Spending control
- "Remaining to spend" per envelope (budget)
- Drift detection: category > budget or > 3-month average
- Subscription detection: recurring payments (same merchant / periodicity)

#### Savings
- Savings goal + planned monthly contribution
- Planned "virtual" transfer (even if executed manually at the beginning)

#### Projects
- Project with budget and automatic actual tracking via `transactions.project_id`
- Milestone = checkpoint + date + "remaining work"

#### Investment (V1)
- Contribution tracking + total value (monthly snapshot)
- Allocation and detailed trades in V2


## Architecture

- Frontend: React + TypeScript
- Backend: Rust REST API
- DB: MySQL
- Cache: Redis
- Auth: JWT + secret encryption
- Import: async jobs

### Rust Architecture (Axum/Tokio)

Stack
- API: Axum
- Runtime: Tokio
- DB: MySQL + SQLx (migrations + queries)
- Auth: Argon2 (password hash) + JWT (RS256 or HS256)
- Money types: i64 in "minor units" (cents) + `currency.minor_unit`
- Observability: tracing + tower-http + opentelemetry
- API docs: Swagger with OpenAPI
- Tests: unit + integration


## Build roadmap

### MVP (the core)

1. Accounts + Transactions + Categories
2. Monthly budgets + envelopes + "remaining to spend"
3. Goals (savings) + planned transfers (manual at first)
4. Projects (list + budget + transaction links)
5. Monthly review (simple)

### V2 (the magic)

- Auto-categorization rules
- Bank import + deduplication
- Alerts + guided weekly review
- Investments (positions and performance)
- Cashflow forecast + scenarios ("if I fund X...")
