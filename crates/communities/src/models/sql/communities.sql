-- пользователи -------
CREATE TABLE users (
    id            SERIAL PRIMARY KEY,
    user_id       INT NOT NULL,
    first_name    VARCHAR(100) NOT NULL,
    last_name     VARCHAR(100) NOT NULL,
    types         SMALLINT NOT NULL DEFAULT 1,
    is_man        BOOLEAN NOT NULL DEFAULT TRUE,
    link          VARCHAR(100) NOT NULL,
    s_avatar      VARCHAR(500),
    last_activity TIMESTAMP NOT NULL,

    see_community SMALLINT NOT NULL, -- кто может видеть сообщества
    communities   INT NOT NULL       -- кол-во сообществ
);

-- Категории сообществ -------
CREATE TABLE community_categorys (
    id       SERIAL PRIMARY KEY,    -- id объекта
    name     VARCHAR(100) NOT NULL, -- название
    avatar   VARCHAR(500),          -- аватар
    position SMALLINT NOT NULL      -- порядковый номер
);

-- Суб-категории сообществ -------
CREATE TABLE community_subcategorys (
    id          SERIAL PRIMARY KEY,    -- id объекта
    name        VARCHAR(100) NOT NULL, -- название
    category_id INT NOT NULL,          -- id категории
    avatar      VARCHAR(500),          -- аватар
    position    SMALLINT NOT NULL      -- порядковый номер
);

CREATE TABLE communitys (
    id          SERIAL PRIMARY KEY,     -- id объекта
    name        VARCHAR(100) NOT NULL,  -- название
    status      VARCHAR(100),           -- статус
    types       SMALLINT NOT NULL,      -- тип
    link        VARCHAR(100) NOT NULL,  -- красивая ссылка
    s_avatar    VARCHAR(500),           -- маленький аватар
    category_id INT NOT NULL,           -- id категории
    user_id     INT NOT NULL,           -- id создателя

    UNIQUE(link)
);
CREATE INDEX communitys_user_id_idx ON communitys (user_id);

-- Члены сообщества -------
-- 1 подписчик
-- 2 модератор
-- 3 редактор
-- 4 рекламщик
-- 5 администратор
CREATE TABLE communities_memberships (
    id                SERIAL PRIMARY KEY,          -- id объекта
    user_id           INT NOT NULL,                -- id пользователя
    community_id      INT NOT NULL,                -- id сообщества
    level             SMALLINT NOT NULL DEFAULT 1, -- уровень доступа
    created           TIMESTAMP NOT NULL,          -- Создано
    visited           SMALLINT NOT NULL DEFAULT 0       -- Визиты в сообщество
);
CREATE UNIQUE INDEX communities_memberships_unq ON communities_memberships (user_id, community_id);

-- информация пользователей -------
CREATE TABLE community_infos (
    id           SERIAL PRIMARY KEY,
    community_id INT NOT NULL,
    avatar_id    INT,
    b_avatar     VARCHAR(500),
    status       VARCHAR(100),
    level        SMALLINT NOT NULL DEFAULT 100,
    cover        VARCHAR(500),
    created      TIMESTAMP NOT NULL,
    description  VARCHAR(500),
    members      INT NOT NULL
);
CREATE UNIQUE INDEX community_infos_unq ON community_infos (community_id, id);

-- 1 Все пользователи
-- 2 Подписчики
-- 3 Персонал
-- 4 Администраторы
-- 5 Владелец сообщества
-- 6 Подписчики, кроме
-- 7 Некоторые подписчики
CREATE TABLE community_privates (
    id           SERIAL PRIMARY KEY,
    community_id INT NOT NULL,
    see_member   SMALLINT NOT NULL, -- Кто видит сообщества
    see_info     SMALLINT NOT NULL, -- Кто видит информацию
    see_settings SMALLINT NOT NULL, -- Кто видит настройки
    see_log      SMALLINT NOT NULL, -- Кто видит логи
    see_stat     SMALLINT NOT NULL  -- Кто видит статистику
);
CREATE UNIQUE INDEX community_privates_unq ON community_privates (id, community_id);

-- Уведомления сообщества -------
CREATE TABLE community_notifications (
    id                   SERIAL PRIMARY KEY,
    community_id         INT NOT NULL,
    connection_request   BOOLEAN NOT NULL DEFAULT true,
    connection_confirmed BOOLEAN NOT NULL DEFAULT true,
    community_invite     BOOLEAN NOT NULL DEFAULT true
);
CREATE UNIQUE INDEX community_notifications_unq ON community_notifications (id, community_id);

-- Черный список -------
CREATE TABLE community_banned_users (
    id           SERIAL PRIMARY KEY,
    community_id INT NOT NULL,
    user_id      INT NOT NULL
);
CREATE UNIQUE INDEX community_banned_users_unq ON community_banned_users (community_id, user_id);

-- заявки на вступление в закрытое сообщество -------
CREATE TABLE community_follows (
    id           SERIAL PRIMARY KEY,
    user_id      INT NOT NULL,
    community_id INT NOT NULL,
    view         BOOLEAN NOT NULL DEFAULT false,
    visited      SMALLINT NOT NULL
);
CREATE UNIQUE INDEX follows_community_user_unq ON community_follows (user_id, community_id);

-- Приглашения в сообщества -------
CREATE TABLE community_invites (
    id             SERIAL PRIMARY KEY,
    user_id        INT NOT NULL,
    community_id   INT NOT NULL,
    invite_creator INT NOT NULL
);
CREATE UNIQUE INDEX community_invites_unq ON community_invites (user_id, community_id);


-- включения и исключения для пользователей касательно конкретного сообщества
-- 1 может видеть подписчиков
-- 2 может видеть информацию
-- 3 может видеть настройки
-- 4 может видеть логи
-- 5 может видеть статистику
-- 11 не может видеть подписчиков
-- 12 не может видеть информацию
-- 13 не может видеть настройки
-- 14 не может видеть логи
-- 15 не может видеть статистику

CREATE TABLE community_visible_perms (
    id           SERIAL PRIMARY KEY,
    community_id INT NOT NULL,
    target_id    INT NOT NULL,
    types        SMALLINT NOT NULL
);
CREATE UNIQUE INDEX community_visible_perms_unq ON community_visible_perms (target_id, community_id);


-- включения и исключения для пользователей касательно конкретного пользоватетеля
-- приватность
-- 1 может видеть сообщества
-- 11 не может видеть сообщества
-- 20 пользователь заблокирован у владельца блока сообществ

CREATE TABLE user_visible_perms (
  id         SERIAL PRIMARY KEY,
  user_id    INT NOT NULL,
  target_id  INT NOT NULL,
  types      SMALLINT NOT NULL
);
CREATE UNIQUE INDEX user_visible_perms_unq ON user_visible_perms (user_id, target_id);

-- друзья пользователей -------
CREATE TABLE friends (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    target_id INT NOT NULL
);
CREATE UNIQUE INDEX friends_user_target_unq ON friends (user_id, target_id);

-- подписчики пользователей -------
CREATE TABLE follows (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    target_id INT NOT NULL
);
CREATE UNIQUE INDEX follows_user_followed_unq ON follows (user_id, target_id);

-- рекомендованные друзья -------
CREATE TABLE featured_communities (
    id           SERIAL PRIMARY KEY,
    user_id      INT NOT NULL,
    community_id INT NOT NULL,
    hidden       BOOLEAN NOT NULL DEFAULT FALSE
);
CREATE UNIQUE INDEX featured_communities_unq ON featured_communities (community_id, user_id);
