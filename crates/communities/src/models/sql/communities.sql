/*
пользователи

Типы пользоватетеля
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

see_all see_community
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

31 Все друзья и списки подписчиков, кроме
32 Все друзья и некоторые списки подписчиков
33 Все подписчики и списки друзей, кроме
34 Все подписчики и некоторые списки друзей
35 Списки друзей, кроме
36 Некоторые списки друзей
37 Списки подписчиков, кроме
38 Некоторые списки подписчиков


*/
CREATE TABLE users (
    id            SERIAL PRIMARY KEY,
    user_id       INT NOT NULL,
    first_name    VARCHAR(100) NOT NULL,
    last_name     VARCHAR(100) NOT NULL,
    types         SMALLINT NOT NULL DEFAULT 1,
    is_man        BOOLEAN NOT NULL DEFAULT TRUE,
    password      VARCHAR(500) NOT NULL,
    link          VARCHAR(100) NOT NULL,
    s_avatar      VARCHAR(100),
    last_activity TIMESTAMP NOT NULL,
    see_all       SMALLINT NOT NULL,   -- кто может видеть открытый профиль
    see_community SMALLINT NOT NULL,   -- кто может видеть сообщества
    invite        SMALLINT NOT NULL,   -- кто может приглашать в сообщества
    lists         SMALLINT NOT NULL,   -- кол-во списков
    communities   INT NOT NULL         -- кол-во сообществ
);

-- Категории сообществ -------
CREATE TABLE community_categorys (
    id       SERIAL PRIMARY KEY,    -- id объекта
    name     VARCHAR(100) NOT NULL, -- название
    avatar   VARCHAR(100),          -- аватар
    position SMALLINT NOT NULL      -- порядковый номер
);

-- Суб-категории сообществ -------
CREATE TABLE community_subcategorys (
    id          SERIAL PRIMARY KEY,    -- id объекта
    name        VARCHAR(100) NOT NULL, -- название
    category_id INT NOT NULL,          -- id категории
    avatar      VARCHAR(100),          -- аватар
    position    SMALLINT NOT NULL      -- порядковый номер
);

/*
Тип списка
0 основной список
5 пользовательский список
45 удаленный пользовательский список
80 закрытый основной список
85 закрытый пользовательский список
120 замороженный основной список
125 замороженный пользовательский список
165 полностью удаленный пользовательский список
190 полностью удаленный пользовательский список приватный

списки фото
ниже цифра выбора приватности тех или иных действий пользователей
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

31 Все друзья и списки подписчиков, кроме 
32 Все друзья и некоторые списки подписчиков
33 Все подписчики и списки друзей, кроме
34 Все подписчики и некоторые списки друзей
35 Списки друзей, кроме
36 Некоторые списки друзей
37 Списки подписчиков, кроме
38 Некоторые списки подписчиков
*/
CREATE TABLE communities_lists (
    id             SERIAL PRIMARY KEY,
    name           VARCHAR(100) NOT NULL, -- название
    user_id        INT NOT NULL,          -- id пользователя (которое выше)
    types          SMALLINT NOT NULL,     -- тип (активен, удален, закрыт...)
    position       SMALLINT NOT NULL,     -- порядок следования
    count          INT NOT NULL,          -- кол-во элементов
    repost         INT NOT NULL,          -- кол-во репостов
    see_el         SMALLINT NOT NULL      -- кто может видеть сообщества списка
);
CREATE INDEX communities_lists_user_id_idx ON communities_lists (user_id);

/*
включения и исключения для пользователей / списков касательно конкретного списка сообществ -------
ниже цифра поля types, которая означает какое либо включение или
исключение:
1 пользователь может видеть список 
11 пользователь не может видеть список

101 список может видеть список 
111 список не может видеть список
*/
CREATE TABLE community_list_perms (
    id      SERIAL PRIMARY KEY,
    item_id INT NOT NULL,       -- id пользователя / списка, включенного или исключенного
    list_id INT NOT NULL,       -- id списка 
    types   SMALLINT NOT NULL   -- статус доступа
);
CREATE UNIQUE INDEX community_list_perms_unq ON community_list_perms (item_id, list_id);


/*
1 Все пользователи
2 Подписчики
3 Персонал
4 Администраторы
5 Владелец сообщества
6 Подписчики, кроме
7 Некоторые подписчики
8 списки подписчиков, кроме
9 Некоторые списки подписчиков
*/
CREATE TABLE communitys (
    id          SERIAL PRIMARY KEY,     -- id объекта
    name        VARCHAR(100) NOT NULL,  -- название
    status      VARCHAR(100),           -- статус
    types       SMALLINT NOT NULL,      -- тип
    link        VARCHAR(100) NOT NULL,  -- красивая ссылка
    s_avatar    VARCHAR(100),           -- маленький аватар
    category_id INT NOT NULL,           -- id категории
    user_id     INT NOT NULL,           -- id создателя 
    lists       INT NOT NULL,
    members     INT NOT NULL,

    UNIQUE(link)
);
CREATE INDEX communitys_user_id_idx ON communitys (user_id);


CREATE TABLE community_list_items (
    id           SERIAL PRIMARY KEY,
    list_id      INT NOT NULL,       -- id списка
    community_id INT NOT NULL,       -- id сообщества 
    visited      INT NOT NULL 
);
CREATE UNIQUE INDEX community_list_items_unq ON community_list_items (community_id, list_id);


-- списки подписчиков сообщества для группировки и групповых прав и событий
CREATE TABLE memberships_lists (
    id             SERIAL PRIMARY KEY,
    name           VARCHAR(100) NOT NULL, -- название
    community_id   INT NOT NULL,          -- id пользователя (которое выше)
    types          SMALLINT NOT NULL,     -- тип (активен, удален, закрыт...)
    position       SMALLINT NOT NULL,     -- порядок следования
    count          INT NOT NULL,          -- кол-во элементов
    repost         INT NOT NULL,          -- кол-во репостов
    see_el         SMALLINT NOT NULL      -- кто может видеть элементы списка
);
CREATE INDEX memberships_lists_community_id_idx ON memberships_lists (community_id);

---------------------------------------------
---------------------------------------------
/*
Члены сообщества -------
1 подписчик
2 модератор
3 редактор
4 рекламщик
5 администратор

6 забанен - нужен для инфы о заблокированном пользователе, есил он не состоит в 
подписчиках. Также для бана людей без их изгнания из сообщества
*/
CREATE TABLE communities_memberships ( 
    id                SERIAL PRIMARY KEY,
    user_id           INT NOT NULL,                -- id пользователя
    community_id      INT NOT NULL,                -- id сообщества
    level             SMALLINT NOT NULL DEFAULT 1, -- уровень доступа
    created           TIMESTAMP NOT NULL,          -- Создано
    visited           SMALLINT NOT NULL DEFAULT 0  -- Визиты в сообщество
);
CREATE UNIQUE INDEX communities_memberships_unq ON communities_memberships (user_id, community_id);

/*
включения и исключения для пользователей касательно конкретного списка подписчиков -------
ниже цифра поля types, которая означает какое либо включение или
исключение:
1 может видеть список 
11 не может видеть список

101 список может видеть список 
111 список не может видеть список
*/
CREATE TABLE memberships_list_perms (
    id       SERIAL PRIMARY KEY,
    item_id  INT NOT NULL, 
    list_id  INT NOT NULL,
    types    SMALLINT NOT NULL
);
CREATE UNIQUE INDEX memberships_list_perms_unq ON memberships_list_perms (item_id, list_id);

CREATE TABLE memberships_list_items (
    id       SERIAL PRIMARY KEY,
    list_id  INT NOT NULL,
    user_id  INT NOT NULL,
    visited  INT NOT NULL 
);
CREATE UNIQUE INDEX memberships_list_items_unq ON memberships_list_items (user_id, list_id);

---------------------------------------------
---------------------------------------------

-- информация пользователей -------
CREATE TABLE community_infos (
    id           SERIAL PRIMARY KEY,
    community_id INT NOT NULL,
    avatar_id    INT,
    b_avatar     VARCHAR(100),
    status       VARCHAR(100),
    level        SMALLINT NOT NULL DEFAULT 100,
    cover        VARCHAR(100),
    created      TIMESTAMP NOT NULL,
    description  VARCHAR(500)
);
CREATE UNIQUE INDEX community_infos_unq ON community_infos (community_id, id);

/*
1 Все пользователи
2 Подписчики
3 Персонал
4 Администраторы
5 Владелец сообщества
6 Подписчики, кроме
7 Некоторые подписчики
8 списки подписчиков, кроме
9 Некоторые списки подписчиков
*/
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
    connection_request   SMALLINT NOT NULL,
    connection_confirmed SMALLINT NOT NULL,
    community_invite     SMALLINT NOT NULL
);
CREATE UNIQUE INDEX community_notifications_unq ON community_notifications (id, community_id);

-- Черный список -------
CREATE TABLE community_banned_users (
    id           SERIAL PRIMARY KEY,
    community_id INT NOT NULL,
    user_id      INT NOT NULL,
    ban_to       TIMESTAMP 
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

/*
включения и исключения для пользователей касательно конкретного сообщества
1 может видеть подписчиков
2 может видеть информацию
3 может видеть настройки
4 может видеть логи
5 может видеть статистику
11 не может видеть подписчиков
12 не может видеть информацию
13 не может видеть настройки
14 не может видеть логи
15 не может видеть статистику

101 может видеть подписчиков
102 может видеть информацию
103 может видеть настройки
104 может видеть логи
105 может видеть статистику
111 не может видеть подписчиков
112 не может видеть информацию
113 не может видеть настройки
114 не может видеть логи
115 не может видеть статистику
*/
CREATE TABLE community_visible_perms (
    id           SERIAL PRIMARY KEY,
    community_id INT NOT NULL,
    item_id      INT NOT NULL,
    types        SMALLINT NOT NULL
);
CREATE UNIQUE INDEX community_visible_perms_unq ON community_visible_perms (item_id, community_id);

/*
включения и исключения для пользователей касательно конкретного пользоватетеля
приватность
0 может видеть профиль
1 может видеть сообщества
2 может приглашать в сообщества
10 не может видеть профиль
11 не может видеть сообщества
12 не может приглашать в сообщества
20 пользователь заблокирован у владельца блока сообществ

101 список может видеть сообщества
102 список может приглашать в сообщества
111 список не может видеть сообщества
112 список не может приглашать в сообщества
*/

CREATE TABLE user_visible_perms (
  id       SERIAL PRIMARY KEY,
  user_id  INT NOT NULL,
  item_id  INT NOT NULL,
  types    SMALLINT NOT NULL
);
CREATE UNIQUE INDEX user_visible_perms_unq ON user_visible_perms (user_id, item_id);


-----------------------------
-----------------------------
-- друзья пользователей -------
CREATE TABLE friends_lists (
    id       SERIAL PRIMARY KEY,
    list_id  INT NOT NULL,
    user_id  INT NOT NULL,
    types    SMALLINT NOT NULL
);
CREATE INDEX friends_lists_id_idx ON friends_lists (user_id);

CREATE TABLE friends (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    target_id INT NOT NULL
);
CREATE UNIQUE INDEX friends_user_target_unq ON friends (user_id, target_id);

CREATE TABLE friends_list_items (
    id       SERIAL PRIMARY KEY,
    list_id  INT NOT NULL,
    user_id  INT NOT NULL
);
CREATE UNIQUE INDEX friends_list_items_unq ON friends_list_items (user_id, list_id);

/*
friends_list_perms
1 пользователь может видеть список 
11 пользователь не может видеть список
101 список может видеть список 
111 список не может видеть список
*/
CREATE TABLE friends_list_perms (
    id       SERIAL PRIMARY KEY,
    item_id  INT NOT NULL, 
    list_id  INT NOT NULL,
    types    SMALLINT NOT NULL
);
CREATE UNIQUE INDEX friends_list_perms_unq ON friends_list_perms (item_id, list_id);

-- подписчики пользователей -------
CREATE TABLE follows_lists (
    id       SERIAL PRIMARY KEY,
    list_id  INT NOT NULL,  -- поле нужно для записи id списка подписчиков с сервиса пользователей.
    user_id  INT NOT NULL,
    types    SMALLINT NOT NULL
);
CREATE INDEX follows_lists_id_idx ON follows_lists (user_id);

CREATE TABLE follows (
    id        SERIAL PRIMARY KEY,
    user_id   INT NOT NULL,
    target_id INT NOT NULL
);
CREATE UNIQUE INDEX follows_user_followed_unq ON follows (user_id, target_id);

CREATE TABLE follows_list_items (
    id       SERIAL PRIMARY KEY,
    list_id  INT NOT NULL,
    user_id  INT NOT NULL
);
CREATE UNIQUE INDEX follows_list_items_unq ON follows_list_items (user_id, list_id);

/*
follows_list_perms
1 пользователь может видеть список 
11 пользователь не может видеть список
101 список может видеть список 
111 список не может видеть список
*/
CREATE TABLE follows_list_perms (
    id       SERIAL PRIMARY KEY,
    item_id  INT NOT NULL, 
    list_id  INT NOT NULL,
    types    SMALLINT NOT NULL
);
CREATE UNIQUE INDEX follows_list_perms_unq ON follows_list_perms (item_id, list_id);

-----------------------------
-----------------------------

-- рекомендованные друзья -------
CREATE TABLE featured_communities (
    id           SERIAL PRIMARY KEY,
    user_id      INT NOT NULL,
    community_id INT NOT NULL,
    hidden       BOOLEAN NOT NULL DEFAULT FALSE
);
CREATE UNIQUE INDEX featured_communities_unq ON featured_communities (community_id, user_id);


CREATE TABLE moderateds (
    id           SERIAL PRIMARY KEY,
    description  VARCHAR(500),
    verified     BOOLEAN NOT NULL DEFAULT false,
    status       SMALLINT NOT NULL,
    community_id INT NOT NULL,
    created      TIMESTAMP NOT NULL,
    count        INT NOT NULL
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
    community_id INT NOT NULL,
    status       SMALLINT NOT NULL,
    created      TIMESTAMP NOT NULL
);
CREATE UNIQUE INDEX moderated_penalties_unq ON moderated_penalties (user_id, moderated_id);

CREATE TABLE moderated_logs (
    id              SERIAL PRIMARY KEY,
    user_id         INT NOT NULL,
    community_id    INT NOT NULL,
    action          SMALLINT NOT NULL,
    description     VARCHAR(500),
    created         TIMESTAMP NOT NULL,
    time_to_suspend TIMESTAMP,

    CONSTRAINT fk_moderated_logs_manager
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);
CREATE INDEX moderated_logs_id_idx ON moderated_logs (user_id);


-- сервисы токенов и их разрешения. Работа с данными -------
-- только для сообществ-владельцев токенов
CREATE TABLE owner_services (
    id    SERIAL PRIMARY KEY,   -- id
    types SMALLINT NOT NULL,    -- определитель сервиса и доступа
    name  VARCHAR(100) NOT NULL -- название сервиса
);
CREATE INDEX owner_serivices_index ON owner_services (types);

-- создадим варианты для токенов, чтобы сто раз не добавлять
INSERT INTO owner_services (id, types, name) 
VALUES (1, 1, 'Сообщество') ON CONFLICT DO NOTHING;
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

-- дальше только для сообществ-владельцев токена - работа с управлением

INSERT INTO owner_services (id, types, name) 
VALUES (16, 31, 'Управление сообществом') ON CONFLICT DO NOTHING;
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

-- ключи доступа / токены к записям -------
CREATE TABLE owners (
    id           SERIAL PRIMARY KEY,     -- id
    user_id      INT NOT NULL,           -- id создателя или владельца
    community_id INT,                    -- id сообщества-владельца (если есть)
    name         VARCHAR(100) NOT NULL,  -- название
    description  VARCHAR(500),           -- описание
    types        SMALLINT NOT NULL,      -- тип владельца: приложение, пользователь, сообщество
    secret_key   VARCHAR(200) NOT NULL,  -- секретный ключ
    service_key  VARCHAR(200) NOT NULL,  -- сервисный ключ
    is_active    BOOLEAN NOT NULL,       -- активно

    UNIQUE(service_key)
);

CREATE INDEX item_service_key_index ON owners (service_key);

-- связь сервисов токенов с токенами -------
CREATE TABLE owner_services_items (
    id         SERIAL PRIMARY KEY, -- id
    owner_id   INT NOT NULL,       -- id токена-владельца
    service_id INT NOT NULL        -- id сервиса
);
CREATE UNIQUE INDEX owner_services_items_unq ON owner_services_items (owner_id, service_id);
