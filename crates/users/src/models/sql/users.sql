-- пользователи -------

CREATE TABLE users (
    id            SERIAL PRIMARY KEY,
    first_name    VARCHAR(100) NOT NULL,
    last_name     VARCHAR(100) NOT NULL,
    phone         VARCHAR(14) NOT NULL,
    types         SMALLINT NOT NULL DEFAULT 1,
    is_man        BOOLEAN NOT NULL DEFAULT TRUE,
    password      VARCHAR(500) NOT NULL,
    link          VARCHAR(100) NOT NULL,
    s_avatar      VARCHAR(500),
    last_activity TIMESTAMP NOT NULL,

    UNIQUE(phone),
    UNIQUE(link)
);
--INSERT INTO users (id, first_name, last_name, phone, types, is_man, level, password, link, last_activity)
--VALUES (1, 'Сергей', 'Зубарев', '79042373637', 1, TRUE, 100, 'ulihos46', 'id1', current_timestamp)
--ON CONFLICT DO NOTHING;

-- информация пользователей -------
CREATE TABLE user_infos (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    avatar_id INT,
    language  VARCHAR(10) NOT NULL,
    email     VARCHAR(100),
    birthday  DATE,
    b_avatar  VARCHAR(500),
    status    VARCHAR(100),
    city      VARCHAR(100),
    level     SMALLINT NOT NULL DEFAULT 100,
    cover     VARCHAR(500),
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

-- Настройки приватности пользователя -------
-- 1 Все пользователи
-- 2 Все друзья и все подписчики
-- 3 Все друзья и подписчики, кроме
-- 4 Все друзья и некоторые подписчики
-- 5 Все подписчики и друзья, кроме
-- 6 Все подписчики и некоторые друзья
-- 7 Все друзья
-- 8 Все подписчики
-- 9 Друзья, кроме
-- 10 Некоторые друзья
-- 11 Подписчики, кроме
-- 12 Некоторые подписчики
-- 13 Только я
CREATE TABLE user_privates (
    id         SERIAL PRIMARY KEY,
    user_id    INT NOT NULL,
    see_all    SMALLINT NOT NULL, -- Для кого профиль открыт...
    see_info   SMALLINT NOT NULL, -- Кто видит информацию
    see_friend SMALLINT NOT NULL  -- Кто видит друзей
);
CREATE UNIQUE INDEX user_privates_unq ON user_privates (user_id, id);

------------------
------------------
-- Смайлы и стикеры

-- Популярные смайлы -------
CREATE TABLE user_populate_smiles (
    id       SERIAL PRIMARY KEY,
    user_id  INT NOT NULL,
    smile_id INT NOT NULL,
    count    INT NOT NULL DEFAULT 0,
    image    VARCHAR(500) NOT NULL
);
CREATE UNIQUE INDEX user_populate_smiles_unq ON user_populate_smiles (user_id, smile_id);

-- Популярные стикеры -------
CREATE TABLE user_populate_stickers (
    id         SERIAL PRIMARY KEY,
    user_id    INT NOT NULL,
    sticker_id INT NOT NULL,
    count      INT NOT NULL DEFAULT 0,
    image      VARCHAR(500) NOT NULL
);
CREATE UNIQUE INDEX user_populate_stickers_unq ON user_populate_stickers (user_id, sticker_id);


-- исключения/включения пользователей
-- 1 может видеть открытый профиль
-- 2 может видеть информацию
-- 3 может видеть друзей
-- 11 не может видеть открытый профиль
-- 12 не может видеть информацию
-- 13 не может видеть друзей

CREATE TABLE user_visible_perms (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    target_id INT NOT NULL,
    types     SMALLINT NOT NULL
);
CREATE UNIQUE INDEX user_visible_perms_unq ON user_visible_perms (user_id, target_id);

-- телефонные коды
CREATE TABLE phone_codes (
    id    SERIAL PRIMARY KEY,
    phone VARCHAR(14) NOT NULL,
    code  INT NOT NULL
);
-- телефоные подтвержденные
CREATE TABLE verified_phones (
    id    SERIAL PRIMARY KEY,
    phone VARCHAR(14) NOT NULL
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


-- ключи доступа / токены к записям -------
CREATE TABLE owners (
    id           SERIAL PRIMARY KEY,     -- id
    user_id      INT NOT NULL,           -- id создателя или владельца
    name         VARCHAR(100) NOT NULL,  -- название
    description  VARCHAR(500),           -- описание
    types        SMALLINT NOT NULL,      -- тип владельца: приложение, пользователь, сообщество
    secret_key   VARCHAR(200) NOT NULL,  -- секретный ключ
    service_key  VARCHAR(200) NOT NULL,  -- сервисный ключ
    is_active    BOOLEAN NOT NULL,       -- активно

    UNIQUE(service_key)
);
CREATE INDEX item_service_key_index ON owners (service_key);


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

CREATE TABLE user_notifications (
    id                   SERIAL PRIMARY KEY,
    user_id              INT NOT NULL,
    connection_request   BOOLEAN NOT NULL DEFAULT true,
    connection_confirmed BOOLEAN NOT NULL DEFAULT true,
    user_invite          BOOLEAN NOT NULL DEFAULT true,

    CONSTRAINT fk_user_notifications
         FOREIGN KEY(user_id)
             REFERENCES users(id)
);
CREATE UNIQUE INDEX user_notifications_unq ON user_notifications (user_id, id);
