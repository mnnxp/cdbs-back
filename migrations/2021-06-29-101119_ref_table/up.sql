-- Your SQL goes here
/* перечень CAD (и других программ) (пред.название type_cad_ref) */
CREATE TABLE program_ref (
  id SERIAL UNIQUE, /* id наименования софта */
  name VARCHAR(225) NOT NULL UNIQUE, /* наименование CAD (название программы) */
  CONSTRAINT program_ref_pk PRIMARY KEY (id)
);

/* таблица соответствия программ и расширений файлов */
CREATE TABLE extension_ref (
  id SERIAL UNIQUE, /* id соответствия */
  extension VARCHAR(50) NOT NULL, /* расширение файла, одно расширение может быть у нескольких программ */
  program_id INTEGER NOT NULL, /* соответствующая программа (CAD) */
  CONSTRAINT extension_ref_pk PRIMARY KEY (id)
);

/* информация о файле (изображении) */
CREATE TABLE file_ref (
  uuid UUID NOT NULL UNIQUE, /* идентификатор объекта/файла */
  parent_file_uuid UUID NOT NULL, /* идентификатор объекта/файла родителя */
  hash BYTEA NOT NULL, /* хеш значение объекта/файла */
  user_uuid UUID NOT NULL, /* идентификатор профиля загрузившего файл */
  filename VARCHAR(225) NOT NULL, /* наименование файла */
  content_type VARCHAR(50) NOT NULL, /* тип контента в файле */
  id_ext INTEGER NOT NULL, /* расширение файла (используется для определения CAD) */
  filesize BIGINT NOT NULL, /* размер файла */
  path_file VARCHAR(225) NOT NULL, /* путь к файлу */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания/загрузки */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата обновления */
  CONSTRAINT file_ref_pk PRIMARY KEY (uuid)
);

/* статус компонента */
CREATE TABLE actual_status_ref (
  id SERIAL, /* id статуса */
  CONSTRAINT actual_status_ref_pk PRIMARY KEY (id)
);

CREATE TABLE actual_status_translate_list (
  actual_status_id INTEGER NOT NULL, /* id статуса */
  lang_id INTEGER NOT NULL, /* идентификатор языка перевода */
  name VARCHAR(100) NOT NULL UNIQUE, /* к примеру: «актуальный», «архивный», «снято с производства» */
  UNIQUE(lang_id, name),
  CONSTRAINT actual_status_translate_list_pk PRIMARY KEY (actual_status_id, lang_id)
);

/* типы доступа */
CREATE TABLE type_access_ref (
  id SERIAL, /* id типа доступа */
  CONSTRAINT type_access_ref_pk PRIMARY KEY (id)
);

CREATE TABLE type_access_translate_list (
  type_access_id INTEGER NOT NULL, /* id типа доступа */
  lang_id INTEGER NOT NULL, /* идентификатор языка перевода */
  name VARCHAR(100) NOT NULL, /* наименование доступа */
  UNIQUE(lang_id, name),
  CONSTRAINT type_access_translate_list_pk PRIMARY KEY (type_access_id, lang_id)
);

/* языки перевода */
CREATE TABLE language_ref (
  id SERIAL UNIQUE, /* id языка */
  lang VARCHAR(100) NOT NULL UNIQUE, /* полное наименование языка */
  langshort VARCHAR(10) NOT NULL UNIQUE, /* краткое наименование языка */
  CONSTRAINT language_ref_pk PRIMARY KEY (id)
);

/* параметры для модификации */
CREATE TABLE param_ref (
  id SERIAL, /* id параметра (характеристики) */
  CONSTRAINT param_ref_pk PRIMARY KEY (id)
);

CREATE TABLE param_translate_list (
  param_id INTEGER NOT NULL, /* id параметра (характеристики) */
  lang_id INTEGER NOT NULL, /* идентификатор языка перевода */
  paramname VARCHAR(100) NOT NULL, /* наименование парметра модификации в переводе */
  UNIQUE(lang_id, paramname),
  CONSTRAINT param_translate_list_pk PRIMARY KEY (param_id, lang_id)
);

/* регион */
CREATE TABLE region_ref (
  id SERIAL, /* id наименования региона */
  CONSTRAINT region_ref_pk PRIMARY KEY (id)
);

CREATE TABLE region_translate_list (
  region_id INTEGER NOT NULL, /* id наименования региона */
  lang_id INTEGER NOT NULL, /* идентификатор языка перевода */
  region VARCHAR(100) NOT NULL, /* наименование региона в переводе */
  UNIQUE(lang_id, region),
  CONSTRAINT region_translate_list_pk PRIMARY KEY (region_id, lang_id)
);

/* перечень типов изменений */
CREATE TABLE type_of_change_ref (
  id SERIAL, /* id типа изменения */
  CONSTRAINT type_of_change_ref_pk PRIMARY KEY (id)
);

CREATE TABLE type_of_change_translate_list (
  type_of_change_id INTEGER NOT NULL, /* id типа изменения */
  lang_id INTEGER NOT NULL, /* идентификатор языка перевода */
  type_of_change VARCHAR(100) NOT NULL, /*наименование изменения */
  CONSTRAINT type_of_change_translate_list_pk PRIMARY KEY (type_of_change_id, lang_id)
);

/* категории (каталога) */
CREATE TABLE spec_ref (
  id SERIAL, /* id категории каталога */
  parent_spec_id INTEGER NOT NULL DEFAULT '1', /* id родительского каталога */
  UNIQUE(id, parent_spec_id),
  CONSTRAINT spec_ref_pk PRIMARY KEY (id)
);

CREATE TABLE spec_translate_list (
  spec_id INTEGER NOT NULL, /* id категории каталога */
  lang_id INTEGER NOT NULL, /* идентификатор языка перевода */
  spec VARCHAR(225) NOT NULL, /* наименование каталога в переводе */
  UNIQUE(spec_id, lang_id, spec),
  CONSTRAINT spec_translate_list_pk PRIMARY KEY (spec_id, lang_id)
);

/* ключевые слова (тегирование) */
CREATE TABLE keyword_ref (
  id SERIAL UNIQUE, /* id тега */
  keyword VARCHAR(10) NOT NULL UNIQUE, /* ключевое слово */
  CONSTRAINT keyword_ref_pk PRIMARY KEY (id)
);
