-- Your SQL goes here
/* перечень CAD (и других программ) (пред.название type_cad_ref) */
CREATE TABLE program_ref (
  id SERIAL, /* id наименования софта */
  name VARCHAR(225) NOT NULL UNIQUE, /* наименование CAD (название программы) */
  CONSTRAINT program_ref_pk PRIMARY KEY (id)
);

/* таблица соответствия программ и расширений файлов */
CREATE TABLE extension_ref (
  id SERIAL, /* id соответствия */
  extension VARCHAR(50) NOT NULL, /* расширение файла, одно расширение может быть у нескольких программ */
  id_program INTEGER NOT NULL, /* соответствующая программа (CAD) */
  CONSTRAINT extension_ref_pk PRIMARY KEY (id)
);

/* информация о файле (изображении) */
CREATE TABLE file_ref (
  id SERIAL, /* id файла */
  uuid UUID NOT NULL UNIQUE, /* идентификатор объекта/файла */
  uuid_file_parent UUID NOT NULL, /* идентификатор объекта/файла родителя */
  hash BYTEA NOT NULL, /* хеш значение объекта/файла */
  uuid_user UUID NOT NULL, /* идентификатор профиля загрузившего файл */
  filename VARCHAR(225) NOT NULL, /* наименование файла */
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
  name VARCHAR(100) NOT NULL UNIQUE, /* к примеру: «актуальный», «архивный», «снято с производства» */
  CONSTRAINT actual_status_ref_pk PRIMARY KEY (id)
);

/* типы доступа */
CREATE TABLE type_access_ref (
  id SERIAL, /* id типа доступа */
  name VARCHAR(100) NOT NULL UNIQUE, /* наименование доступа */
  CONSTRAINT type_access_ref_pk PRIMARY KEY (id)
);

/* параметры для модификации */
CREATE TABLE param_ref (
  id SERIAL, /* id параметра (характеристики) */
  paramname VARCHAR(100) NOT NULL UNIQUE, /* наименование парметра модификации*/
  CONSTRAINT param_ref_pk PRIMARY KEY (id)
);

/* языки перевода */
CREATE TABLE language_ref (
  id SERIAL, /* id языка */
  lang VARCHAR(100) NOT NULL UNIQUE, /* полное наименование языка */
  langshort VARCHAR(10) NOT NULL UNIQUE, /* краткое наименование языка */
  CONSTRAINT language_ref_pk PRIMARY KEY (id)
);

/* перевод параметра */
CREATE TABLE param_translate_list (
  id SERIAL, /* id перевода */
  id_param INTEGER NOT NULL, /* идентификатор параметра */
  id_lang INTEGER NOT NULL, /* идентификатор языка перевода */
  param VARCHAR(100) NOT NULL, /* перевод параметра */
  CONSTRAINT param_translate_list_pk PRIMARY KEY (id)
);

/* перевод раздела каталога */
CREATE TABLE spec_translate_list (
  id SERIAL, /* id перевода */
  id_spec INTEGER NOT NULL, /* идентификатор раздела */
  id_lang INTEGER NOT NULL, /* идентификатор языка перевода */
  spec VARCHAR(255) NOT NULL, /* перевод раздела */
  CONSTRAINT spec_translate_list_pk PRIMARY KEY (id)
);

/* регион */
CREATE TABLE region_ref (
  id SERIAL, /* id наименования региона */
  region VARCHAR(100) NOT NULL, /* наименование региона */
  CONSTRAINT region_ref_pk PRIMARY KEY (id)
);

/* перечень типов изменений */
CREATE TABLE type_of_change_ref (
  id SERIAL, /* id типа изменения */
  type_of_change VARCHAR(100) NOT NULL, /*наименование изменения */
  CONSTRAINT type_of_change_ref_pk PRIMARY KEY (id)
);
