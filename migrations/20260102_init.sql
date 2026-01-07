-- MySQL 8+ / utf8mb4 / UUID as BINARY(16)

SET NAMES utf8mb4;
SET time_zone = '+00:00';

-- -----------------------------
-- USERS
-- -----------------------------
CREATE TABLE users (
    id                 BINARY(16) PRIMARY KEY,
    email              VARCHAR(320) NOT NULL UNIQUE,
    password_hash      VARCHAR(255) NOT NULL,
    role               ENUM('ADMIN','USER') NOT NULL DEFAULT 'USER',
    first_name         VARCHAR(255) NOT NULL,
    last_name          VARCHAR(255) NOT NULL,
    base_currency_code CHAR(3) NOT NULL DEFAULT 'EUR',
    created_at         TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at         TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- -----------------------------
-- CURRENCIES + FX
-- -----------------------------
CREATE TABLE currencies (
    code       CHAR(3) PRIMARY KEY,
    name       VARCHAR(64) NOT NULL,
    minor_unit TINYINT UNSIGNED NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

INSERT INTO currencies(code, name, minor_unit) VALUES
    ('EUR','Euro',2),
    ('XAF','Central African CFA franc',0),

    ('USD','US Dollar',2),
    ('CAD','Canadian Dollar',2),
    ('GBP','Pound Sterling',2),
    ('CHF','Swiss Franc',2),
    ('DKK','Danish Krone',2),

    -- "etc..." (monnaies courantes supplémentaires)
    ('NOK','Norwegian Krone',2),
    ('SEK','Swedish Krona',2),
    ('JPY','Japanese Yen',0),
    ('CNY','Chinese Yuan Renminbi',2),
    ('AUD','Australian Dollar',2),
    ('NZD','New Zealand Dollar',2)
ON DUPLICATE KEY UPDATE
     name = VALUES(name),
     minor_unit = VALUES(minor_unit);

CREATE TABLE fx_rates (
    id         BINARY(16) PRIMARY KEY,
    base_code  CHAR(3) NOT NULL,   -- devise de base (ex: EUR)
    quote_code CHAR(3) NOT NULL,   -- devise cotée (ex: XAF)
    rate       DECIMAL(24,12) NOT NULL, -- 1 base_code = rate quote_code
    as_of_date DATE NOT NULL,
    source     VARCHAR(32) NOT NULL DEFAULT 'manual',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    UNIQUE KEY uq_fx (base_code, quote_code, as_of_date),

    CONSTRAINT fk_fx_base  FOREIGN KEY (base_code)  REFERENCES currencies(code),
    CONSTRAINT fk_fx_quote FOREIGN KEY (quote_code) REFERENCES currencies(code)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- -----------------------------
-- PEOPLE (beneficiary/person)
-- -----------------------------
CREATE TABLE people (
    id         BINARY(16) PRIMARY KEY,
    user_id    BINARY(16) NOT NULL,
    name       VARCHAR(120) NOT NULL,

    email      VARCHAR(320) NULL,
    phone      VARCHAR(32) NULL,
    image_url  VARCHAR(512) NULL,
    note       TEXT NULL,

    archived   TINYINT(1) NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    UNIQUE KEY uq_people_user_name (user_id, name),
    KEY idx_people_user (user_id),

    CONSTRAINT fk_people_user
        FOREIGN KEY (user_id) REFERENCES users(id)
        ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- -----------------------------
-- LOCATIONS
-- -----------------------------
CREATE TABLE locations (
    id           BINARY(16) PRIMARY KEY,
    user_id      BINARY(16) NOT NULL,
    name         VARCHAR(120) NOT NULL,

    address      VARCHAR(255) NULL,
    city         VARCHAR(80) NULL,
    region       VARCHAR(80) NULL,
    postal_code  VARCHAR(24) NULL,
    country_code CHAR(2) NULL,          -- IT, FR, CM...

    latitude     DECIMAL(9,6) NULL,
    longitude    DECIMAL(9,6) NULL,

    archived     TINYINT(1) NOT NULL DEFAULT 0,
    created_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    UNIQUE KEY uq_locations_user_name (user_id, name),
    KEY idx_locations_user (user_id),

    CONSTRAINT fk_locations_user
        FOREIGN KEY (user_id) REFERENCES users(id)
        ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- -----------------------------
-- ACCOUNTS
-- -----------------------------
CREATE TABLE accounts (
    id            BINARY(16) PRIMARY KEY,
    user_id       BINARY(16) NOT NULL,
    name          VARCHAR(80) NOT NULL,
    account_type  ENUM('checking','savings','cash','broker','debt') NOT NULL,
    currency_code CHAR(3) NOT NULL,
    institution   VARCHAR(80) NULL,
    archived      TINYINT(1) NOT NULL DEFAULT 0,
    created_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    KEY idx_accounts_user (user_id),

    CONSTRAINT fk_accounts_user
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_accounts_currency
        FOREIGN KEY (currency_code) REFERENCES currencies(code)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- -----------------------------
-- CATEGORIES
-- -----------------------------
CREATE TABLE categories (
    id          BINARY(16) PRIMARY KEY,
    user_id     BINARY(16) NOT NULL,
    name        VARCHAR(80) NOT NULL,
    kind        ENUM('income','expense') NOT NULL,
    parent_id   BINARY(16) NULL,
    sort_order  INT NOT NULL DEFAULT 0,
    archived    TINYINT(1) NOT NULL DEFAULT 0,
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    UNIQUE KEY uq_categories_user_name_kind (user_id, name, kind),
    KEY idx_categories_user (user_id),
    KEY idx_categories_parent (parent_id),

    CONSTRAINT fk_categories_user
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_categories_parent
        FOREIGN KEY (parent_id) REFERENCES categories(id) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- -----------------------------
-- PAYEES (merchant/store/entity)
-- -----------------------------
CREATE TABLE payees (
    id         BINARY(16) PRIMARY KEY,
    user_id    BINARY(16) NOT NULL,
    name       VARCHAR(120) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    UNIQUE KEY uq_payees_user_name (user_id, name),
    KEY idx_payees_user (user_id),

    CONSTRAINT fk_payees_user
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- -----------------------------
-- GOALS
-- -----------------------------
CREATE TABLE goals (
    id                 BINARY(16) PRIMARY KEY,
    user_id            BINARY(16) NOT NULL,
    name               VARCHAR(120) NOT NULL,
    goal_type          ENUM('savings','debt','investment','one_shot') NOT NULL,
    target_base_minor  BIGINT NOT NULL DEFAULT 0,
    target_date        DATE NULL,
    priority           INT NOT NULL DEFAULT 0,
    linked_account_id  BINARY(16) NULL,
    created_at         TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at         TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    KEY idx_goals_user (user_id),

    CONSTRAINT fk_goals_user
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_goals_account
        FOREIGN KEY (linked_account_id) REFERENCES accounts(id) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- -----------------------------
-- PROJECTS (optional person + location)
-- -----------------------------
CREATE TABLE projects (
    id                BINARY(16) PRIMARY KEY,
    user_id           BINARY(16) NOT NULL,
    name              VARCHAR(120) NOT NULL,
    status            ENUM('planned','active','paused','done','cancelled') NOT NULL DEFAULT 'planned',
    priority          INT NOT NULL DEFAULT 0,
    start_date        DATE NULL,
    due_date          DATE NULL,
    budget_base_minor BIGINT NOT NULL DEFAULT 0,
    goal_id           BINARY(16) NULL,

    person_id         BINARY(16) NULL,
    location_id       BINARY(16) NULL,

    description       TEXT NULL,
    created_at        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    KEY idx_projects_user (user_id),
    KEY idx_projects_goal (goal_id),
    KEY idx_projects_user_person (user_id, person_id),
    KEY idx_projects_user_location (user_id, location_id),

    CONSTRAINT fk_projects_user
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_projects_goal
        FOREIGN KEY (goal_id) REFERENCES goals(id) ON DELETE SET NULL,
    CONSTRAINT fk_projects_person
        FOREIGN KEY (person_id) REFERENCES people(id) ON DELETE SET NULL,
    CONSTRAINT fk_projects_location
        FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- -----------------------------
-- PROJECT TASKS (tasks)
-- -----------------------------
CREATE TABLE project_tasks (
    id               BINARY(16) PRIMARY KEY,
    project_id       BINARY(16) NOT NULL,
    title            VARCHAR(200) NOT NULL,
    status           ENUM('todo','doing','done') NOT NULL DEFAULT 'todo',

    due_date         DATE NULL,
    parent_task_id   BINARY(16) NULL,
    order_idx        INT NOT NULL DEFAULT 0,

    estimate_minutes INT NULL,
    actual_minutes   INT NULL,

    assigned_person_id BINARY(16) NULL, -- optional (useful for "bénéficiaire" / assignation)
    location_id      BINARY(16) NULL,   -- optional

    note             TEXT NULL,
    created_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    KEY idx_tasks_project (project_id),
    KEY idx_tasks_project_status (project_id, status),
    KEY idx_tasks_due (due_date),
    KEY idx_tasks_parent (parent_task_id),

    CONSTRAINT fk_tasks_project
        FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    CONSTRAINT fk_tasks_parent
        FOREIGN KEY (parent_task_id) REFERENCES project_tasks(id) ON DELETE SET NULL,
    CONSTRAINT fk_tasks_person
        FOREIGN KEY (assigned_person_id) REFERENCES people(id) ON DELETE SET NULL,
    CONSTRAINT fk_tasks_location
        FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- -----------------------------
-- PROJECT MILESTONES (jalons)
-- -----------------------------
CREATE TABLE project_milestones (
    id          BINARY(16) PRIMARY KEY,
    project_id  BINARY(16) NOT NULL,
    title       VARCHAR(200) NOT NULL,
    due_date    DATE NULL,
    status      ENUM('planned','done','cancelled') NOT NULL DEFAULT 'planned',

    person_id   BINARY(16) NULL,  -- optional
    location_id BINARY(16) NULL,  -- optional

    note        TEXT NULL,
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    KEY idx_milestones_project (project_id),
    KEY idx_milestones_due (due_date),

    CONSTRAINT fk_milestones_project
        FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    CONSTRAINT fk_milestones_person
        FOREIGN KEY (person_id) REFERENCES people(id) ON DELETE SET NULL,
    CONSTRAINT fk_milestones_location
        FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- -----------------------------
-- BUDGETS (optional person association)
-- -----------------------------
CREATE TABLE budgets (
    id                 BINARY(16) PRIMARY KEY,
    user_id            BINARY(16) NOT NULL,
    month              DATE NOT NULL, -- always 1st day of month
    base_currency_code CHAR(3) NOT NULL,
    person_id          BINARY(16) NULL,
    status             ENUM('draft','active','closed') NOT NULL DEFAULT 'active',
    created_at         TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at         TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    UNIQUE KEY uq_budgets_user_month (user_id, month),
    KEY idx_budgets_user (user_id),
    KEY idx_budgets_user_person_month (user_id, person_id, month),

    CONSTRAINT fk_budgets_user
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_budgets_currency
        FOREIGN KEY (base_currency_code) REFERENCES currencies(code),
    CONSTRAINT fk_budgets_person
        FOREIGN KEY (person_id) REFERENCES people(id) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE budget_envelopes (
    id                   BINARY(16) PRIMARY KEY,
    budget_id            BINARY(16) NOT NULL,
    category_id          BINARY(16) NOT NULL,
    planned_base_minor   BIGINT NOT NULL DEFAULT 0,
    carryover_base_minor BIGINT NOT NULL DEFAULT 0,
    rollover_rule        ENUM('none','full','partial') NOT NULL DEFAULT 'full',
    created_at           TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at           TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    UNIQUE KEY uq_env_budget_category (budget_id, category_id),
    KEY idx_env_budget (budget_id),

    CONSTRAINT fk_env_budget
        FOREIGN KEY (budget_id) REFERENCES budgets(id) ON DELETE CASCADE,
    CONSTRAINT fk_env_category
        FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE RESTRICT
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- -----------------------------
-- TRANSACTIONS (optional person + location + payee)
-- -----------------------------
CREATE TABLE transactions (
    id                 BINARY(16) PRIMARY KEY,
    user_id            BINARY(16) NOT NULL,
    account_id         BINARY(16) NOT NULL,
    occurred_at        DATETIME(3) NOT NULL,

    amount_minor       BIGINT NOT NULL, -- signé: dépense négative, revenu positif
    currency_code      CHAR(3) NOT NULL,

    base_amount_minor  BIGINT NOT NULL,
    base_currency_code CHAR(3) NOT NULL,

    fx_rate_id         BINARY(16) NULL,

    category_id        BINARY(16) NULL,
    payee_id           BINARY(16) NULL,
    person_id          BINARY(16) NULL,
    location_id        BINARY(16) NULL,

    note               TEXT NULL,
    project_id         BINARY(16) NULL,
    goal_id            BINARY(16) NULL,

    status             ENUM('pending','cleared') NOT NULL DEFAULT 'cleared',

    created_at         TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at         TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    KEY idx_tx_user_date (user_id, occurred_at),
    KEY idx_tx_user_account_date (user_id, account_id, occurred_at),
    KEY idx_tx_user_category_date (user_id, category_id, occurred_at),
    KEY idx_tx_user_project_date (user_id, project_id, occurred_at),
    KEY idx_tx_user_goal_date (user_id, goal_id, occurred_at),
    KEY idx_tx_user_person_date (user_id, person_id, occurred_at),
    KEY idx_tx_user_location_date (user_id, location_id, occurred_at),

    CONSTRAINT fk_tx_user
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_tx_account
        FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE RESTRICT,

    CONSTRAINT fk_tx_currency
        FOREIGN KEY (currency_code) REFERENCES currencies(code),
    CONSTRAINT fk_tx_base_currency
        FOREIGN KEY (base_currency_code) REFERENCES currencies(code),

    CONSTRAINT fk_tx_fx
        FOREIGN KEY (fx_rate_id) REFERENCES fx_rates(id) ON DELETE SET NULL,

    CONSTRAINT fk_tx_category
        FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE SET NULL,
    CONSTRAINT fk_tx_payee
        FOREIGN KEY (payee_id) REFERENCES payees(id) ON DELETE SET NULL,
    CONSTRAINT fk_tx_person
        FOREIGN KEY (person_id) REFERENCES people(id) ON DELETE SET NULL,
    CONSTRAINT fk_tx_location
        FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE SET NULL,

    CONSTRAINT fk_tx_project
        FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE SET NULL,
    CONSTRAINT fk_tx_goal
        FOREIGN KEY (goal_id) REFERENCES goals(id) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;


-- -----------------------------
-- NOTIFICATIONS (type + deliveries + preferences)
-- -----------------------------
CREATE TABLE notification_types (
    id            BINARY(16) PRIMARY KEY,
    code          VARCHAR(64) NOT NULL UNIQUE,   -- ex: "BUDGET_ALERT", "SECURITY_PASSWORD_RESET"
    name          VARCHAR(120) NOT NULL,
    severity      ENUM('INFO','SUCCESS','WARNING','ERROR','SECURITY') NOT NULL DEFAULT 'INFO',

    -- Templates (optionnels) : tu peux rendre dynamiquement côté backend via {variables}
    title_template VARCHAR(200) NULL,
    body_template  TEXT NULL,

    default_in_app TINYINT(1) NOT NULL DEFAULT 1,
    default_email  TINYINT(1) NOT NULL DEFAULT 0,

    is_active     TINYINT(1) NOT NULL DEFAULT 1,
    created_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE notifications (
    id           BINARY(16) PRIMARY KEY,
    user_id      BINARY(16) NOT NULL,
    type_id      BINARY(16) NOT NULL,

    title        VARCHAR(200) NOT NULL,
    body         TEXT NULL,
    data         JSON NULL,              -- payload libre (montant, devise, ids, etc.)

    -- Optionnel : lien vers une entité
    entity_type  VARCHAR(40) NULL,       -- ex: "transaction", "project", "budget"
    entity_id    BINARY(16) NULL,
    link_url     VARCHAR(512) NULL,

    is_read      TINYINT(1) NOT NULL DEFAULT 0,
    read_at      DATETIME(3) NULL,
    archived     TINYINT(1) NOT NULL DEFAULT 0,

    created_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    KEY idx_notif_user_date (user_id, created_at),
    KEY idx_notif_user_read (user_id, is_read, created_at),
    KEY idx_notif_user_type (user_id, type_id, created_at),

    CONSTRAINT fk_notifications_user
       FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_notifications_type
       FOREIGN KEY (type_id) REFERENCES notification_types(id) ON DELETE RESTRICT
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE notification_deliveries (
     id              BINARY(16) PRIMARY KEY,
     notification_id BINARY(16) NOT NULL,
     channel         ENUM('IN_APP','EMAIL') NOT NULL,
     status          ENUM('PENDING','SENT','FAILED','SKIPPED') NOT NULL DEFAULT 'PENDING',

    -- si envoyé via email, on référence le mail outbox
     email_message_id BINARY(16) NULL,

     error           TEXT NULL,
     sent_at         DATETIME(3) NULL,
     created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

     UNIQUE KEY uq_delivery_notif_channel (notification_id, channel),
     KEY idx_delivery_status (status, created_at),
     KEY idx_delivery_email_message (email_message_id),

     CONSTRAINT fk_delivery_notification
         FOREIGN KEY (notification_id) REFERENCES notifications(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE notification_preferences (
    user_id  BINARY(16) NOT NULL,
    type_id  BINARY(16) NOT NULL,
    channel  ENUM('IN_APP','EMAIL') NOT NULL,
    enabled  TINYINT(1) NOT NULL DEFAULT 1,

    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    PRIMARY KEY (user_id, type_id, channel),

    CONSTRAINT fk_pref_user
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_pref_type
        FOREIGN KEY (type_id) REFERENCES notification_types(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;


-- -----------------------------
-- EMAILS (templates + outbox + events + tokens)
-- -----------------------------

CREATE TABLE email_templates (
    id            BINARY(16) PRIMARY KEY,
    code          VARCHAR(64) NOT NULL,          -- ex: "PASSWORD_RESET", "BUDGET_ALERT"
    locale        VARCHAR(10) NOT NULL DEFAULT 'fr', -- ex: fr, en, fr-FR
    subject_tpl   VARCHAR(200) NOT NULL,
    body_text_tpl MEDIUMTEXT NULL,
    body_html_tpl MEDIUMTEXT NULL,

    description   VARCHAR(255) NULL,
    variables     JSON NULL,                    -- doc des variables attendues (optionnel)
    is_active     TINYINT(1) NOT NULL DEFAULT 1,

    created_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    UNIQUE KEY uq_email_tpl_code_locale (code, locale)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE email_messages (
    id               BINARY(16) PRIMARY KEY,

    user_id           BINARY(16) NULL,           -- si mail lié à un user (reset, alertes)
    template_id       BINARY(16) NULL,

    to_email          VARCHAR(320) NOT NULL,
    to_name           VARCHAR(120) NULL,
    from_email        VARCHAR(320) NULL,
    from_name         VARCHAR(120) NULL,
    reply_to          VARCHAR(320) NULL,

    -- Rendering (si template) ou contenu direct
    template_vars     JSON NULL,
    subject           VARCHAR(200) NOT NULL,
    body_text         MEDIUMTEXT NULL,
    body_html         MEDIUMTEXT NULL,

    status            ENUM('QUEUED','SENDING','SENT','FAILED','CANCELLED') NOT NULL DEFAULT 'QUEUED',
    priority          ENUM('LOW','NORMAL','HIGH') NOT NULL DEFAULT 'NORMAL',

    provider          VARCHAR(40) NULL,          -- ex: "ses", "sendgrid", "smtp"
    provider_msg_id   VARCHAR(128) NULL,

    attempt_count     INT NOT NULL DEFAULT 0,
    max_attempts      INT NOT NULL DEFAULT 5,
    next_attempt_at   DATETIME(3) NULL,
    last_error        TEXT NULL,

    scheduled_at      DATETIME(3) NULL,
    sent_at           DATETIME(3) NULL,

    created_at        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    KEY idx_email_status (status, priority, created_at),
    KEY idx_email_user (user_id, created_at),
    KEY idx_email_next_attempt (next_attempt_at),
    KEY idx_email_provider_msg (provider_msg_id),

    CONSTRAINT fk_email_user
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL,
    CONSTRAINT fk_email_template
        FOREIGN KEY (template_id) REFERENCES email_templates(id) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE email_events (
    id              BINARY(16) PRIMARY KEY,
    email_message_id BINARY(16) NOT NULL,

    event_type      ENUM('QUEUED','SENDING','SENT','DELIVERED','BOUNCED','DROPPED','OPENED','CLICKED','FAILED') NOT NULL,
    event_at        DATETIME(3) NOT NULL,
    meta            JSON NULL,

    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    KEY idx_email_events_message (email_message_id, event_at),

    CONSTRAINT fk_email_events_message
        FOREIGN KEY (email_message_id) REFERENCES email_messages(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE password_reset_tokens (
    id          BINARY(16) PRIMARY KEY,
    user_id     BINARY(16) NOT NULL,

    token_hash  VARBINARY(32) NOT NULL,     -- SHA-256(token)
    expires_at  DATETIME(3) NOT NULL,
    used_at     DATETIME(3) NULL,

    request_ip  VARCHAR(45) NULL,           -- IPv4/IPv6
    user_agent  VARCHAR(255) NULL,

    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    UNIQUE KEY uq_pwd_reset_token_hash (token_hash),
    KEY idx_pwd_reset_user (user_id, created_at),
    KEY idx_pwd_reset_exp (expires_at),

    CONSTRAINT fk_pwd_reset_user
       FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE email_verification_tokens (
    id          BINARY(16) PRIMARY KEY,
    user_id     BINARY(16) NOT NULL,

    token_hash  VARBINARY(32) NOT NULL,     -- SHA-256(token)
    expires_at  DATETIME(3) NOT NULL,
    used_at     DATETIME(3) NULL,

    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    UNIQUE KEY uq_email_verify_token_hash (token_hash),
    KEY idx_email_verify_user (user_id, created_at),

    CONSTRAINT fk_email_verify_user
       FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

