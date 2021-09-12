-- Your SQL goes here
/* Лицензии */
CREATE TABLE license_ref (
  id SERIAL, /* id лицензии */
  name VARCHAR(225) NOT NULL UNIQUE, /* наименование лицензии */
  keyword VARCHAR(50) NOT NULL UNIQUE, /* ключевое слово лицензии */
  publication_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата публикации */
  CONSTRAINT license_ref_pk PRIMARY KEY (id)
);

/* разрешения */
CREATE TABLE license_permission_ref (
  id SERIAL, /* id статуса */
  CONSTRAINT license_permission_ref_pk PRIMARY KEY (id)
);

CREATE TABLE license_permission_translate_list (
  permission_license_id INTEGER NOT NULL, /* id статуса */
  lang_id INTEGER NOT NULL, /* идентификатор языка перевода */
  permission VARCHAR(225) NOT NULL, /* наименование разрешения */
  UNIQUE (lang_id, permission),
  CONSTRAINT license_permission_translate_list_pk PRIMARY KEY (
    permission_license_id,
    lang_id
  )
);

/* разрешения для лицензии */
CREATE TABLE permission_to_license (
  permission_id INTEGER NOT NULL, /* идентификатор разрешения */
  license_id INTEGER NOT NULL, /* идентификатор лицензии */
  CONSTRAINT permission_to_license_pk PRIMARY KEY (permission_id, license_id)
);

/* ограничения */
CREATE TABLE license_limitation_ref (
  id SERIAL, /* id статуса */
  CONSTRAINT license_limitation_ref_pk PRIMARY KEY (id)
);

CREATE TABLE license_limitation_translate_list (
  limitation_license_id INTEGER NOT NULL, /* id статуса */
  lang_id INTEGER NOT NULL, /* идентификатор языка перевода */
  limitation VARCHAR(225) NOT NULL, /* наименование разрешения */
  UNIQUE (lang_id, limitation),
  CONSTRAINT license_limitation_translate_list_pk PRIMARY KEY (
    limitation_license_id,
    lang_id
  )
);

/* ограничения для лицензии */
CREATE TABLE limitation_to_license (
  limitation_id INTEGER NOT NULL, /* идентификатор ограничения */
  license_id INTEGER NOT NULL, /* идентификатор лицензии */
  CONSTRAINT limitation_to_license_pk PRIMARY KEY (limitation_id, license_id)
);

/* условия */
CREATE TABLE license_condition_ref (
  id SERIAL, /* id статуса */
  CONSTRAINT license_condition_ref_pk PRIMARY KEY (id)
);

CREATE TABLE license_condition_translate_list (
  condition_license_id INTEGER NOT NULL, /* id статуса */
  lang_id INTEGER NOT NULL, /* идентификатор языка перевода */
  condition VARCHAR(225) NOT NULL, /* наименование разрешения */
  UNIQUE (lang_id, condition),
  CONSTRAINT license_condition_translate_list_pk PRIMARY KEY (
    condition_license_id,
    lang_id
  )
);

/* условия для лицензии */
CREATE TABLE condition_to_license (
  condition_id INTEGER NOT NULL, /* идентификатор условия */
  license_id INTEGER NOT NULL, /* идентификатор лицензии */
  CONSTRAINT condition_to_license_pk PRIMARY KEY (condition_id, license_id)
);
