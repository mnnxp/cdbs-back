-- Your SQL goes here
/* стандарт */
CREATE TABLE standard_ref (
  uuid UUID NOT NULL UNIQUE,
  uuid_standard_parent UUID NOT NULL, /* родительский стандарт */
  classifier VARCHAR(225) NOT NULL, /* классификатор стандарта */
  name VARCHAR(2000) NOT NULL, /* наименование стандарта */
  description VARCHAR(4000) NOT NULL, /* краткое описание стандарта */
  specified_tolerance VARCHAR(225) NOT NULL, /* степень допуска стандарта */
  technical_committee VARCHAR(225) NOT NULL, /* технический комитет */
  publication_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата публикации */
  uuid_image_file UUID NOT NULL, /* картинка стандарта */
  uuid_user UUID NOT NULL, /* идентификатор профиля загрузившего стандарт */
  uuid_company UUID NOT NULL, /* идентификатор разработавшей стандарт компании */
  id_type_access INTEGER NOT NULL, /* тип доступности стандарта */
  id_standard_status INTEGER NOT NULL, /* статус, к примеру: «актуальный», «архивный», «снято с производства» */
  id_region INTEGER NOT NULL, /* регион */
  is_delete BOOLEAN NOT NULL DEFAULT 'f', /* флаг удаления стандарта */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания/загрузки */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата обновления */
  CONSTRAINT standard_ref_pk PRIMARY KEY (uuid)
);

/* запись изменений данных стандарта */
CREATE TABLE standard_history_list (
  id SERIAL, /* id события */
  uuid_standard UUID NOT NULL, /* идентификатор стандарта к которому относится изменение */
  id_type_of_change INTEGER NOT NULL, /* id изменения (тип изменения) */
  old_data VARCHAR(2000) NOT NULL, /*  обновляемые данные данные */
  changed_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата изменения */
  CONSTRAINT standard_history_list_pk PRIMARY KEY (id)
);

/* статус стандарта */
CREATE TABLE standard_status_ref (
  id SERIAL, /* id статуса */
  CONSTRAINT standard_status_ref_pk PRIMARY KEY (id)
);

CREATE TABLE standard_status_translate_list (
  id_standard_status INTEGER NOT NULL, /* id статуса */
  id_lang INTEGER NOT NULL, /* идентификатор языка перевода */
  name VARCHAR(100) NOT NULL, /* наименование в переводе к примеру: «Стандарт опубликован» */
  UNIQUE(id_lang, name),
  CONSTRAINT standard_status_translate_list_pk PRIMARY KEY (id_standard_status, id_lang)
);

/* объект/файл стандарта */
CREATE TABLE file_to_standard (
  uuid_file UUID NOT NULL, /* идентификатор объекта/файла */
  uuid_standard UUID NOT NULL, /* идентификатор стандарта */
  CONSTRAINT file_to_standard_pk PRIMARY KEY (uuid_file, uuid_standard)
);

/* каталог стандарта */
CREATE TABLE spec_to_standard (
  id_spec INTEGER NOT NULL, /* идентификатор позиции в каталоге */
  uuid_standard UUID NOT NULL, /* идентификатор стандарта */
  CONSTRAINT spec_to_standard_pk PRIMARY KEY (id_spec, uuid_standard)
);

/* ключевые слова связанные со стандартом (тегирование) */
CREATE TABLE standard_to_keyword (
  uuid_standard UUID NOT NULL, /* идентификатор стандарта */
  id_keyword INTEGER NOT NULL, /* идентификатор ключевого слова (тега) */
  CONSTRAINT standard_to_keyword_pk PRIMARY KEY (uuid_standard, id_keyword)
);
