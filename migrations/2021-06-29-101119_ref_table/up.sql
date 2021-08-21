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
  id_program INTEGER NOT NULL, /* соответствующая программа (CAD) */
  CONSTRAINT extension_ref_pk PRIMARY KEY (id)
);

/* информация о файле (изображении) */
CREATE TABLE file_ref (
  uuid UUID NOT NULL UNIQUE, /* идентификатор объекта/файла */
  uuid_file_parent UUID NOT NULL, /* идентификатор объекта/файла родителя */
  hash BYTEA NOT NULL, /* хеш значение объекта/файла */
  uuid_user UUID NOT NULL, /* идентификатор профиля загрузившего файл */
  filename VARCHAR(225) NOT NULL, /* наименование файла */
  content_type VARCHAR(50) NOT NULL, /* тип контента в файле */
  id_ext INTEGER NOT NULL, /* расширение файла (используется для определения CAD) */
  filesize INTEGER NOT NULL, /* размер файла */
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
  id_actual_status INTEGER NOT NULL, /* id статуса */
  id_lang INTEGER NOT NULL, /* идентификатор языка перевода */
  name VARCHAR(100) NOT NULL UNIQUE, /* к примеру: «актуальный», «архивный», «снято с производства» */
  UNIQUE(id_lang, name),
  CONSTRAINT actual_status_translate_list_pk PRIMARY KEY (id_actual_status, id_lang)
);

/* типы доступа */
CREATE TABLE type_access_ref (
  id SERIAL, /* id типа доступа */
  CONSTRAINT type_access_ref_pk PRIMARY KEY (id)
);

CREATE TABLE type_access_translate_list (
  id_type_access INTEGER NOT NULL, /* id типа доступа */
  id_lang INTEGER NOT NULL, /* идентификатор языка перевода */
  name VARCHAR(100) NOT NULL, /* наименование доступа */
  UNIQUE(id_lang, name),
  CONSTRAINT type_access_translate_list_pk PRIMARY KEY (id_type_access, id_lang)
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
  id_param INTEGER NOT NULL, /* id параметра (характеристики) */
  id_lang INTEGER NOT NULL, /* идентификатор языка перевода */
  paramname VARCHAR(100) NOT NULL, /* наименование парметра модификации в переводе */
  UNIQUE(id_lang, paramname),
  CONSTRAINT param_translate_list_pk PRIMARY KEY (id_param, id_lang)
);

/* регион */
CREATE TABLE region_ref (
  id SERIAL, /* id наименования региона */
  CONSTRAINT region_ref_pk PRIMARY KEY (id)
);

CREATE TABLE region_translate_list (
  id_region INTEGER NOT NULL, /* id наименования региона */
  id_lang INTEGER NOT NULL, /* идентификатор языка перевода */
  region VARCHAR(100) NOT NULL, /* наименование региона в переводе */
  UNIQUE(id_lang, region),
  CONSTRAINT region_translate_list_pk PRIMARY KEY (id_region, id_lang)
);

/* перечень типов изменений */
CREATE TABLE type_of_change_ref (
  id SERIAL, /* id типа изменения */
  CONSTRAINT type_of_change_ref_pk PRIMARY KEY (id)
);

CREATE TABLE type_of_change_translate_list (
  id_type_of_change INTEGER NOT NULL, /* id типа изменения */
  id_lang INTEGER NOT NULL, /* идентификатор языка перевода */
  type_of_change VARCHAR(100) NOT NULL, /*наименование изменения */
  CONSTRAINT type_of_change_translate_list_pk PRIMARY KEY (id_type_of_change, id_lang)
);

/* категории (каталога) */
CREATE TABLE spec_ref (
  id SERIAL, /* id категории каталога */
  id_spec_parent INTEGER NOT NULL DEFAULT '1', /* id родительского каталога */
  UNIQUE(id, id_spec_parent),
  CONSTRAINT spec_ref_pk PRIMARY KEY (id)
);

CREATE TABLE spec_translate_list (
  id_spec INTEGER NOT NULL, /* id категории каталога */
  id_lang INTEGER NOT NULL, /* идентификатор языка перевода */
  spec VARCHAR(225) NOT NULL, /* наименование каталога в переводе */
  UNIQUE(id_spec, id_lang, spec),
  CONSTRAINT spec_translate_list_pk PRIMARY KEY (id_spec, id_lang)
);

/* ключевые слова (тегирование) */
CREATE TABLE keyword_ref (
  id SERIAL UNIQUE, /* id тега */
  keyword VARCHAR(10) NOT NULL UNIQUE, /* ключевое слово */
  CONSTRAINT keyword_ref_pk PRIMARY KEY (id)
);
