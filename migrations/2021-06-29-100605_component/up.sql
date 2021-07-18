-- Your SQL goes here
/* компонент */
CREATE TABLE component_ref (
  id SERIAL UNIQUE, /* id компонента */
  uuid UUID NOT NULL UNIQUE,
  uuid_component_parent UUID NOT NULL, /* родительский компонент */
  name VARCHAR(225) NOT NULL, /* наименование компонента */
  description VARCHAR(2000) NOT NULL, /* краткое описание компонента */
  uuid_user UUID NOT NULL, /* идентификатор профиля загрузившего компонент */
  id_type_access INTEGER NOT NULL, /* доступност к компоненту по умолчанию */
  id_component_type INTEGER NOT NULL, /* тип компонента */
  id_actual_status INTEGER NOT NULL, /* номер статуса, к примеру: «актуальный», «архивный», «снято с производства» */
  is_standard BOOLEAN NOT NULL DEFAULT 'f', /* компонент соответствует стандарту */
  is_delete BOOLEAN NOT NULL DEFAULT 'f', /* флаг удаления компонента */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания/загрузки */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата обновления */
  -- UNIQUE (name, uuid_user),
  CONSTRAINT component_ref_pk PRIMARY KEY (uuid)
);

/* запись изменений данных компонента */
CREATE TABLE component_history_list (
  id SERIAL UNIQUE, /* id события */
  uuid_component UUID NOT NULL, /* идентификатор стандарта к которому относится изменение */
  id_type_of_change INTEGER NOT NULL, /* id изменения (тип изменения) */
  old_data VARCHAR(2000) NOT NULL, /*  обновляемые данные данные */
  changed_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата изменения */
  CONSTRAINT component_history_list_pk PRIMARY KEY (id)
);

/* тип компонента (базовый, кастомный) */
CREATE TABLE component_type_ref (
  id SERIAL UNIQUE, /* id типа */
  component_type VARCHAR(100) NOT NULL UNIQUE, /* наименование типа */
  CONSTRAINT component_type_ref_pk PRIMARY KEY (id)
);

/* ключевые слова компонента (тегирование) */
CREATE TABLE component_keyword_ref (
  id SERIAL UNIQUE, /* id тега */
  keyword VARCHAR(10) NOT NULL UNIQUE, /* ключевое слово */
  CONSTRAINT component_keyword_ref_pk PRIMARY KEY (id)
);

/* ключевые слова связанные с компонентом (тегирование) */
CREATE TABLE component_to_keyword (
  id SERIAL UNIQUE, /* id связи тега и компонента */
  uuid_component UUID NOT NULL, /* идентификатор компонента */
  id_component_keyword INTEGER NOT NULL, /* идентификатор ключевого слова (тега) */
  CONSTRAINT component_to_keyword_pk PRIMARY KEY (id)
);

/* параметр компонента */
CREATE TABLE param_to_component (
  id SERIAL UNIQUE, /* id параметра компонента */
  uuid_component UUID NOT NULL, /* идентификатор компонента */
  id_param INTEGER NOT NULL, /* идентификатор параметра */
  value VARCHAR(100) NOT NULL, /* параметр компонента */
  CONSTRAINT param_to_component_pk PRIMARY KEY (id)
);

/* список поставщиков компонента (list shippers) */
CREATE TABLE supplier_to_component (
  id SERIAL UNIQUE, /* id записи профиля в поставщики компонента */
  uuid_component UUID NOT NULL, /* идентификатор компонента */
  uuid_company UUID NOT NULL, /* идентификатор компании-поставщика */
  description VARCHAR(255) NOT NULL, /* комментарий к поставщику */
  CONSTRAINT supplier_to_component_pk PRIMARY KEY (id)
);

/* обсуждение компонента */
CREATE TABLE discussion_component_ref (
  id SERIAL UNIQUE, /* id комментария */
  id_discussion_parent INTEGER NOT NULL, /* id родительского комментария */
  uuid_component UUID NOT NULL, /* идентификатор обсуждаемого компонента */
  uuid_author UUID NOT NULL, /* идентификатор профиля отправителя */
  message_content VARCHAR(4000) NOT NULL, /* сообщение/комментарий */
  is_delete BOOLEAN NOT NULL DEFAULT 'f', /* флаг удаления */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания/редактирования */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата обновления */
  CONSTRAINT discussion_component_ref_pk PRIMARY KEY (id)
);

/* Список модификаций компонента */
CREATE TABLE component_modification_list (
  id SERIAL UNIQUE, /* id компонента */
  uuid UUID NOT NULL UNIQUE,
  uuid_component UUID NOT NULL, /* идентификатор компонента */
  uuid_modification_parent UUID NOT NULL, /* родительская модификация */
  modification_name VARCHAR(100) NOT NULL, /* наименование модификации */
  description VARCHAR(2000) NOT NULL, /*  комментарий к модификации */
  id_actual_status INTEGER NOT NULL, /* номер статуса, к примеру: «актуальный», «архивный», «снято с производства» */
  is_delete BOOLEAN NOT NULL DEFAULT 'f', /* флаг удаления компонента */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания/загрузки */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата обновления */
  -- UNIQUE (uuid_component, modification_name, uuid_modification_parent),
  CONSTRAINT component_modification_list_pk PRIMARY KEY (uuid)
);

/* параметр модификации */
CREATE TABLE param_to_modification (
  id SERIAL UNIQUE, /* id параметра модификации */
  uuid_modification UUID NOT NULL, /* идентификатор модификации */
  id_param INTEGER NOT NULL, /* идентификатор параметра */
  value VARCHAR(255) NOT NULL, /* параметр компонента */
  CONSTRAINT param_to_modification_pk PRIMARY KEY (id)
);

/* объект/файл компонента */
CREATE TABLE file_to_component (
  id SERIAL UNIQUE, /* id файла компонента */
  uuid_file UUID NOT NULL, /* идентификатор объекта/файла */
  uuid_component UUID NOT NULL, /* идентификатор компонента */
  UNIQUE (uuid_file, uuid_component),
  CONSTRAINT file_to_component_pk PRIMARY KEY (id)
);

/* объект/файл модификации */
CREATE TABLE file_to_modification (
  id SERIAL UNIQUE, /* id файла модификации */
  uuid_file UUID NOT NULL, /* идентификатор объекта/файла */
  uuid_modification UUID NOT NULL, /* идентификатор модификации */
  UNIQUE (uuid_file, uuid_modification),
  CONSTRAINT file_to_modification_pk PRIMARY KEY (id)
);

/* набор файлов модификации (файлы под САПР) */
CREATE TABLE set_files_for_program (
  id SERIAL UNIQUE, /* идентификатор набора объектов/файлов */
  uuid_modification UUID NOT NULL, /* идентификатор модификации */
  id_program INTEGER NOT NULL, /* САПР (для быстрой загрузки данных) */
  UNIQUE (uuid_modification, id_program), /* один набор файлов модификации для одного САПРа */
  CONSTRAINT set_files_for_program_pk PRIMARY KEY (id)
);

/* файлы набора модификации (файлы под САПР) */
CREATE TABLE file_to_set_modification (
  id SERIAL UNIQUE, /* id файла модификации */
  id_set INTEGER NOT NULL, /* идентификатор набора */
  uuid_file UUID NOT NULL, /* идентификатор объекта/файла */
  UNIQUE (id_set, uuid_file), /* один набор файлов модификации для одного САПРа */
  CONSTRAINT file_to_set_modification_pk PRIMARY KEY (id)
);

/* каталог компонента */
CREATE TABLE spec_to_component (
  id SERIAL UNIQUE, /* id связи компонента с каталогом */
  id_spec INTEGER NOT NULL, /* идентификатор позиции в каталоге */
  uuid_component UUID NOT NULL, /* идентификатор компонента */
  UNIQUE (id_spec, uuid_component),
  CONSTRAINT spec_to_component_pk PRIMARY KEY (id)
);

/* лицензия компонента */
CREATE TABLE license_to_component (
  id SERIAL UNIQUE, /* id связи компонента с лицензией */
  uuid_component UUID NOT NULL, /* идентификатор компонента */
  id_license INTEGER NOT NULL, /* идентификатор лицензии  */
  UNIQUE (uuid_component, id_license),
  CONSTRAINT license_to_component_pk PRIMARY KEY (id)
);

/* стандарт компонента */
CREATE TABLE standard_to_component (
  id SERIAL UNIQUE, /* id связи компонента со стандартом */
  uuid_standard UUID NOT NULL, /* идентификатор стандарта  */
  uuid_component UUID NOT NULL, /* идентификатор компонента */
  UNIQUE (uuid_standard, uuid_component),
  CONSTRAINT standard_to_component_pk PRIMARY KEY (id)
);
