-- объекты других сервисов -------
----------------------------------

-- пользователи - владельцы прикрепленных объектов -------
-- таблица нужна для ассоциации прикрепленных объектов с их создателями,
-- а также для самостоятельности сервиса.
CREATE TABLE item_users (
    id         SERIAL PRIMARY KEY,           -- id записи
    user_id    INT NOT NULL,                 -- id пользователя (ссылка на основную таблицу)
    first_name VARCHAR(100) NOT NULL,        -- имя пользователя
    last_name  VARCHAR(100) NOT NULL,        -- фамилия пользователя
    types      SMALLINT NOT NULL DEFAULT 1,  -- тип (активен, удален, закрыт...)
    link       VARCHAR(100) NOT NULL,        -- ссылка и связь с основной таблицей
    s_avatar   VARCHAR(500),                 -- миниатюра

    UNIQUE(link)
);
CREATE INDEX item_users_id_idx ON item_users (user_id);

-- сообщества - владельцы прикрепленных объектов -------
-- таблица нужна для ассоциации ассоциации прикрепленных объектов с их сообществами,
-- а также для самостоятельности сервиса.
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


-- объекты списков объектов универсальные -------
-- аватар, фио, ссылку легко получить из объекта владельца.
-- Владелец либо будет на этом сервисе, либо мы его создадим.
-- ЗДЕСЬ И ДАЛЕЕ - ТА ЖЕ СХЕМА!!

-- list_types ↓
-- 20 Список записей (не создаем, он и так есть на этом сервисе)
-- 21 Плейлист
-- 22 Список документов
-- 23 Список опросов
-- 24 Список фотографий
-- 25 Список роликов
-- 26 Список товаров
-- 27 Список обсуждений
-- 28 Список википедии
-- 29 Список статей
-- 30 Папка
-- 31 Список стикеров

CREATE TABLE item_lists (
    id             SERIAL PRIMARY KEY,    -- id списка записей
    name           VARCHAR(100) NOT NULL, -- название
    user_id        INT NOT NULL,          -- id пользователя (ссылка на таблицу выше)
    community_id   INT,                   -- id сообщества (ссылка на таблицу выше)
    list_id        INT NOT NULL,          -- id списка
    list_types     SMALLINT NOT NULL,     -- тип списка (выше)
    types          SMALLINT NOT NULL,     -- тип (активен, удален, закрыт...)
    image          VARCHAR(500),          -- миниатюра
    count          INT NOT NULL           -- кол-во элементов
);

-- объекты комментарий универсальные -------
-- comment_types ↓
-- 81 Коммент к записи     (не создаем, он и так есть на этом сервисе)
-- 82 Коммент к фотографии
-- 83 Коммент к ролику
-- 84 Коммент к товару
-- 85 Коммент к обсуждению
-- 86 Коммент к статье википедии
-- 87 Коммент форума

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

----------------
-- объекты записей -------
-- связь со списком нужна, так как именно список будет определять
-- приватность прикрепленных объектов.
-- значит, придется первый раз создавать и список записей...
-- (не создаем, он и так есть на этом сервисе)
-- CREATE TABLE item_posts (
--     id            SERIAL PRIMARY KEY,  -- id записи
--     content       VARCHAR(100),        -- часть содержания
--     list_id       INT NOT NULL,        -- id списка (а тип и так понятен)
--     community_id  INT,                 -- id сообщества (которое выше)
--     user_id       INT NOT NULL,        -- id пользователя (которое выше)
--     item_id       INT NOT NULL,        -- id записи (которое выше)
--     types         SMALLINT NOT NULL,   -- тип (активен, удален, закрыт...)
--     attach        VARCHAR(100),        -- прикрепленные объекты
--     created       TIMESTAMP NOT NULL,  -- время создания
--     is_signature  BOOLEAN NOT NULL     -- разрешить подпись
-- );

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

----------------------------------
-- прикрепленные копии -------
-- типы объектов (item_types) для attach_items
-- 1  Пользователь item_users
-- 2  Сообщество   item_communitys
-- 3  Сайт         item_sites
-- 20 Список записей
-- 21 Плейлист
-- 22 Список документов
-- 23 Список опросов
-- 24 Список фотографий
-- 25 Список роликов
-- 26 Список товаров
-- 27 Список обсуждений
-- 28 Список википедии
-- 29 Список статей
-- 30 Папка
-- 31 Список стикеров
-- 81 Коммент к записи
-- 82 Коммент к фотографии
-- 83 Коммент к ролику
-- 84 Коммент к товару
-- 85 Коммент к обсуждению
-- 86 Коммент к статье википедии
-- 87 Коммент форума
-- 51 Запись       item_posts
-- 52 Трек         item_audios
-- 53 Документ     item_docs
-- 54 Опрос        item_surveys
-- 55 Фотография   item_photos
-- 56 Ролик        item_videos
-- 57 Товар        item_goods
-- 58 Обсуждение   item_forums
-- 59 Статья вики  item_wikis
-- 60 Статья       item_articles

-- 101 Рабочее пространство планировщика
-- 102 Доска планировщика
-- 103 Колонка планировщика
-- 104 Карточка планировщика
-- 105 Коммент к карточке планировщика

CREATE TABLE attach_items (
    id           SERIAL PRIMARY KEY, -- id записи
    item_id      INT NOT NULL,       -- id объекта (которые выше)
    item_types   SMALLINT NOT NULL,  -- тип связанного объекта (которые выше)
    attach_types SMALLINT NOT NULL   -- к чему прикреплен объект
);
