-- Your SQL goes here
/* компонент */
CREATE TABLE component_ref (
  uuid UUID NOT NULL UNIQUE,
  parent_component_uuid UUID NOT NULL, /* родительский компонент */
  name VARCHAR(225) NOT NULL, /* наименование компонента */
  description VARCHAR(2000) NOT NULL, /* краткое описание компонента */
  user_uuid UUID NOT NULL, /* идентификатор профиля загрузившего компонент */
  type_access_id INTEGER NOT NULL, /* доступност к компоненту по умолчанию */
  component_type_id INTEGER NOT NULL, /* тип компонента */
  actual_status_id INTEGER NOT NULL, /* номер статуса, к примеру: «актуальный», «архивный», «снято с производства» */
  is_standard BOOLEAN NOT NULL DEFAULT 'f', /* компонент соответствует стандарту */
  is_delete BOOLEAN NOT NULL DEFAULT 'f', /* флаг удаления компонента */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания/загрузки */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата обновления */
  CONSTRAINT component_ref_pk PRIMARY KEY (uuid)
);

/* запись изменений данных компонента */
CREATE TABLE component_history_list (
  id SERIAL UNIQUE, /* id события */
  component_uuid UUID NOT NULL, /* идентификатор стандарта к которому относится изменение */
  type_of_change_id INTEGER NOT NULL, /* id изменения (тип изменения) */
  old_data VARCHAR(2000) NOT NULL, /*  обновляемые данные данные */
  changed_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата изменения */
  CONSTRAINT component_history_list_pk PRIMARY KEY (id)
);

/* тип компонента (базовый, кастомный) */
CREATE TABLE component_type_ref (
  id SERIAL, /* id типа */
  CONSTRAINT component_type_ref_pk PRIMARY KEY (id)
);

CREATE TABLE component_type_translate_list (
  component_type_id INTEGER NOT NULL, /* id типа */
  lang_id INTEGER NOT NULL, /* идентификатор языка перевода */
  component_type VARCHAR(100) NOT NULL, /* наименование типа */
  UNIQUE (lang_id, component_type),
  CONSTRAINT component_type_translate_list_pk PRIMARY KEY (component_type_id, lang_id)
);

/* ключевые слова связанные с компонентом (тегирование) */
CREATE TABLE keyword_to_component (
  component_uuid UUID NOT NULL, /* идентификатор компонента */
  keyword_id INTEGER NOT NULL, /* идентификатор ключевого слова (тега) */
  CONSTRAINT keyword_to_component_pk PRIMARY KEY (component_uuid, keyword_id)
);

/* параметр компонента */
CREATE TABLE param_to_component (
  component_uuid UUID NOT NULL, /* идентификатор компонента */
  param_id INTEGER NOT NULL, /* идентификатор параметра */
  value VARCHAR(100) NOT NULL, /* параметр компонента */
  CONSTRAINT param_to_component_pk PRIMARY KEY (component_uuid, param_id)
);

/* список поставщиков компонента (list shippers) */
CREATE TABLE supplier_to_component (
  component_uuid UUID NOT NULL, /* идентификатор компонента */
  company_uuid UUID NOT NULL, /* идентификатор компании-поставщика */
  description VARCHAR(255) NOT NULL, /* комментарий к поставщику */
  CONSTRAINT supplier_to_component_pk PRIMARY KEY (component_uuid, company_uuid)
);

/* обсуждение компонента */
CREATE TABLE discussion_component_ref (
  id SERIAL UNIQUE, /* id комментария */
  parent_discussion_id INTEGER NOT NULL, /* id родительского комментария */
  component_uuid UUID NOT NULL, /* идентификатор обсуждаемого компонента */
  author_uuid UUID NOT NULL, /* идентификатор профиля отправителя */
  message_content VARCHAR(4000) NOT NULL, /* сообщение/комментарий */
  is_delete BOOLEAN NOT NULL DEFAULT 'f', /* флаг удаления */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания/редактирования */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата обновления */
  CONSTRAINT discussion_component_ref_pk PRIMARY KEY (id)
);

/* Список модификаций компонента */
CREATE TABLE component_modification_list (
  uuid UUID NOT NULL UNIQUE,
  component_uuid UUID NOT NULL, /* идентификатор компонента */
  parent_modification_uuid UUID NOT NULL, /* родительская модификация */
  modification_name VARCHAR(100) NOT NULL, /* наименование модификации */
  description VARCHAR(2000) NOT NULL, /*  комментарий к модификации */
  actual_status_id INTEGER NOT NULL, /* номер статуса, к примеру: «актуальный», «архивный», «снято с производства» */
  is_delete BOOLEAN NOT NULL DEFAULT 'f', /* флаг удаления компонента */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания/загрузки */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата обновления */
  CONSTRAINT component_modification_list_pk PRIMARY KEY (uuid)
);

/* параметр модификации */
CREATE TABLE param_to_modification (
  modification_uuid UUID NOT NULL, /* идентификатор модификации */
  param_id INTEGER NOT NULL, /* идентификатор параметра */
  value VARCHAR(255) NOT NULL, /* параметр модификации */
  CONSTRAINT param_to_modification_pk PRIMARY KEY (modification_uuid, param_id)
);

/* объект/файл компонента */
CREATE TABLE file_to_component (
  file_uuid UUID NOT NULL, /* идентификатор объекта/файла */
  component_uuid UUID NOT NULL, /* идентификатор компонента */
  CONSTRAINT file_to_component_pk PRIMARY KEY (file_uuid, component_uuid)
);

/* объект/файл модификации */
CREATE TABLE file_to_modification (
  file_uuid UUID NOT NULL, /* идентификатор объекта/файла */
  modification_uuid UUID NOT NULL, /* идентификатор модификации */
  CONSTRAINT file_to_modification_pk PRIMARY KEY (file_uuid, modification_uuid)
);

/* набор файлов модификации (файлы под САПР) */
CREATE TABLE fileset_for_program (
  uuid UUID, /* идентификатор набора объектов/файлов */
  modification_uuid UUID NOT NULL, /* идентификатор модификации */
  program_id INTEGER NOT NULL, /* САПР (для быстрой загрузки данных) */
  UNIQUE (modification_uuid, program_id), /* один набор файлов модификации для одного САПРа */
  CONSTRAINT fileset_for_program_pk PRIMARY KEY (uuid)
);

/* файлы набора модификации (файлы под САПР) */
CREATE TABLE file_of_modification_set (
  fileset_uuid UUID NOT NULL, /* идентификатор набора */
  file_uuid UUID NOT NULL, /* идентификатор объекта/файла */
  CONSTRAINT file_of_modification_set_pk PRIMARY KEY (fileset_uuid, file_uuid)
);

/* каталог компонента */
CREATE TABLE spec_to_component (
  spec_id INTEGER NOT NULL, /* идентификатор позиции в каталоге */
  component_uuid UUID NOT NULL, /* идентификатор компонента */
  CONSTRAINT spec_to_component_pk PRIMARY KEY (spec_id, component_uuid)
);

/* лицензия компонента */
CREATE TABLE license_to_component (
  component_uuid UUID NOT NULL, /* идентификатор компонента */
  license_id INTEGER NOT NULL, /* идентификатор лицензии  */
  CONSTRAINT license_to_component_pk PRIMARY KEY (component_uuid, license_id)
);

/* стандарт компонента */
CREATE TABLE standard_to_component (
  standard_uuid UUID NOT NULL, /* идентификатор стандарта  */
  component_uuid UUID NOT NULL, /* идентификатор компонента */
  CONSTRAINT standard_to_component_pk PRIMARY KEY (standard_uuid, component_uuid)
);
