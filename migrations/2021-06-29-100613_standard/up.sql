-- Your SQL goes here
/* стандарт */
CREATE TABLE standard_ref (
  uuid UUID NOT NULL UNIQUE,
  parent_standard_uuid UUID NOT NULL, /* родительский стандарт */
  classifier VARCHAR(225) NOT NULL, /* классификатор стандарта */
  name VARCHAR(2000) NOT NULL, /* наименование стандарта */
  description VARCHAR(4000) NOT NULL, /* краткое описание стандарта */
  specified_tolerance VARCHAR(225) NOT NULL, /* степень допуска стандарта */
  technical_committee VARCHAR(225) NOT NULL, /* технический комитет */
  publication_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата публикации */
  image_file_uuid UUID NOT NULL, /* картинка стандарта */
  user_uuid UUID NOT NULL, /* идентификатор профиля загрузившего стандарт */
  company_uuid UUID NOT NULL, /* идентификатор разработавшей стандарт компании */
  type_access_id INTEGER NOT NULL, /* тип доступности стандарта */
  standard_status_id INTEGER NOT NULL, /* статус, к примеру: «актуальный», «архивный», «снято с производства» */
  region_id INTEGER NOT NULL, /* регион */
  is_delete BOOLEAN NOT NULL DEFAULT 'f', /* флаг удаления стандарта */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания/загрузки */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата обновления */
  CONSTRAINT standard_ref_pk PRIMARY KEY (uuid)
);

/* запись изменений данных стандарта */
CREATE TABLE standard_history_list (
  id SERIAL, /* id события */
  standard_uuid UUID NOT NULL, /* идентификатор стандарта к которому относится изменение */
  type_of_change_id INTEGER NOT NULL, /* id изменения (тип изменения) */
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
  standard_status_id INTEGER NOT NULL, /* id статуса */
  lang_id INTEGER NOT NULL, /* идентификатор языка перевода */
  name VARCHAR(100) NOT NULL, /* наименование в переводе к примеру: «Стандарт опубликован» */
  UNIQUE(lang_id, name),
  CONSTRAINT standard_status_translate_list_pk PRIMARY KEY (standard_status_id, lang_id)
);

/* объект/файл стандарта */
CREATE TABLE file_to_standard (
  file_uuid UUID NOT NULL, /* идентификатор объекта/файла */
  standard_uuid UUID NOT NULL, /* идентификатор стандарта */
  CONSTRAINT file_to_standard_pk PRIMARY KEY (file_uuid, standard_uuid)
);

/* каталог стандарта */
CREATE TABLE spec_to_standard (
  spec_id INTEGER NOT NULL, /* идентификатор позиции в каталоге */
  standard_uuid UUID NOT NULL, /* идентификатор стандарта */
  CONSTRAINT spec_to_standard_pk PRIMARY KEY (spec_id, standard_uuid)
);

/* ключевые слова связанные со стандартом (тегирование) */
CREATE TABLE keyword_to_standard (
  standard_uuid UUID NOT NULL, /* идентификатор стандарта */
  keyword_id INTEGER NOT NULL, /* идентификатор ключевого слова (тега) */
  CONSTRAINT keyword_to_standard_pk PRIMARY KEY (standard_uuid, keyword_id)
);
