-- MySQL 8+ / utf8mb4 / UUID as BINARY(16)

SET NAMES utf8mb4;
SET time_zone = '+00:00';

CREATE TABLE users (
                       user_id          BINARY(16) PRIMARY KEY,
                       email            VARCHAR(320) NOT NULL UNIQUE,
                       password_hash    VARCHAR(255) NOT NULL,
                       base_currency_code CHAR(3) NOT NULL DEFAULT 'EUR',
                       created_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                       updated_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE currencies (
                            code        CHAR(3) PRIMARY KEY,
                            name        VARCHAR(64) NOT NULL,
                            minor_unit  TINYINT UNSIGNED NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- Quelques devises (tu peux compléter)
INSERT INTO currencies(code, name, minor_unit) VALUES
                                                   ('EUR','Euro',2),
                                                   ('XAF','Central African CFA franc',0)
ON DUPLICATE KEY UPDATE name=VALUES(name), minor_unit=VALUES(minor_unit);

CREATE TABLE fx_rates (
                          fx_rate_id  BINARY(16) PRIMARY KEY,
                          base_code   CHAR(3) NOT NULL,   -- devise de base (ex: EUR)
                          quote_code  CHAR(3) NOT NULL,   -- devise cotée (ex: XAF)
                          rate        DECIMAL(24,12) NOT NULL, -- 1 base_code = rate quote_code
                          as_of_date  DATE NOT NULL,
                          source      VARCHAR(32) NOT NULL DEFAULT 'manual',
                          created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                          UNIQUE KEY uq_fx (base_code, quote_code, as_of_date),
                          CONSTRAINT fk_fx_base FOREIGN KEY (base_code) REFERENCES currencies(code),
                          CONSTRAINT fk_fx_quote FOREIGN KEY (quote_code) REFERENCES currencies(code)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE accounts (
                          account_id    BINARY(16) PRIMARY KEY,
                          user_id       BINARY(16) NOT NULL,
                          name          VARCHAR(80) NOT NULL,
                          account_type  ENUM('checking','savings','cash','broker','debt') NOT NULL,
                          currency_code CHAR(3) NOT NULL,
                          institution   VARCHAR(80) NULL,
                          archived      TINYINT(1) NOT NULL DEFAULT 0,
                          created_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                          updated_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                          KEY idx_accounts_user (user_id),
                          CONSTRAINT fk_accounts_user FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
                          CONSTRAINT fk_accounts_currency FOREIGN KEY (currency_code) REFERENCES currencies(code)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE categories (
                            category_id BINARY(16) PRIMARY KEY,
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
                            CONSTRAINT fk_categories_user FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
                            CONSTRAINT fk_categories_parent FOREIGN KEY (parent_id) REFERENCES categories(category_id) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE payees (
                        payee_id   BINARY(16) PRIMARY KEY,
                        user_id    BINARY(16) NOT NULL,
                        name       VARCHAR(120) NOT NULL,
                        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        UNIQUE KEY uq_payees_user_name (user_id, name),
                        KEY idx_payees_user (user_id),
                        CONSTRAINT fk_payees_user FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE goals (
                       goal_id          BINARY(16) PRIMARY KEY,
                       user_id          BINARY(16) NOT NULL,
                       name             VARCHAR(120) NOT NULL,
                       type             ENUM('savings','debt','investment','one_shot') NOT NULL,
                       target_base_minor BIGINT NOT NULL DEFAULT 0,
                       target_date      DATE NULL,
                       priority         INT NOT NULL DEFAULT 0,
                       linked_account_id BINARY(16) NULL,
                       created_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                       updated_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                       KEY idx_goals_user (user_id),
                       CONSTRAINT fk_goals_user FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
                       CONSTRAINT fk_goals_account FOREIGN KEY (linked_account_id) REFERENCES accounts(account_id) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE projects (
                          project_id       BINARY(16) PRIMARY KEY,
                          user_id          BINARY(16) NOT NULL,
                          name             VARCHAR(120) NOT NULL,
                          status           ENUM('planned','active','paused','done','cancelled') NOT NULL DEFAULT 'planned',
                          priority         INT NOT NULL DEFAULT 0,
                          start_date       DATE NULL,
                          due_date         DATE NULL,
                          budget_base_minor BIGINT NOT NULL DEFAULT 0,
                          goal_id          BINARY(16) NULL,
                          description      TEXT NULL,
                          created_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                          updated_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                          KEY idx_projects_user (user_id),
                          KEY idx_projects_goal (goal_id),
                          CONSTRAINT fk_projects_user FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
                          CONSTRAINT fk_projects_goal FOREIGN KEY (goal_id) REFERENCES goals(goal_id) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE budgets (
                         budget_id          BINARY(16) PRIMARY KEY,
                         user_id            BINARY(16) NOT NULL,
                         month              DATE NOT NULL,  -- toujours le 1er du mois
                         base_currency_code CHAR(3) NOT NULL,
                         status             ENUM('draft','active','closed') NOT NULL DEFAULT 'active',
                         created_at         TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                         updated_at         TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                         UNIQUE KEY uq_budgets_user_month (user_id, month),
                         KEY idx_budgets_user (user_id),
                         CONSTRAINT fk_budgets_user FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
                         CONSTRAINT fk_budgets_currency FOREIGN KEY (base_currency_code) REFERENCES currencies(code)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE budget_envelopes (
                                  budget_envelope_id  BINARY(16) PRIMARY KEY,
                                  budget_id           BINARY(16) NOT NULL,
                                  category_id         BINARY(16) NOT NULL,
                                  planned_base_minor  BIGINT NOT NULL DEFAULT 0,
                                  carryover_base_minor BIGINT NOT NULL DEFAULT 0,
                                  rollover_rule       ENUM('none','full','partial') NOT NULL DEFAULT 'full',
                                  created_at          TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                  updated_at          TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                                  UNIQUE KEY uq_env_budget_category (budget_id, category_id),
                                  KEY idx_env_budget (budget_id),
                                  CONSTRAINT fk_env_budget FOREIGN KEY (budget_id) REFERENCES budgets(budget_id) ON DELETE CASCADE,
                                  CONSTRAINT fk_env_category FOREIGN KEY (category_id) REFERENCES categories(category_id) ON DELETE RESTRICT
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE transactions (
                              transaction_id    BINARY(16) PRIMARY KEY,
                              user_id           BINARY(16) NOT NULL,
                              account_id        BINARY(16) NOT NULL,
                              occurred_at       DATETIME(3) NOT NULL,
                              amount_minor      BIGINT NOT NULL, -- signé: dépense négative, revenu positif
                              currency_code     CHAR(3) NOT NULL,
                              base_amount_minor BIGINT NOT NULL,
                              base_currency_code CHAR(3) NOT NULL,
                              fx_rate_id        BINARY(16) NULL,
                              category_id       BINARY(16) NULL,
                              payee_id          BINARY(16) NULL,
                              note              TEXT NULL,
                              project_id        BINARY(16) NULL,
                              goal_id           BINARY(16) NULL,
                              status            ENUM('pending','cleared') NOT NULL DEFAULT 'cleared',
                              created_at        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                              updated_at        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

                              KEY idx_tx_user_date (user_id, occurred_at),
                              KEY idx_tx_user_account_date (user_id, account_id, occurred_at),
                              KEY idx_tx_user_category_date (user_id, category_id, occurred_at),
                              KEY idx_tx_user_project_date (user_id, project_id, occurred_at),
                              KEY idx_tx_user_goal_date (user_id, goal_id, occurred_at),

                              CONSTRAINT fk_tx_user FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
                              CONSTRAINT fk_tx_account FOREIGN KEY (account_id) REFERENCES accounts(account_id) ON DELETE RESTRICT,
                              CONSTRAINT fk_tx_currency FOREIGN KEY (currency_code) REFERENCES currencies(code),
                              CONSTRAINT fk_tx_base_currency FOREIGN KEY (base_currency_code) REFERENCES currencies(code),
                              CONSTRAINT fk_tx_fx FOREIGN KEY (fx_rate_id) REFERENCES fx_rates(fx_rate_id) ON DELETE SET NULL,
                              CONSTRAINT fk_tx_category FOREIGN KEY (category_id) REFERENCES categories(category_id) ON DELETE SET NULL,
                              CONSTRAINT fk_tx_payee FOREIGN KEY (payee_id) REFERENCES payees(payee_id) ON DELETE SET NULL,
                              CONSTRAINT fk_tx_project FOREIGN KEY (project_id) REFERENCES projects(project_id) ON DELETE SET NULL,
                              CONSTRAINT fk_tx_goal FOREIGN KEY (goal_id) REFERENCES goals(goal_id) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
