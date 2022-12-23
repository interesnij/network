-- Your SQL goes here

CREATE TABLE sticker_categories (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    position    SMALLINT NOT NULL DEFAULT 0,
    user_id     INT,
    description VARCHAR(200),
    avatar      VARCHAR(500)
);

CREATE TABLE stickers (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    position    SMALLINT NOT NULL DEFAULT 0,
    category_id INT NOT NULL,
    image       VARCHAR(500) NOT NULL,

    CONSTRAINT fk_stickers
        FOREIGN KEY(category_id)
            REFERENCES sticker_categories(id)
);
CREATE UNIQUE INDEX stickers_category_id_unq ON stickers (category_id);

CREATE TABLE smile_categories (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    position    SMALLINT NOT NULL DEFAULT 0,
    description VARCHAR(200)
);

CREATE TABLE smiles (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    position    SMALLINT NOT NULL DEFAULT 0,
    category_id INT NOT NULL,
    image       VARCHAR(500) NOT NULL,

    CONSTRAINT fk_smiles
        FOREIGN KEY(category_id)
            REFERENCES smile_categories(id)
);
CREATE UNIQUE INDEX smiles_category_id_unq ON smiles (category_id);


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

-------
-- все реакции -------
CREATE TABLE reactions (
  id        SERIAL PRIMARY KEY,            -- id записи
  image     VARCHAR(500) NOT NULL,         -- изображение
  gif       VARCHAR(500) NOT NULL,         -- гифка
  name      VARCHAR(100) NOT NULL,         -- название
  is_active BOOLEAN NOT NULL DEFAULT true, -- активная реакция?
  position  SMALLINT NOT NULL              -- позиция
);

CREATE TABLE custom_links (
    id   SERIAL PRIMARY KEY,
    link VARCHAR(100) NOT NULL,
    owner SMALLINT NOT NULL DEFAULT 0
);
CREATE UNIQUE INDEX custom_links_unq ON custom_links (link);
