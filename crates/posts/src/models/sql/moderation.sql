CREATE TABLE moderateds (
    id          SERIAL PRIMARY KEY,
    description VARCHAR(500),
    verified    BOOLEAN NOT NULL DEFAULT false,
    status      SMALLINT NOT NULL,
    types       SMALLINT NOT NULL,
    object_id   INT NOT NULL,
    created     TIMESTAMP NOT NULL,
    count       INT NOT NULL
);

CREATE TABLE moderated_reports (
    id           SERIAL PRIMARY KEY,
    user_id      INT NOT NULL,
    moderated_id INT NOT NULL,
    description  VARCHAR(500),
    types        SMALLINT NOT NULL,
    created      TIMESTAMP NOT NULL
);
CREATE UNIQUE INDEX moderated_reports_unq ON moderated_reports (user_id, moderated_id);

CREATE TABLE moderated_penalties (
    id           SERIAL PRIMARY KEY,
    user_id      INT NOT NULL,
    moderated_id INT NOT NULL,
    expiration   TIMESTAMP,
    types        SMALLINT NOT NULL,
    object_id    INT NOT NULL,
    status       SMALLINT NOT NULL,
    created      TIMESTAMP NOT NULL
);
CREATE UNIQUE INDEX moderated_penalties_unq ON moderated_penalties (user_id, moderated_id);

CREATE TABLE moderated_logs (
    id              SERIAL PRIMARY KEY,
    user_id         INT NOT NULL,
    object_id       INT NOT NULL,
    action          SMALLINT NOT NULL,
    description     VARCHAR(500),
    types           SMALLINT NOT NULL,
    created         TIMESTAMP NOT NULL,
    time_to_suspend TIMESTAMP,

    CONSTRAINT fk_moderated_logs_manager
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);
CREATE INDEX moderated_logs_id_idx ON moderated_logs (user_id);

-- сервисы токенов и их разрешения. Работа с данными -------
CREATE TABLE owner_services (
    id    SERIAL PRIMARY KEY,   -- id
    types SMALLINT NOT NULL,    -- определитель сервиса и доступа
    name  VARCHAR(100) NOT NULL -- название сервиса
);
CREATE INDEX owner_serivices_index ON owner_services (types);

-- создадим варианты для токенов, чтобы сто раз не добавлять
INSERT INTO owner_services (id, types, name) 
VALUES (1, 4, 'Записи') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name) 
VALUES (2, 34, 'Управление записями') ON CONFLICT DO NOTHING;


-- ключи доступа / токены к записям ------- 
CREATE TABLE owners (
    id           SERIAL PRIMARY KEY,     -- id
    user_id      INT NOT NULL,           -- id создателя или владельца
    community_id INT,                    -- id сообщества-владельца (если есть)
    name         VARCHAR(100) NOT NULL,  -- название
    types        SMALLINT NOT NULL,      -- тип владельца: приложение, пользователь, сообщество
    secret_key   VARCHAR(200) NOT NULL,  -- секретный ключ
    service_key  VARCHAR(200) NOT NULL,  -- сервисный ключ
    is_active    BOOLEAN NOT NULL,       -- активно

    UNIQUE(service_key) 
);
CREATE INDEX item_service_key_index ON owners (service_key);

INSERT INTO owners (id, user_id, name, types, secret_key, service_key, is_active)
VALUES (1, 1, 'Браузерное приложение', 1, '%n%#Nv!|y9nU', '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', true ) ON CONFLICT DO NOTHING;

-- связь сервисов токенов с токенами -------
CREATE TABLE owner_services_items (
    id          SERIAL PRIMARY KEY, -- id
    owner_id    INT NOT NULL,       -- id токена-владельца
    service_id  INT NOT NULL        -- id сервиса
);
CREATE UNIQUE INDEX owner_services_items_unq ON owner_services_items (owner_id, service_id);

-- создадим варианты для токенов, чтобы сто раз не добавлять
INSERT INTO owner_services (id, types, name)
VALUES (1, 4, 'Записи') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (2, 34, 'Управление записями') ON CONFLICT DO NOTHING;