/*
пользователи
таблица нужна для ассоциации постов с их создателями,
а также для самостоятельности сервиса.

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
    id             SERIAL PRIMARY KEY,    -- id записи
    user_id        INT NOT NULL,          -- id пользователя (ссылка на основную таблицу)
    first_name     VARCHAR(100) NOT NULL, -- имя пользователя
    last_name      VARCHAR(100) NOT NULL, -- фамилия пользователя
    types          SMALLINT NOT NULL,     -- тип (активен, удален, закрыт...)
    is_man         BOOLEAN NOT NULL,      -- это мужчина?
    password       VARCHAR(500) NOT NULL,
    link           VARCHAR(100) NOT NULL, -- ссылка и связь с основной таблицей
    s_avatar       VARCHAR(100),          -- миниатюра
    last_activity  TIMESTAMP NOT NULL,    -- когда был в сети

    see_all        SMALLINT NOT NULL,     -- кто может видеть открытый профиль
    see_el         SMALLINT NOT NULL,     -- кто может видеть записи
    see_comment    SMALLINT NOT NULL,     -- кто может видеть комменты
    create_el      SMALLINT NOT NULL,     -- кто может создавать записи
    create_comment SMALLINT NOT NULL,     -- кто может создавать комменты
    copy_el        SMALLINT NOT NULL,     -- кто может копировать / репостить

    lists          INT NOT NULL,          -- кол-во списков записей
    posts          INT NOT NULL,          -- кол-во записей
    comments       INT NOT NULL,          -- кол-во комментов к записям

    UNIQUE(link)
);

/*
сообщества
таблица нужна для ассоциации постов с их сообществами,
а также для самостоятельности сервиса.

1 Все пользователи
2 Подписчики
3 Персонал
4 Администраторы
5 Владелец сообщества
6 Подписчики, кроме
7 Некоторые подписчики
*/
CREATE TABLE communitys ( 
    id             SERIAL PRIMARY KEY,    -- id записи
    community_id   INT NOT NULL,          -- копия id сообщества с сервиса сообществ
    user_id        INT NOT NULL,          -- id владельца (ссылка на основную таблицу)
    name           VARCHAR(100) NOT NULL, -- название
    types          SMALLINT NOT NULL,     -- тип
    link           VARCHAR(100) NOT NULL, -- ссылка и связь с основной таблицей
    s_avatar       VARCHAR(100),          -- миниатюра

    see_el         SMALLINT NOT NULL,     -- кто может видеть записи
    see_comment    SMALLINT NOT NULL,     -- кто может видеть комменты
    create_list    SMALLINT NOT NULL,     -- кто может создавать списки
    create_el      SMALLINT NOT NULL,     -- кто может создавать записи
    create_comment SMALLINT NOT NULL,     -- кто может создавать комменты
    copy_el        SMALLINT NOT NULL,     -- кто может копировать / репостить

    lists          INT NOT NULL,          -- кол-во списков записей
    posts          INT NOT NULL,          -- кол-во записей
    comments       INT NOT NULL,          -- кол-во комментов к записям

    UNIQUE(link)
);
CREATE INDEX communitys_user_id_idx ON communitys (user_id);

/*
списки записей
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

14 Все пользователи
15 Подписчики
16 Персонал
17 Администраторы
18 Подписчики, кроме
19 Некоторые подписчики
20 Владелец сообщества 

31 Все друзья и списки подписчиков, кроме
32 Все друзья и некоторые списки подписчиков
33 Все подписчики и списки друзей, кроме
34 Все подписчики и некоторые списки друзей
35 Списки друзей, кроме
36 Некоторые списки друзей
37 Списки подписчиков, кроме
38 Некоторые списки подписчиков
39 Списки подписчиков сообщества, кроме
40 Некоторые списки подписчиков сообщества
*/
CREATE TABLE post_lists (
    id             SERIAL PRIMARY KEY,    -- id списка записей
    name           VARCHAR(100) NOT NULL, -- название

    community_id   INT,                   -- id сообщества (которое выше)
    user_id        INT NOT NULL,          -- id пользователя (которое выше)
    types          SMALLINT NOT NULL,     -- тип (активен, удален, закрыт...)
    description    VARCHAR(500),          -- описание
    image          VARCHAR(100),          -- миниатюра
    created        TIMESTAMP NOT NULL,    -- время создания

    count          INT NOT NULL,          -- кол-во записей
    repost         INT NOT NULL,          -- кол-во репостов
    copy           INT NOT NULL,          -- кол-во копий

    see_el         SMALLINT NOT NULL,     -- кто может видеть записи
    see_comment    SMALLINT NOT NULL,     -- кто может видеть комменты
    create_el      SMALLINT NOT NULL,     -- кто может создавать записи
    create_comment SMALLINT NOT NULL,     -- кто может создавать комменты
    copy_el        SMALLINT NOT NULL,     -- кто может копировать / репостить
    reactions      VARCHAR(100),          -- разрешенные реакции

    CONSTRAINT fk_post_lists_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_post_lists_community
        FOREIGN KEY(community_id)
            REFERENCES communitys(id)
);
CREATE INDEX post_lists_user_id_idx ON post_lists (user_id);
CREATE INDEX post_lists_community_id_idx ON post_lists (community_id);


-- записи -------
CREATE TABLE posts (
    id           SERIAL PRIMARY KEY,            -- id записи
    content      VARCHAR(5000),                 -- содержание
    community_id INT,                           -- id сообщества (которое выше)
    user_id      INT NOT NULL,                  -- id пользователя (которое выше)
    post_list_id INT NOT NULL,                  -- id спискм записей
    types        SMALLINT NOT NULL,             -- тип (активен, удален, закрыт...)
    attach       VARCHAR(100),                  -- прикрепленные объекты
    comments_on  BOOLEAN NOT NULL DEFAULT true, -- комменты разрешены
    created      TIMESTAMP NOT NULL,            -- время создания
    comment      INT NOT NULL,                  -- кол-во комментов
    view         INT NOT NULL,                  -- кол-во просмотров
    repost       INT NOT NULL,                  -- кол-во репостов
    copy         INT NOT NULL,                  -- кол-во копий
    position     SMALLINT NOT NULL,             -- позиция
    is_signature BOOLEAN NOT NULL DEFAULT false,-- разрешить подпись
    parent_id    INT,                           -- id родителя
    reactions    INT NOT NULL,

    CONSTRAINT fk_posts_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_posts_community
        FOREIGN KEY(community_id)
            REFERENCES communitys(id),

    CONSTRAINT fk_posts_parent
        FOREIGN KEY(parent_id)
            REFERENCES posts(id),

    CONSTRAINT fk_posts_list
        FOREIGN KEY(post_list_id)
            REFERENCES post_lists(id)
);
CREATE INDEX posts_community_id_idx ON posts (community_id);
CREATE INDEX posts_user_id_idx ON posts (user_id);
CREATE INDEX posts_list_id_idx ON posts (post_list_id);
CREATE INDEX posts_parent_id_idx ON posts (parent_id);


-- комментарии к записям -------
CREATE TABLE post_comments (
    id           SERIAL PRIMARY KEY, -- id коммента
    post_id      INT NOT NULL,       -- id записи
    user_id      INT NOT NULL,       -- id комментатора
    community_id INT,
    parent_id    INT,                -- id родителя
    content      VARCHAR(1000),      -- содержание
    attach       VARCHAR(100),
    types        SMALLINT NOT NULL,  -- тип (активен, удален, закрыт...)
    created      TIMESTAMP NOT NULL, -- время создания
    repost       INT NOT NULL,       -- кол-во репостов
    reactions    INT NOT NULL,       -- кол-во реакций
    replies      INT NOT NULL,       -- кол-во ответов

    CONSTRAINT fk_post_comments_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_post_comments_community
        FOREIGN KEY(community_id)
            REFERENCES communitys(id),

    CONSTRAINT fk_post_comment
        FOREIGN KEY(post_id)
            REFERENCES posts(id),

    CONSTRAINT fk_post_parent_comment
        FOREIGN KEY(parent_id)
          REFERENCES post_comments(id)
);
CREATE INDEX post_comments_post_id_idx ON post_comments (post_id);
CREATE INDEX post_comments_user_id_idx ON post_comments (user_id);
CREATE INDEX post_comments_parent_id_idx ON post_comments (parent_id);


-- Сохранение списка у пользователя в коллекции -------
CREATE TABLE user_post_list_collections (
    id           SERIAL PRIMARY KEY, -- id записи
    user_id      INT NOT NULL,       -- id пользователя
    post_list_id INT NOT NULL        -- id списка записей
);
CREATE UNIQUE INDEX user_post_list_collections_unq ON user_post_list_collections (user_id, post_list_id);

-- Сохранение списка у сообщества в коллекции -------
CREATE TABLE community_post_list_collections (
    id           SERIAL PRIMARY KEY, -- id записи
    community_id INT NOT NULL,       -- id сообщества
    post_list_id INT NOT NULL        -- id списка записей
);
CREATE UNIQUE INDEX community_post_list_collections_unq ON community_post_list_collections (community_id, post_list_id);

/*
включения и исключения для пользователей касательно конкретного списка записей -------
ниже цифра поля types, которая означает какое либо включение или
исключение:
1 может видеть записи 
2 может видеть комменты
3 может создавать записи
4 может создавать комменты
5 может копировать / репостить
11 не может видеть записи
12 не может видеть комменты
13 не может создавать записи
14 не может создавать комменты
15 не может копировать / репостить
*/
CREATE TABLE post_list_perms (
    id              SERIAL PRIMARY KEY, -- id записи
    user_id         INT NOT NULL,       -- id пользователя
    post_list_id    INT NOT NULL,       -- id списка записей
    types           SMALLINT NOT NULL       -- статус доступа
); 
CREATE UNIQUE INDEX post_list_perms_unq ON post_list_perms (user_id, post_list_id);

-- Уведомления записей пользователя -------
CREATE TABLE user_post_notifications (
    id              SERIAL PRIMARY KEY,            -- id записи
    user_id         INT NOT NULL,                  -- id пользователя
    comment         SMALLINT NOT NULL, -- получать ли уведомления о новых комментах
    comment_reply   SMALLINT NOT NULL, -- получать ли уведомления о новых ответах
    mention         SMALLINT NOT NULL, -- получать ли уведомления о новых упоминаниях в постах
    comment_mention SMALLINT NOT NULL, -- получать ли уведомления о новых упоминаниях в комментах
    repost          SMALLINT NOT NULL, -- получать ли уведомления о новых репостах
    reactions       SMALLINT NOT NULL  -- получать ли уведомления о новых реакциях
);

CREATE UNIQUE INDEX user_post_notifications_unq ON user_post_notifications (user_id, id);

-- Уведомления записей сообщества ------- 
CREATE TABLE community_post_notifications (
  id              SERIAL PRIMARY KEY,            -- id записи
  community_id    INT NOT NULL,                  -- id сообщества
  comment         SMALLINT NOT NULL, -- получать ли уведомления о новых комментах
  comment_reply   SMALLINT NOT NULL, -- получать ли уведомления о новых ответах
  mention         SMALLINT NOT NULL, -- получать ли уведомления о новых упоминаниях в постах
  comment_mention SMALLINT NOT NULL, -- получать ли уведомления о новых упоминаниях в комментах
  repost          SMALLINT NOT NULL, -- получать ли уведомления о новых репостах
  reactions       SMALLINT NOT NULL  -- получать ли уведомления о новых реакциях
);
CREATE UNIQUE INDEX community_post_notifications_unq ON community_post_notifications (id, community_id);

-- Порядок следования списков записей -------
CREATE TABLE user_post_list_positions (
    id       SERIAL PRIMARY KEY,
    user_id  INT NOT NULL,      -- Пользователь
    list_id  INT NOT NULL,      -- Список записей
    position SMALLINT NOT NULL, -- Порядок отображения
    types    SMALLINT NOT NULL  -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списков записей -------
CREATE TABLE community_post_list_positions (
    id           SERIAL PRIMARY KEY,
    community_id INT NOT NULL,      -- Сообщество
    list_id      INT NOT NULL,      -- Список записей
    position     SMALLINT NOT NULL, -- Порядок отображения
    types        SMALLINT NOT NULL  -- 1 - открыт, 0 - недоступен (например, удален)
);
CREATE UNIQUE INDEX community_post_list_positions_unq ON community_post_list_positions (id, community_id);

/*
счетчики реакций записи -------
каждой реакции поста выделена запись,
которая считает кол-во реакций.
*/
CREATE TABLE post_counter_reactions (
    id          SERIAL PRIMARY KEY, -- id записи
    post_id     INT NOT NULL,       -- id поста
    reaction_id INT NOT NULL,       -- id реакции
    count       INT NOT NULL        -- кол-во отреагировавших
);

/*
счетчики реакций коммента к записи -------
каждой реакции коммента выделена запись,
которая считает кол-во реакций.
*/
CREATE TABLE post_comment_counter_reactions (
    id              SERIAL PRIMARY KEY, -- id записи
    post_comment_id INT NOT NULL,       -- id коммента
    reaction_id     INT NOT NULL,       -- id реакции
    count           INT NOT NULL        -- кол-во отреагировавших
);

/*
реакции записи
тут те, кто реагирует на запись, со ссылкой на пользователей
этого сервиса.
*/
CREATE TABLE post_reactions (
  id          SERIAL PRIMARY KEY,
  user_id     INT NOT NULL,
  post_id     INT NOT NULL,
  reaction_id INT NOT NULL
);
CREATE UNIQUE INDEX post_reactions_unq ON post_reactions (user_id, post_id);

/*
реакции комментов
тут те, кто реагирует на комменты, со ссылкой на пользователей
этого сервиса.
*/
CREATE TABLE post_comment_reactions (
  id              SERIAL PRIMARY KEY,
  user_id         INT NOT NULL,
  post_comment_id INT NOT NULL,
  reaction_id     INT NOT NULL
);
CREATE UNIQUE INDEX post_comment_reactions_unq ON post_comment_reactions (user_id, post_comment_id);

/*
ссылки на записи или сообщения, репостнувшие список записей
нужно для показа сообществ или пользователей, которые репостнули
*/
CREATE TABLE item_reposts (
  id         SERIAL PRIMARY KEY,
  item_id    INT NOT NULL,
  item_types SMALLINT NOT NULL,
  post_id    INT,
  message_id INT,

  CONSTRAINT fk_item_reposts_post
      FOREIGN KEY(post_id)
          REFERENCES posts(id)
);
CREATE INDEX item_reposts_item_id_idx ON item_reposts (item_id);
CREATE INDEX item_reposts_post_id_idx ON item_reposts (post_id);
CREATE INDEX item_reposts_message_id_idx ON item_reposts (message_id);

/*
Список ключей новостей, уведомлений или рекомендаций (если пользователь хочет их 
группировать)
*/
CREATE TABLE list_user_communities_keys (
    id    SERIAL PRIMARY KEY,
    types SMALLINT NOT NULL,     -- тип списка: 0 - неактивен, 1 - основной, 2 - пользовательский
    name  VARCHAR(100) NOT NULL, -- название
    owner INT NOT NULL           -- владелец
);

/*
Ключи новостей
таблица содержит id пользователей и сообществ,
для получения их записей в разделе новостей и рекомендаций
*/
CREATE TABLE news_user_communities (
    id           SERIAL PRIMARY KEY,
    owner        INT NOT NULL,                   -- кто получает новости
    list_id      INT,
    user_id      INT,                            -- новости друга
    community_id INT,                            -- новости сообщества
    mute         BOOLEAN NOT NULL DEFAULT false, -- не получать новости источника
    sleep        TIMESTAMP
);

/*
Ключи уведомлений
таблица содержит id пользователей и сообществ,
записи которых получают в уведомлениях
*/
CREATE TABLE notify_user_communities (
    id           SERIAL PRIMARY KEY,
    owner        INT NOT NULL,                   -- кто получает уведомления
    list_id      INT,
    user_id      INT,                            -- уведомления друга
    community_id INT,                            -- уведомления сообщества
    mute         BOOLEAN NOT NULL DEFAULT false, -- не получать уведомления источника
    sleep        TIMESTAMP
);

-- друзья -------
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

/*
Члены сообщества -------
1 подписчик
2 модератор
3 редактор
4 рекламщик
5 администратор
*/
CREATE TABLE communities_memberships (
    id                SERIAL PRIMARY KEY,         -- id объекта
    user_id           INT NOT NULL,               -- id пользователя
    community_id      INT NOT NULL,               -- id сообщества
    level             SMALLINT NOT NULL DEFAULT 1 -- уровень доступа
);
CREATE UNIQUE INDEX communities_memberships_unq ON communities_memberships (user_id, community_id);


/*
включения и исключения для пользователей касательно конкретного сообщества
1 может видеть записи
2 может видеть комменты к записям
3 может создавать записи
4 может создавать комменты к записям
5 может копировать списки / записи
6 может создавать списки

11 не может видеть записи
12 не может видеть комменты к записям
13 не может создавать записи
14 не может создавать комменты к записям
15 не может копировать списки / записи
16 не может создавать списки

20 пользователь заблокирован у сообщества записей

51 создает уведомление о комментарии 
52 создает уведомление о ответе
53 создает уведомление о упоминании в посте
54 создает уведомление о упоминании в комменте
55 создает уведомление о репосте 
56 создает уведомление о реакции

61 не создает уведомление о комментарии
62 не создает уведомление о ответе
63 не создает уведомление о упоминании в посте
64 не создает уведомление о упоминании в комменте
65 не создает уведомление о репосте
66 не создает уведомление о реакции
*/
CREATE TABLE community_visible_perms (
    id           SERIAL PRIMARY KEY,
    community_id INT NOT NULL,
    target_id    INT NOT NULL,
    types        SMALLINT NOT NULL
);
CREATE UNIQUE INDEX community_visible_perms_unq ON community_visible_perms (community_id, id);

/*
включения и исключения для пользователей касательно конкретного пользоватетеля
приватность
0 может видеть профиль открытым
1 может видеть записи
2 может видеть комменты к записям
3 может создавать записи
4 может создавать комменты к записям
5 может копировать списки / записи

10 не может видеть профиль открытым
11 не может видеть записи
12 не может видеть комменты к записям
13 не может создавать записи
14 не может создавать комменты к записям
15 не может копировать списки / записи

20 пользователь заблокирован у владельца записей

51 не создает уведомление о комментарии
52 не создает уведомление о ответе
53 не создает уведомление о упоминании в посте
54 не создает уведомление о упоминании в комменте
55 не создает уведомление о репосте 
56 не создает уведомление о реакции

61 не создает уведомление о комментарии
62 не создает уведомление о ответе
63 не создает уведомление о упоминании в посте
64 не создает уведомление о упоминании в комменте
65 не создает уведомление о репосте
66 не создает уведомление о реакции
*/
CREATE TABLE user_visible_perms (
  id         SERIAL PRIMARY KEY,
  user_id    INT NOT NULL,
  target_id  INT NOT NULL,
  types      SMALLINT NOT NULL
);
CREATE UNIQUE INDEX user_visible_perms_unq ON user_visible_perms (user_id, id);

-- Ключи рекомендаций -------
CREATE TABLE featured_user_communities (
    id            SERIAL PRIMARY KEY,
    owner         INT NOT NULL,                   -- кто получает рекомендации
    list_id       INT,                            -- список, если есть
    user_id       INT,                            -- рекомендуемый друг
    community_id  INT,                            -- рекомендуемое сообщество
    mute          BOOLEAN NOT NULL DEFAULT false, -- не получать рекомендации и>
    sleep         TIMESTAMP
);


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
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (1, 0, 4) ON CONFLICT DO NOTHING;
INSERT INTO owner_services_items (id, owner_id, service_id)
VALUES (2, 0, 34) ON CONFLICT DO NOTHING;

/*
объекты других сервисов

пользователи - владельцы прикрепленных объектов -------
таблица нужна для: 
1. ассоциации прикрепленных объектов с их создателями,
2. ассоциации включенных / исключенных пользователей для настроек

see_all - кто видит открытый профиль и следоватенльно всю
информацию.
*/
CREATE TABLE item_users (
    id         SERIAL PRIMARY KEY,    -- id записи
    user_id    INT NOT NULL,          -- id пользователя (ссылка на основную таблицу)
    first_name VARCHAR(100) NOT NULL, -- имя пользователя
    last_name  VARCHAR(100) NOT NULL, -- фамилия пользователя
    types      SMALLINT NOT NULL,     -- тип (активен, удален, закрыт...)
    link       VARCHAR(100) NOT NULL, -- ссылка и связь с основной таблицей
    s_avatar   VARCHAR(500),          -- миниатюра
    see_all    SMALLINT NOT NULL,     -- кто может видеть открытый профиль

    UNIQUE(link) 
);
CREATE INDEX item_users_id_idx ON item_users (user_id);

/*
сообщества - владельцы прикрепленных объектов -------
таблица нужна для ассоциации ассоциации прикрепленных объектов с их сообществами,
а также для самостоятельности сервиса.
*/

CREATE TABLE item_communitys (
    id           SERIAL PRIMARY KEY,    -- id записи
    community_id INT NOT NULL,          -- копия id сообщества с сервиса сообществ
    name         VARCHAR(100) NOT NULL, -- название
    types        SMALLINT NOT NULL,     -- тип
    link         VARCHAR(100) NOT NULL, -- ссылка и связь с основной таблицей
    s_avatar     VARCHAR(500),          -- миниатюра

    UNIQUE(link)
);
CREATE INDEX item_communitys_id_idx ON item_communitys (community_id);

/*
объекты списков объектов универсальные -------
аватар, фио, ссылку легко получить из объекта владельца.
Владелец либо будет на этом сервисе, либо мы его создадим.
ЗДЕСЬ И ДАЛЕЕ - ТА ЖЕ СХЕМА!!

list_types ↓
20 Список записей (не создаем, он и так есть на этом сервисе)
21 Плейлист
22 Список документов
23 Список опросов
24 Список фотографий
25 Список роликов
26 Список товаров
27 Список обсуждений
28 Список википедии
29 Список статей
30 Папка
31 Список стикеров

добавим поля приватности
*/

CREATE TABLE item_lists (
    id           SERIAL PRIMARY KEY,    -- id списка записей
    name         VARCHAR(100) NOT NULL, -- название
    user_id      INT NOT NULL,          -- id пользователя (ссылка на таблицу выше)
    community_id INT,                   -- id сообщества (ссылка на таблицу выше)
    list_id      INT NOT NULL,          -- id списка
    list_types   SMALLINT NOT NULL,     -- тип списка (выше)
    types        SMALLINT NOT NULL,     -- тип (активен, удален, закрыт...)
    image        VARCHAR(500),          -- миниатюра
    count        INT NOT NULL,          -- кол-во элементов
    see_el       SMALLINT NOT NULL,     -- кто может видеть список
    copy_el      SMALLINT NOT NULL      -- кто может копировать список
);

/*
основняк приватности элементов и комментов. Если владелец
меняет приватность списка или целого сервиса,
эта таблица хранит расчет приватности для прикрепленного объекта.
например, изменилась приватность стены пользователя: смотрим по всем сервисам
есть ли такие записи с id пользователя, то меняем цифру приватности
на соразмерную всей стене. Если меняем список записей, то
ищем одну запись с list_id == list.id и типом "записи".

вообще, при изменении стены нужно проверять и таблицу perms_lists,
и запись item_lists. При изменении приватности списка - также две эти таблицы
*/

CREATE TABLE perms_lists (
    id           SERIAL PRIMARY KEY, -- id списка записей
    user_id      INT NOT NULL,       -- id пользователя-владельца (ссылка на таблицу выше)
    community_id INT,                -- id сообщества-владельца (ссылка на таблицу выше)
    list_id      INT NOT NULL,       -- id списка
    list_types   SMALLINT NOT NULL,  -- тип списка (выше)
    types        SMALLINT NOT NULL,  -- тип (активен, удален, закрыт...)
    see_el       SMALLINT NOT NULL,  -- кто может видеть элементы списка
    copy_el      SMALLINT NOT NULL   -- кто может копировать элементы списка
);

/*
объекты комментарий универсальные -------
comment_types ↓
81 Коммент к записи     (не создаем, он и так есть на этом сервисе)
82 Коммент к фотографии
83 Коммент к ролику
84 Коммент к товару
85 Коммент к обсуждению
86 Коммент к статье википедии
87 Коммент форума
*/

CREATE TABLE item_comments (
    id            SERIAL PRIMARY KEY, -- id коммента
    user_id       INT NOT NULL,       -- id комментатора
    community_id  INT,
    content       VARCHAR(100),       -- часть содержания
    attach        VARCHAR(100),
    comment_id    INT NOT NULL,       -- id коммента
    comment_types SMALLINT NOT NULL,  -- тип коммента
    item_id       INT NOT NULL,       -- id объекта
    item_types    SMALLINT NOT NULL,  -- тип объекта
    types         SMALLINT NOT NULL,  -- тип (активен, удален, закрыт...)
    created       TIMESTAMP NOT NULL  -- время создания
);

/*
объекты записей 
связь со списком нужна, так как именно список будет определять
приватность прикрепленных объектов.
значит, придется первый раз создавать и список записей...
(не создаем, он и так есть на этом сервисе)

CREATE TABLE item_posts (
    id            SERIAL PRIMARY KEY,  -- id записи
    content       VARCHAR(100),        -- часть содержания
    list_id       INT NOT NULL,        -- id списка (а тип и так понятен)
    community_id  INT,                 -- id сообщества (которое выше)
    user_id       INT NOT NULL,        -- id пользователя (которое выше)
    item_id       INT NOT NULL,        -- id записи (которое выше)
    types         SMALLINT NOT NULL,   -- тип (активен, удален, закрыт...)
    attach        VARCHAR(100),        -- прикрепленные объекты
    created       TIMESTAMP NOT NULL,  -- время создания
    is_signature  BOOLEAN NOT NULL     -- разрешить подпись
);
*/


-- объекты фотографий -------
CREATE TABLE item_photos (
    id           SERIAL PRIMARY KEY,
    user_id      INT NOT NULL,       -- id комментатора
    community_id INT,
    list_id      INT NOT NULL,        -- id списка (а тип и так понятен)
    item_id      INT NOT NULL,
    preview      VARCHAR(500) NOT NULL,
    file         VARCHAR(500) NOT NULL,
    types        SMALLINT NOT NULL
);

-- объекты документов -------
CREATE TABLE item_docs (
    id           SERIAL PRIMARY KEY,
    title        VARCHAR(200) NOT NULL,
    user_id      INT NOT NULL,
    community_id INT,
    list_id      INT NOT NULL,
    item_id      INT NOT NULL,
    types        SMALLINT NOT NULL,
    file         VARCHAR(500) NOT NULL
);

-- объекты товаров -------
CREATE TABLE item_goods (
    id           SERIAL PRIMARY KEY,
    title        VARCHAR(100) NOT NULL,
    user_id      INT NOT NULL,
    community_id INT,
    list_id      INT NOT NULL,
    item_id      INT NOT NULL,
    price        INT,
    types        SMALLINT NOT NULL,
    image        VARCHAR(500)
);

-- объекты статей -------
CREATE TABLE item_articles (
    id           SERIAL PRIMARY KEY,
    title        VARCHAR(100) NOT NULL,
    user_id      INT NOT NULL,
    community_id INT,
    list_id      INT NOT NULL,
    item_id      INT NOT NULL,
    types        SMALLINT NOT NULL,
    image        VARCHAR(500)
);

-- объекты вики -------
CREATE TABLE item_wikis (
    id           SERIAL PRIMARY KEY,
    title        VARCHAR(100) NOT NULL,
    user_id      INT NOT NULL,
    community_id INT,
    list_id      INT NOT NULL,
    item_id      INT NOT NULL,
    types        SMALLINT NOT NULL,
    image        VARCHAR(500)
);

-- объекты вики -------
CREATE TABLE item_forums (
    id           SERIAL PRIMARY KEY,
    title        VARCHAR(100) NOT NULL,
    user_id      INT NOT NULL,
    community_id INT,
    list_id      INT NOT NULL,
    item_id      INT NOT NULL,
    types        SMALLINT NOT NULL
);

-- объекты треков -------
CREATE TABLE item_audios (
    id           SERIAL PRIMARY KEY,
    title        VARCHAR(100) NOT NULL,
    user_id      INT NOT NULL,
    community_id INT,
    list_id      INT NOT NULL,
    item_id      INT NOT NULL,
    artist_id    INT,
    types        SMALLINT NOT NULL,
    file         VARCHAR(500) NOT NULL,
    image        VARCHAR(500)
);

-- объекты опросов -------
CREATE TABLE item_surveys (
    id           SERIAL PRIMARY KEY,
    title        VARCHAR(100) NOT NULL,
    user_id      INT NOT NULL,
    community_id INT,
    list_id      INT NOT NULL,
    item_id      INT NOT NULL,
    types        SMALLINT NOT NULL,
    image        VARCHAR(500),
    is_anonymous BOOLEAN NOT NULL DEFAULT false,
    is_multiple  BOOLEAN NOT NULL DEFAULT false,
    is_no_edited BOOLEAN NOT NULL DEFAULT false,
    time_end     TIMESTAMP,
    vote         INT NOT NULL
);

-- объекты роликов -------
CREATE TABLE item_videos (
    id           SERIAL PRIMARY KEY,
    title        VARCHAR(100) NOT NULL,
    user_id      INT NOT NULL,
    community_id INT,
    list_id      INT NOT NULL,
    item_id      INT NOT NULL,
    types        SMALLINT NOT NULL,
    image        VARCHAR(500),
    file         VARCHAR(500) NOT NULL,
    view         INT NOT NULL
);

-- объекты сайтов -------
CREATE TABLE item_sites (
    id           SERIAL PRIMARY KEY,
    title        VARCHAR(100) NOT NULL,
    user_id      INT NOT NULL,
    community_id INT,
    item_id      INT NOT NULL,
    types        SMALLINT NOT NULL
);

/*
прикрепленные копии -------
типы объектов (item_types) для attach_items
1  Пользователь item_users
2  Сообщество   item_communitys
3  Сайт         item_sites
20 Список записей
21 Плейлист
22 Список документов
23 Список опросов
24 Список фотографий
25 Список роликов
26 Список товаров
27 Список обсуждений
28 Список википедии
29 Список статей
30 Папка
31 Список стикеров
81 Коммент к записи
82 Коммент к фотографии
83 Коммент к ролику
84 Коммент к товару
85 Коммент к обсуждению
86 Коммент к статье википедии
87 Коммент форума
51 Запись       item_posts
52 Трек         item_audios
53 Документ     item_docs
54 Опрос        item_surveys
55 Фотография   item_photos
56 Ролик        item_videos
57 Товар        item_goods
58 Обсуждение   item_forums
59 Статья вики  item_wikis
60 Статья       item_articles

101 Рабочее пространство планировщика
102 Доска планировщика
103 Колонка планировщика
104 Карточка планировщика
105 Коммент к карточке планировщика
*/

CREATE TABLE attach_items (
    id           SERIAL PRIMARY KEY, -- id записи
    item_id      INT NOT NULL,       -- id объекта (которые выше)
    item_types   SMALLINT NOT NULL,  -- тип связанного объекта (которые выше)
    attach_types SMALLINT NOT NULL   -- к чему прикреплен объект
);
