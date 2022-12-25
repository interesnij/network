-- пользователи -------

/*
1 стандартный тип пользователя
2 стандартный пославший запрос на идентификацию
3 стандартный идентифицированный
4 детский тип пользователя
5 детский пославший запрос на идентификацию
6 детский идентифицированный

10 TRAINEE_MODERATOR
11 MODERATOR
12 HIGH_MODERATOR
13 TEAMLEAD_MODERATOR
14 TRAINEE_MANAGER
15 MANAGER
16 HIGH_MANAGER
17 TEAMLEAD_MANAGER
18 ADVERTISER
19 HIGH_ADVERTISER
20 TEAMLEAD_ADVERTISER
21 ADMINISTRATOR
22 HIGH_ADMINISTRATOR
23 TEAMLEAD_ADMINISTRATOR
25 SUPERMANAGER

31 удаленный стандартный
32 удаленный пославший запрос на идентификацию
33 удаленный идентифицированный
34 удаленный ребенок
35 удаленный ребенок пославший запрос на идентификацию
36 удаленный ребенок идентифицированный

41 закрытый стандартный
42 закрытый пославший запрос на идентификацию
43 закрытый идентифицированный
44 закрытый ребенок
45 закрытый ребенок пославший запрос на идентификацию
46 закрытый ребенок идентифицированный

51 приостановленный стандартный
52 приостановленный пославший запрос на идентификацию
53 приостановленный идентифицированный
54 приостановленный ребенок
55 приостановленный ребенок пославший запрос на идентификацию
56 приостановленный ребенок идентифицированный

61 закрытый баннером стандартный
62 закрытый баннером пославший запрос на идентификацию
63 закрытый баннером идентифицированный
64 приостановленный ребенок
65 приостановленный ребенок пославший запрос на идентификацию
66 приостановленный ребенок идентифицированный
*/

CREATE TABLE users (
    id            SERIAL PRIMARY KEY,
    first_name    VARCHAR(100) NOT NULL,
    last_name     VARCHAR(100) NOT NULL,
    phone         VARCHAR(14) NOT NULL,
    types         SMALLINT NOT NULL DEFAULT 1,
    is_man        BOOLEAN NOT NULL DEFAULT TRUE,
    password      VARCHAR(500) NOT NULL,
    link          VARCHAR(100) NOT NULL,
    s_avatar      VARCHAR(100),
    last_activity TIMESTAMP NOT NULL,

    UNIQUE(phone),
    UNIQUE(link)
);

-- информация пользователей -------
CREATE TABLE user_infos (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    avatar_id INT,
    language  VARCHAR(10) NOT NULL,
    email     VARCHAR(100),
    birthday  DATE,
    b_avatar  VARCHAR(100),
    status    VARCHAR(100),
    city      VARCHAR(100),
    level     SMALLINT NOT NULL DEFAULT 100,
    cover     VARCHAR(100),
    created   TIMESTAMP NOT NULL,
    friends   INT NOT NULL,
    follows   INT NOT NULL
);
CREATE UNIQUE INDEX user_infos_unq ON user_infos (user_id, id);

-- местоположения пользователей -------
CREATE TABLE user_locations (
    id         SERIAL PRIMARY KEY,
    user_id    INT NOT NULL,
    city_ru    VARCHAR(100),
    city_en    VARCHAR(100),
    region_ru  VARCHAR(100),
    region_en  VARCHAR(100),
    country_ru VARCHAR(100),
    country_en VARCHAR(100)
);
CREATE INDEX user_locations_user_idx ON user_locations (user_id);

-- айпи пользователей -------
CREATE TABLE ip_users (
    id      SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    ip      VARCHAR NOT NULL
);
CREATE INDEX ip_users_user_idx ON ip_users (user_id);


-- Причина удаления аккаунта -------
CREATE TABLE user_delete_anketas (
    id      SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    answer  SMALLINT NOT NULL,
    other   VARCHAR(200),
    created TIMESTAMP NOT NULL
);
CREATE INDEX user_delete_anketas_idx ON user_delete_anketas (user_id);

-- Статус отношений -------
CREATE TABLE user_love_statuss (
    id            SERIAL PRIMARY KEY,
    user_id       INT NOT NULL,
    male_status   SMALLINT NOT NULL,
    female_status SMALLINT NOT NULL
);
CREATE UNIQUE INDEX user_love_statuss_unq ON user_love_statuss (user_id, id);


-- Муж/Жена -------
CREATE TABLE user_partner_ones (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    target_id INT NOT NULL
);
CREATE UNIQUE INDEX user_partner_ones_unq ON user_partner_ones (user_id, target_id);

-- Мама -------
CREATE TABLE user_mom_ones (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    target_id INT NOT NULL
);
CREATE UNIQUE INDEX user_mom_ones_unq ON user_mom_ones (user_id, target_id);

-- Папа -------
CREATE TABLE user_dad_ones (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    target_id INT NOT NULL
);
CREATE UNIQUE INDEX user_dad_ones_unq ON user_dad_ones (user_id, target_id);

-- Братья, сёстры -------
CREATE TABLE user_brother_sisters (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    target_id INT NOT NULL
);
CREATE UNIQUE INDEX user_brother_sisters_ones_unq ON user_brother_sisters (user_id, target_id);

-- Дети -------
CREATE TABLE user_children_ones (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    target_id INT NOT NULL
);
CREATE UNIQUE INDEX user_children_ones_unq ON user_children_ones (id, target_id);

-- Внуки -------
CREATE TABLE user_grandsons_ones (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    target_id INT NOT NULL
);
CREATE UNIQUE INDEX user_grandsons_ones_unq ON user_grandsons_ones (user_id, target_id);

-- Коллеги -------
CREATE TABLE user_colleagues_ones (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    target_id INT NOT NULL
);
CREATE UNIQUE INDEX user_colleagues_ones_unq ON user_colleagues_ones (user_id, target_id);

-- Черный список -------
CREATE TABLE user_blocks (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    target_id INT NOT NULL
);
CREATE UNIQUE INDEX user_blocks_unq ON user_blocks (user_id, target_id);

------------------
------------------
-- Список ключей новостей, уведомлений или рекомендаций (если пользователь хочет их группировать) -------
CREATE TABLE list_user_communities_keys (
    id    SERIAL PRIMARY KEY,
    types SMALLINT NOT NULL,     -- тип списка: 0 - неактивен, 1 - основной, 2 - пользовательский
    name  VARCHAR(100) NOT NULL, -- название
    owner INT NOT NULL           -- владелец
);

/*
Настройки приватности пользователя -------
1 Все пользователи
2 Все друзья и все подписчики
3 Все друзья и подписчики, кроме
4 Все друзья и некоторые подписчики
5 Все подписчики и друзья, кроме
6 Все подписчики и некоторые друзья
7 Все друзья
8 Все подписчики
9 Друзья, кроме
10 Некоторые друзья
11 Подписчики, кроме
12 Некоторые подписчики
13 Только я
*/
CREATE TABLE user_privates (
    id         SERIAL PRIMARY KEY,
    user_id    INT NOT NULL,
    see_all    SMALLINT NOT NULL, -- Для кого профиль открыт...
    see_info   SMALLINT NOT NULL, -- Кто видит информацию
    see_friend SMALLINT NOT NULL  -- Кто видит друзей
);
CREATE UNIQUE INDEX user_privates_unq ON user_privates (user_id, id);

/*
исключения/включения пользователей
1 может видеть открытый профиль
2 может видеть информацию
3 может видеть друзей
11 не может видеть открытый профиль
12 не может видеть информацию
13 не может видеть друзей
*/
CREATE TABLE user_visible_perms (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    target_id INT NOT NULL,
    types     SMALLINT NOT NULL
);
CREATE UNIQUE INDEX user_visible_perms_unq ON user_visible_perms (user_id, target_id);

-- телефонные коды
CREATE TABLE phone_codes ( 
    id      SERIAL PRIMARY KEY,
    phone   VARCHAR(14) NOT NULL,
    code    INT NOT NULL,
    types   SMALLINT NOT NULL,
    accept  BOOLEAN NOT NULL DEFAULT FALSE,
    created TIMESTAMP NOT NULL
);

-- друзья -------
CREATE TABLE friends (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    target_id INT NOT NULL,
    visited   INT NOT NULL
);
CREATE UNIQUE INDEX friends_user_target_unq ON friends (user_id, target_id);

-- подписчики -------
CREATE TABLE follows (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    target_id INT NOT NULL,
    view      BOOLEAN NOT NULL DEFAULT false,
    visited   INT NOT NULL
);
CREATE UNIQUE INDEX follows_user_followed_unq ON follows (user_id, target_id);

-- рекомендованные друзья -------
CREATE TABLE featured_friends (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    target_id INT NOT NULL,
    hidden    BOOLEAN NOT NULL DEFAULT FALSE
);
CREATE UNIQUE INDEX featured_friends_unq ON featured_friends (user_id, target_id);


CREATE TABLE moderateds (
    id          SERIAL PRIMARY KEY,
    description VARCHAR(500),
    verified    BOOLEAN NOT NULL DEFAULT false,
    status      SMALLINT NOT NULL,
    target_id   INT NOT NULL,
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
    target_id    INT NOT NULL,
    status       SMALLINT NOT NULL,
    created      TIMESTAMP NOT NULL
);
CREATE UNIQUE INDEX moderated_penalties_unq ON moderated_penalties (user_id, moderated_id);

CREATE TABLE moderated_logs (
    id              SERIAL PRIMARY KEY,
    user_id         INT NOT NULL,
    target_id       INT NOT NULL,
    action          SMALLINT NOT NULL,
    description     VARCHAR(500),
    created         TIMESTAMP NOT NULL,
    time_to_suspend TIMESTAMP,

    CONSTRAINT fk_moderated_logs_manager
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);
CREATE INDEX moderated_logs_id_idx ON moderated_logs (user_id);

CREATE TABLE user_notifications (
    id                   SERIAL PRIMARY KEY,
    user_id              INT NOT NULL, 
    connection_request   BOOLEAN NOT NULL DEFAULT true,
    connection_confirmed BOOLEAN NOT NULL DEFAULT true,

    CONSTRAINT fk_user_notifications
         FOREIGN KEY(user_id)
             REFERENCES users(id)
);
CREATE UNIQUE INDEX user_notifications_unq ON user_notifications (user_id, id);



-- сервисы токенов и их разрешения. Работа с данными -------
-- только для владельцев токенов
CREATE TABLE owner_services (
    id    SERIAL PRIMARY KEY,   -- id
    types SMALLINT NOT NULL,    -- определитель сервиса и доступа
    name  VARCHAR(100) NOT NULL -- название сервиса
);
CREATE INDEX owner_serivices_index ON owner_services (types);

-- создадим варианты для токенов, чтобы сто раз не добавлять
INSERT INTO owner_services (id, types, name)
VALUES (1, 1, 'Профиль') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (2, 2, 'Сайты') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (3, 3, 'Почта') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (4, 4, 'Записи') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (5, 5, 'Аудиозаписи') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (6, 6, 'Документы') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (7, 7, 'Опросы') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (8, 8, 'Фотографии') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (9, 9, 'Видиозаписи') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (10, 10, 'Товары') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (11, 11, 'Обсуждения') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (12, 12, 'Википедия') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (13, 13, 'Статьи') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (14, 14, 'Сообщения') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (15, 15, 'Планировщик') ON CONFLICT DO NOTHING;

-- дальше только для владельцев токена - работа с управлением

INSERT INTO owner_services (id, types, name)
VALUES (16, 31, 'Управление профилем') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (17, 32, 'Управление сайтами') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (18, 33, 'Управление почтой') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (19, 34, 'Управление записями') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (20, 35, 'Управление аудиозаписями') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (21, 36, 'Управление документами') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (22, 37, 'Управление опросами') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (23, 38, 'Управление фотографиями') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (24, 39, 'Управление видиозаписями') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (25, 40, 'Управление товарами') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (26, 41, 'Управление обсуждениями') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (27, 42, 'Управление википедией') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (28, 43, 'Управление статьями') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (29, 44, 'Управление сообщениями') ON CONFLICT DO NOTHING;
INSERT INTO owner_services (id, types, name)
VALUES (30, 45, 'Управление планировщиком') ON CONFLICT DO NOTHING;

-- ключи доступа / токены пользователей или приложений -------
CREATE TABLE owners ( 
    id           SERIAL PRIMARY KEY,     -- id
    user_id      INT NOT NULL,           -- id создателя или владельца
    name         VARCHAR(100) NOT NULL,  -- название
    description  VARCHAR(500),           -- описание
    types        SMALLINT NOT NULL,      -- тип владельца: приложение, пользователь
    secret_key   VARCHAR(200) NOT NULL,  -- секретный ключ
    service_key  VARCHAR(200) NOT NULL,  -- сервисный ключ
    is_active    BOOLEAN NOT NULL,       -- активно

    UNIQUE(service_key)
);
CREATE INDEX item_service_key_index ON owners (service_key);
-- создадим токен нашего браузерного приложения, чтобы сто раз не добавлять
INSERT INTO owners (id, user_id, name, description, types, secret_key, service_key, is_active)
VALUES (1, 1, 'Браузерное приложение', 'general app', 1, '%n%#Nv!|y9nU', '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', true ) ON CONFLICT DO NOTHING;


-- связь сервисов токенов с токенами -------
CREATE TABLE owner_services_items (
    id         SERIAL PRIMARY KEY, -- id
    owner_id   INT NOT NULL,       -- id токена-владельца
    service_id INT NOT NULL        -- id токена-сервиса
);
CREATE UNIQUE INDEX owner_services_items_unq ON owner_services_items (owner_id, service_id);

-- создадим сервисные разрешения для браузерного приложения
-- создадим варианты для токенов, чтобы сто раз не добавлять
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (1, 0, 1) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (2, 0, 2) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (3, 0, 3) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (4, 0, 4) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (5, 0, 5) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (6, 0, 6) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (7, 0, 7) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (8, 0, 8) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (9, 0, 9) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (10, 0, 10) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (11, 0, 11) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (12, 0, 12) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (13, 0, 13) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (14, 0, 14) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (15, 0, 15) ON CONFLICT DO NOTHING;

-- дальше только для владельцев токена - работа с управлением

INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (16, 0, 31) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (17, 0, 32) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (18, 0, 33) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (19, 0, 34) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (20, 0, 35) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (21, 0, 36) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (22, 0, 37) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (23, 0, 38) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (24, 0, 39) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (25, 0, 40) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (26, 0, 41) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (27, 0, 42) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (28, 0, 43) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (29, 0, 44) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (30, 0, 45) ON CONFLICT DO NOTHING;

