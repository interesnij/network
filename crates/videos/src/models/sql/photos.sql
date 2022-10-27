-- пользователи -------
-- таблица нужна для ассоциации постов с их создателями,
-- а также для самостоятельности сервиса.
CREATE TABLE users (
    id             SERIAL PRIMARY KEY,           -- id записи
    user_id        INT NOT NULL,                 -- id пользователя (ссылка на основную таблицу)
    first_name     VARCHAR(100) NOT NULL,        -- имя пользователя
    last_name      VARCHAR(100) NOT NULL,        -- фамилия пользователя
    types          SMALLINT NOT NULL DEFAULT 1,  -- тип (активен, удален, закрыт...)
    is_man         BOOLEAN NOT NULL DEFAULT TRUE,-- это мужчина?
    link           VARCHAR(100) NOT NULL,        -- ссылка и связь с основной таблицей
    s_avatar       VARCHAR(500),                 -- миниатюра
    last_activity  TIMESTAMP NOT NULL,           -- когда был в сети

    see_el         SMALLINT NOT NULL,          -- кто может видеть записи
    see_comment    SMALLINT NOT NULL,          -- кто может видеть комменты
    create_el      SMALLINT NOT NULL,          -- кто может создавать записи
    create_comment SMALLINT NOT NULL,          -- кто может создавать комменты
    copy_el        SMALLINT NOT NULL,          -- кто может копировать / репостить

    lists          INT NOT NULL,               -- кол-во списков записей
    photos         INT NOT NULL,               -- кол-во записей
    comments       INT NOT NULL,               -- кол-во комментов к записям

    UNIQUE(link)
);

-- сообщества -------
-- таблица нужна для ассоциации постов с их сообществами,
-- а также для самостоятельности сервиса.
CREATE TABLE communitys (
    id             SERIAL PRIMARY KEY,    -- id записи
    community_id   INT NOT NULL,          -- копия id сообщества с сервиса сообществ
    user_id        INT NOT NULL,          -- id владельца (ссылка на основную таблицу)
    name           VARCHAR(100) NOT NULL, -- название
    types          SMALLINT NOT NULL,     -- тип
    link           VARCHAR(100) NOT NULL, -- ссылка и связь с основной таблицей
    s_avatar       VARCHAR(500),          -- миниатюра

    see_el         SMALLINT NOT NULL,     -- кто может видеть записи
    see_comment    SMALLINT NOT NULL,     -- кто может видеть комменты
    create_el      SMALLINT NOT NULL,     -- кто может создавать записи
    create_comment SMALLINT NOT NULL,     -- кто может создавать комменты
    copy_el        SMALLINT NOT NULL,     -- кто может копировать / репостить

    lists          INT NOT NULL,          -- кол-во списков записей
    photos         INT NOT NULL,          -- кол-во записей
    comments       INT NOT NULL,          -- кол-во комментов к записям

    UNIQUE(link)
);
CREATE INDEX communitys_user_id_idx ON communitys (user_id);


-- списки фотографий -------
-- ниже цифра выбора приватности тех или иных действий пользователей
-- 1 Все пользователи
-- 2 Все друзья и все подписчики
-- 3 Все друзья и подписчики, кроме
-- 4 Все друзья и некоторые подписчики
-- 5 Все подписчики и друзья, кроме
-- 6 Все подписчики и некоторые друзья
-- 7 Все друзья
-- 8 Друзья, кроме
-- 9 Некоторые друзья
-- 10 Подписчики, кроме
-- 11 Некоторые подписчики
-- 12 Только я

-- 14 Все пользователи
-- 15 Подписчики
-- 16 Персонал
-- 17 Администраторы
-- 18 Подписчики, кроме
-- 19 Некоторые подписчики
-- 20 Владелец сообщества

CREATE TABLE photos_lists (
    id             SERIAL PRIMARY KEY,    -- id списка записей
    name           VARCHAR(100) NOT NULL, -- название

    community_id   INT,                   -- id сообщества (которое выше)
    user_id        INT NOT NULL,          -- id пользователя (которое выше)
    types          SMALLINT NOT NULL,     -- тип (активен, удален, закрыт...)
    description    VARCHAR(500),          -- описание
    image          VARCHAR(500),          -- миниатюра
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

    CONSTRAINT fk_photo_lists_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_photo_lists_community
        FOREIGN KEY(community_id)
            REFERENCES communitys(id)
);
CREATE INDEX photo_lists_user_id_idx ON photo_lists (user_id);
CREATE INDEX photo_lists_community_id_idx ON photo_lists (community_id);


-- записи -------
CREATE TABLE photos (
    id            SERIAL PRIMARY KEY,            -- id записи
    community_id  INT,                            -- id сообщества (которое выше)
    user_id       INT NOT NULL,                  -- id пользователя (которое выше)
    photo_list_id INT NOT NULL,                  -- id спискм записей
    types         SMALLINT NOT NULL,             -- тип (активен, удален, закрыт...)
    preview       VARCHAR(500) NOT NULL,
    file          VARCHAR(500) NOT NULL,
    description   VARCHAR(500),
    comments_on   BOOLEAN NOT NULL DEFAULT true, -- комменты разрешены
    created       TIMESTAMP NOT NULL,            -- время создания
    comment       INT NOT NULL,                  -- кол-во комментов
    view          INT NOT NULL,                  -- кол-во просмотров
    repost        INT NOT NULL,                  -- кол-во репостов
    copy          INT NOT NULL,                  -- кол-во копий
    position      SMALLINT NOT NULL,             -- позиция
    reactions     INT NOT NULL,

    CONSTRAINT fk_photos_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_photos_community
        FOREIGN KEY(community_id)
            REFERENCES communitys(id),

    CONSTRAINT fk_photos_parent
        FOREIGN KEY(parent_id)
            REFERENCES photos(id),

    CONSTRAINT fk_photos_list
        FOREIGN KEY(photo_list_id)
            REFERENCES photo_lists(id)
);
CREATE INDEX photos_community_id_idx ON photos (community_id);
CREATE INDEX photos_user_id_idx ON photos (user_id);
CREATE INDEX photos_list_id_idx ON photos (photo_list_id);
CREATE INDEX photos_parent_id_idx ON photos (parent_id);


-- комментарии к записям -------
CREATE TABLE photo_comments (
    id           SERIAL PRIMARY KEY, -- id коммента
    photo_id     INT NOT NULL,       -- id записи
    user_id      INT NOT NULL,       -- id комментатора
    community_id INT,
    sticker_id   INT,                -- id стикера
    parent_id    INT,                -- id родителя
    content      VARCHAR(1000),      -- содержание
    attach       VARCHAR(100),
    types        SMALLINT NOT NULL,  -- тип (активен, удален, закрыт...)
    created      TIMESTAMP NOT NULL, -- время создания
    repost       INT NOT NULL,       -- кол-во репостов
    reactions    INT NOT NULL,       -- кол-во реакций
    replies      INT NOT NULL,       -- кол-во ответов

    CONSTRAINT fk_photo_comments_user
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_photo_comments_community
        FOREIGN KEY(community_id)
            REFERENCES communitys(id),

    CONSTRAINT fk_photo_comment
        FOREIGN KEY(photo_id)
            REFERENCES photos(id),

    CONSTRAINT fk_photo_parent_comment
        FOREIGN KEY(parent_id)
          REFERENCES photo_comments(id)
);
CREATE INDEX photo_comments_photo_id_idx ON photo_comments (photo_id);
CREATE INDEX photo_comments_user_id_idx ON photo_comments (user_id);
CREATE INDEX photo_comments_sticker_id_idx ON photo_comments (sticker_id);
CREATE INDEX photo_comments_parent_id_idx ON photo_comments (parent_id);


-- Сохранение списка у пользователя в коллекции -------
CREATE TABLE user_photo_list_collections (
    id            SERIAL PRIMARY KEY, -- id записи
    user_id       INT NOT NULL,       -- id пользователя
    photo_list_id INT NOT NULL        -- id списка записей
);
CREATE UNIQUE INDEX user_photo_list_collections_unq ON user_photo_list_collections (user_id, photo_list_id);

-- Сохранение списка у сообщества в коллекции -------
CREATE TABLE community_photo_list_collections (
    id            SERIAL PRIMARY KEY, -- id записи
    community_id  INT NOT NULL,       -- id сообщества
    photo_list_id INT NOT NULL        -- id списка записей
);
CREATE UNIQUE INDEX community_photo_list_collections_unq ON community_photo_list_collections (community_id, photo_list_id);


-- включения и исключения для пользователей касательно конкретного списка записей -------
-- ниже цифра поля types, которая означает какое либо включение или
-- исключение:
-- 1 может видеть записи
-- 2 может видеть комменты
-- 3 может создавать записи
-- 4 может создавать комменты
-- 5 может копировать / репостить
-- 11 не может видеть записи
-- 12 не может видеть комменты
-- 13 не может создавать записи
-- 14 не может создавать комменты
-- 15 не может копировать / репостить
-- 21 в черном списке пользователя
-- 22 в черном списке сообщества

CREATE TABLE photo_list_perms (
    id            SERIAL PRIMARY KEY, -- id записи
    user_id       INT NOT NULL,       -- id пользователя
    photo_list_id INT NOT NULL,       -- id списка записей
    types         SMALLINT NOT NULL       -- статус доступа
);
CREATE UNIQUE INDEX photo_list_perms_unq ON photo_list_perms (user_id, photo_list_id);

-------
-- все реакции сервиса записей -------
CREATE TABLE reactions (
  id        SERIAL PRIMARY KEY,            -- id записи
  image     VARCHAR(500) NOT NULL,         -- изображение
  gif       VARCHAR(500) NOT NULL,         -- гифка
  name      VARCHAR(100) NOT NULL,         -- название
  is_active BOOLEAN NOT NULL DEFAULT true, -- активная реакция?
  position  SMALLINT NOT NULL              -- позиция
);

-- Уведомления записей пользователя -------
CREATE TABLE user_photo_notifications (
    id              SERIAL PRIMARY KEY,            -- id записи
    user_id         INT NOT NULL,                  -- id пользователя
    comment         BOOLEAN NOT NULL DEFAULT true, -- получать ли уведомления о новых комментах
    comment_reply   BOOLEAN NOT NULL DEFAULT true, -- получать ли уведомления о новых ответах
    mention         BOOLEAN NOT NULL DEFAULT true, -- получать ли уведомления о новых упоминаниях в постах
    comment_mention BOOLEAN NOT NULL DEFAULT true, -- получать ли уведомления о новых упоминаниях в комментах
    repost          BOOLEAN NOT NULL DEFAULT true, -- получать ли уведомления о новых репостах
    reactions       BOOLEAN NOT NULL DEFAULT true  -- получать ли уведомления о новых реакциях
);

CREATE UNIQUE INDEX user_photo_notifications_unq ON user_photo_notifications (user_id, id);

-- Уведомления записей сообщества -------
CREATE TABLE community_photo_notifications (
  id              SERIAL PRIMARY KEY,            -- id записи
  community_id    INT NOT NULL,                  -- id сообщества
  comment         BOOLEAN NOT NULL DEFAULT true, -- получать ли уведомления о новых комментах
  comment_reply   BOOLEAN NOT NULL DEFAULT true, -- получать ли уведомления о новых ответах
  mention         BOOLEAN NOT NULL DEFAULT true, -- получать ли уведомления о новых упоминаниях в постах
  comment_mention BOOLEAN NOT NULL DEFAULT true, -- получать ли уведомления о новых упоминаниях в комментах
  repost          BOOLEAN NOT NULL DEFAULT true, -- получать ли уведомления о новых репостах
  reactions       BOOLEAN NOT NULL DEFAULT true  -- получать ли уведомления о новых реакциях
);
CREATE UNIQUE INDEX community_photo_notifications_unq ON community_photo_notifications (id, community_id);

-- Порядок следования списков записей -------
CREATE TABLE user_photo_list_positions (
    id       SERIAL PRIMARY KEY,
    user_id  INT NOT NULL,      -- Пользователь
    list_id  INT NOT NULL,      -- Список записей
    position SMALLINT NOT NULL, -- Порядок отображения
    types    SMALLINT NOT NULL  -- 1 - открыт, 0 - недоступен (например, удален)
);

-- Порядок следования списков записей -------
CREATE TABLE community_photo_list_positions (
    id           SERIAL PRIMARY KEY,
    community_id INT NOT NULL,      -- Сообщество
    list_id      INT NOT NULL,      -- Список записей
    position     SMALLINT NOT NULL, -- Порядок отображения
    types        SMALLINT NOT NULL  -- 1 - открыт, 0 - недоступен (например, удален)
);
CREATE UNIQUE INDEX community_photo_list_positions_unq ON community_photo_list_positions (id, community_id);


-- счетчики реакций записи -------
-- каждой реакции поста выделена запись,
-- которая считает кол-во реакций.
CREATE TABLE photo_counter_reactions (
    id          SERIAL PRIMARY KEY, -- id записи
    photo_id    INT NOT NULL,       -- id поста
    reaction_id INT NOT NULL,       -- id реакции
    count       INT NOT NULL        -- кол-во отреагировавших
);

-- счетчики реакций коммента к записи -------
-- каждой реакции коммента выделена запись,
-- которая считает кол-во реакций.
CREATE TABLE photo_comment_counter_reactions (
    id               SERIAL PRIMARY KEY, -- id записи
    photo_comment_id INT NOT NULL,       -- id коммента
    reaction_id      INT NOT NULL,       -- id реакции
    count            INT NOT NULL        -- кол-во отреагировавших
);

-- реакции записи -------
-- тут те, кто реагирует на запись, со ссылкой на пользователей
-- этого сервиса.
CREATE TABLE photo_reactions (
  id          SERIAL PRIMARY KEY,
  user_id     INT NOT NULL,
  photo_id    INT NOT NULL,
  reaction_id INT NOT NULL
);
CREATE UNIQUE INDEX photo_reactions_unq ON photo_reactions (user_id, photo_id);

-- реакции комментов к записи -------
-- тут те, кто реагирует на комменты, со ссылкой на пользователей
-- этого сервиса.
CREATE TABLE photo_comment_reactions (
  id               SERIAL PRIMARY KEY,
  user_id          INT NOT NULL,
  photo_comment_id INT NOT NULL,
  reaction_id      INT NOT NULL
);
CREATE UNIQUE INDEX photo_comment_reactions_unq ON photo_comment_reactions (user_id, photo_comment_id);

------------------
------------------
-- Список ключей новостей, уведомлений или рекомендаций (если пользователь хочет их группировать) -------
CREATE TABLE list_user_communities_keys (
    id    SERIAL PRIMARY KEY,
    types SMALLINT NOT NULL,     -- тип списка: 0 - неактивен, 1 - основной, 2 - пользовательский
    name  VARCHAR(100) NOT NULL, -- название
    owner INT NOT NULL           -- владелец
);
-- Ключи новостей -------
-- таблица содержит id пользователей и сообществ,
-- для получения их записей в разделе новостей и рекомендаций
CREATE TABLE news_user_communities (
    id           SERIAL PRIMARY KEY,
    owner        INT NOT NULL,                   -- кто получает новости
    list_id      INT,
    user_id      INT,                            -- новости друга
    community_id INT,                            -- новости сообщества
    mute         BOOLEAN NOT NULL DEFAULT false, -- не получать новости источника
    sleep        TIMESTAMP
);

-- Ключи уведомлений -------
-- таблица содержит id пользователей и сообществ,
-- записи которых получают в уведомлениях
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

-- Члены сообщества -------
-- 1 подписчик
-- 2 модератор
-- 3 редактор
-- 4 рекламщик
-- 5 администратор
CREATE TABLE communities_memberships (
    id                SERIAL PRIMARY KEY,         -- id объекта
    user_id           INT NOT NULL,               -- id пользователя
    community_id      INT NOT NULL,               -- id сообщества
    level             SMALLINT NOT NULL DEFAULT 1 -- уровень доступа
);
CREATE UNIQUE INDEX communities_memberships_unq ON communities_memberships (user_id, community_id);


-- включения и исключения для пользователей касательно конкретного сообщества
-- 1 может видеть записи
-- 2 может видеть комменты к записям
-- 3 может создавать записи
-- 4 может создавать комменты к записям
-- 5 может копировать списки / записи
-- 11 не может видеть записи
-- 12 не может видеть комменты к записям
-- 13 не может создавать записи
-- 14 не может создавать комменты к записям
-- 15 не может копировать списки / записи
-- 20 пользователь заблокирован у сообщества записей

CREATE TABLE community_visible_perms (
    id           SERIAL PRIMARY KEY,
    community_id INT NOT NULL,
    target_id    INT NOT NULL,
    types        SMALLINT NOT NULL
);
CREATE UNIQUE INDEX community_visible_perms_unq ON community_visible_perms (community_id, id);


-- включения и исключения для пользователей касательно конкретного пользоватетеля
-- приватность
-- 1 может видеть записи
-- 2 может видеть комменты к записям
-- 3 может создавать записи
-- 4 может создавать комменты к записям
-- 5 может копировать списки / записи
-- 11 не может видеть записи
-- 12 не может видеть комменты к записям
-- 13 не может создавать записи
-- 14 не может создавать комменты к записям
-- 15 не может копировать списки / записи
-- 20 пользователь заблокирован у владельца записей

CREATE TABLE user_visible_perms (
  id         SERIAL PRIMARY KEY,
  user_id    INT NOT NULL,
  target_id  INT NOT NULL,
  types      SMALLINT NOT NULL
);
CREATE UNIQUE INDEX user_visible_perms_unq ON user_visible_perms (user_id, id);

-- Your SQL goes here

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
