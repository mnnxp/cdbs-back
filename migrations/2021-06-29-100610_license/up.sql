-- Your SQL goes here
/* Лицензии */
CREATE TABLE license_ref (
  id SERIAL UNIQUE, /* id лицензии */
  name VARCHAR(225) NOT NULL UNIQUE, /* наименование лицензии */
  keyword VARCHAR(50) NOT NULL UNIQUE, /* ключевое слово лицензии */
  publication_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата публикации */
  CONSTRAINT license_ref_pk PRIMARY KEY (id)
);

/* разрешения */
CREATE TABLE license_permission_ref (
  id SERIAL UNIQUE, /* id статуса */
  id_lang INTEGER NOT NULL, /* идентификатор языка перевода */
  permission VARCHAR(225) NOT NULL, /* наименование разрешения */
  UNIQUE (id_lang, permission),
  CONSTRAINT license_permission_ref_pk PRIMARY KEY (id)
);

/* разрешения для лицензии */
CREATE TABLE permission_to_license (
  -- id SERIAL UNIQUE, /* id лицензии */
  id_permission INTEGER NOT NULL, /* идентификатор разрешения */
  id_license INTEGER NOT NULL, /* идентификатор лицензии */
  CONSTRAINT permission_to_license_pk PRIMARY KEY (id_permission, id_license)
);

/* ограничения */
CREATE TABLE license_limitation_ref (
  id SERIAL UNIQUE, /* id статуса */
  id_lang INTEGER NOT NULL, /* идентификатор языка перевода */
  limitation VARCHAR(225) NOT NULL, /* наименование разрешения */
  UNIQUE (id_lang, limitation),
  CONSTRAINT license_limitation_ref_pk PRIMARY KEY (id)
);

/* ограничения для лицензии */
CREATE TABLE limitation_to_license (
  -- id SERIAL UNIQUE, /* id лицензии */
  id_limitation INTEGER NOT NULL, /* идентификатор ограничения */
  id_license INTEGER NOT NULL, /* идентификатор лицензии */
  CONSTRAINT limitation_to_license_pk PRIMARY KEY (id_limitation, id_license)
);

/* условия */
CREATE TABLE license_condition_ref (
  id SERIAL UNIQUE, /* id статуса */
  id_lang INTEGER NOT NULL, /* идентификатор языка перевода */
  condition VARCHAR(225) NOT NULL, /* наименование разрешения */
  UNIQUE (id_lang, condition),
  CONSTRAINT license_condition_ref_pk PRIMARY KEY (id)
);

/* условия для лицензии */
CREATE TABLE condition_to_license (
  -- id SERIAL UNIQUE, /* id лицензии */
  id_condition INTEGER NOT NULL, /* идентификатор условия */
  id_license INTEGER NOT NULL, /* идентификатор лицензии */
  CONSTRAINT condition_to_license_pk PRIMARY KEY (id_condition, id_license)
);
