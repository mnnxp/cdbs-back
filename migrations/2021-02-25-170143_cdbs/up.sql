/* профиль */
CREATE TABLE user_ref (
  id SERIAL, /* id профиля */
  uuid UUID NOT NULL UNIQUE PRIMARY KEY,
  email VARCHAR(100) NOT NULL, /*email профиля, на один адрес может быть несколько профилей (закос под reddit) */
  psw_hash BYTEA NOT NULL, /* хеш пароля профиля */
  psw_salt VARCHAR(255) NOT NULL, /* соль для пароля профиля */
  firstname VARCHAR(100) NOT NULL, /*Имя */
  lastname VARCHAR(100) NOT NULL, /*Фамилия */
  secondname VARCHAR(100) NOT NULL, /*Отчество */
  username VARCHAR(100) NOT NULL UNIQUE, /* ник профиля (может использоваться для авторизации) */
  phone VARCHAR(100) NOT NULL, /*номер телефона */
  description VARCHAR(2000) NOT NULL, /* информация для связи, подпись */
  address VARCHAR(512) NOT NULL, /*почтовый адрес */
  position VARCHAR(255) NOT NULL, /*роль/должность */
  time_zone INTEGER NOT NULL, /*часовой пояс профиля */
  uuid_image_file UUID NOT NULL, /* картинка пользователя */
  id_region INTEGER NOT NULL, /* регион */
  id_programm INTEGER NOT NULL DEFAULT '1', /* САПР «по умолчанию» (для быстрой загрузки данных) */
  is_email_verified BOOLEAN NOT NULL DEFAULT 'f', /* подтверждение email */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /* флаг активности пользователь */
  is_delete BOOLEAN NOT NULL DEFAULT 'f', /* флаг удаления пользователя */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания профиля */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW() /* дата обновления профиля */
);

/* токен сессии клиента */
CREATE TABLE user_tokens_ref (
  id SERIAL, /* id токена */
  uuid_user UUID NOT NULL, /* идентификатор пользователя */
  token VARCHAR(512) NOT NULL, /* токен пользователя */
  date_start TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания токена */
  date_end TIMESTAMP NOT NULL DEFAULT NOW(), /* дата окончания действия токена */
  is_enabled BOOLEAN NOT NULL DEFAULT 't',
  CONSTRAINT user_tokens_ref_pk PRIMARY KEY (id)
);

/* компания */
CREATE TABLE company_ref (
  id SERIAL, /* id компании */
  uuid UUID NOT NULL UNIQUE PRIMARY KEY,
  orgname VARCHAR(255) NOT NULL, /* наименование организации (для юр.лиц) */
  shortname VARCHAR(255) NOT NULL, /* сокращённое наименование организации (для юр.лиц) */
  inn VARCHAR(30) NOT NULL, /*инн компании */
  phone VARCHAR(100) NOT NULL, /*номер телефона */
  email VARCHAR(100) NOT NULL, /* email компании */
  description VARCHAR(4000) NOT NULL, /* информация, описание */
  address VARCHAR(512) NOT NULL, /*почтовый адрес */
  site_url VARCHAR(255) NOT NULL, /* URL адрес сайта компании */
  time_zone INTEGER NOT NULL, /*часовой пояс компании */
  uuid_user UUID NOT NULL, /* uuuid профиля - владельца */
  uuid_image_file UUID NOT NULL, /* логотип компании */
  id_region INTEGER NOT NULL, /* регион */
  id_type_org INTEGER NOT NULL DEFAULT '1', /* тип компании (ао, пао, ооо, ип) */
  is_supplier BOOLEAN NOT NULL DEFAULT 'f',  /* роль пользователя: поставщик/заказчик */
  is_email_verified BOOLEAN NOT NULL DEFAULT 'f', /* подтверждение email */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /* флаг активности пользователь */
  is_delete BOOLEAN NOT NULL DEFAULT 'f', /* флаг удаления пользователя */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания компании */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW() /* дата обновления компании */
);

/* тип компании */
CREATE TABLE type_company_ref (
  id SERIAL, /* id типа компании*/
  name VARCHAR(255) NOT NULL UNIQUE, /* полное наименование (прим. юридическое лицо) */
  shortname VARCHAR(50) NOT NULL UNIQUE, /* сокращенное наименование (прим. юр. лицо) */
  CONSTRAINT type_company_ref_pk PRIMARY KEY (id)
);

/* локальное представительство профиля */
CREATE TABLE company_represent_ref (
  id SERIAL, /* id представительства */
  uuid UUID NOT NULL UNIQUE,
  uuid_company UUID NOT NULL, /* uuid компании (чьё представительства) */
  id_region INTEGER NOT NULL DEFAULT '1', /* регион представительства */
  id_representation_type INTEGER NOT NULL DEFAULT '1', /* тип представительства */
  name VARCHAR(255) NOT NULL, /* наименование представительства */
  address VARCHAR(512) NOT NULL, /* почтовый адрес представительства */
  phone VARCHAR(100) NOT NULL, /* телефон представительства */
  -- UNIQUE (uuid_company, name),
  CONSTRAINT company_represent_ref_pk PRIMARY KEY (uuid)
);

/* тип представительства компании */
CREATE TABLE representation_type_ref (
  id SERIAL, /* id типа представительства */
  representation_type VARCHAR(100) NOT NULL UNIQUE, /* наименование типа представительства */
  CONSTRAINT representation_type_ref_pk PRIMARY KEY (id)
);

/* члены компании и их роли */
CREATE TABLE company_member_role (
  id SERIAL, /* id записи */
  uuid_company UUID NOT NULL, /* uuid компании */
  uuid_user UUID NOT NULL, /* uuid профиля */
  id_role INTEGER NOT NULL, /* идентификатор роли пользователя */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /* член компании активен */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата обновления */
  UNIQUE(uuid_company, uuid_user, id_role), /* совокупоность id не может повторяться */
  CONSTRAINT company_member_role_pk PRIMARY KEY (id)
);

/* роль члена */
CREATE TABLE role_member_ref (
  id SERIAL, /* id записи */
  name VARCHAR(50) NOT NULL, /* наименование роли */
  CONSTRAINT role_member_ref_pk PRIMARY KEY (id)
);

/* уровень доступа роли */
CREATE TABLE role_access (
  id SERIAL, /* id записи */
  id_role INTEGER NOT NULL, /* идентификатор роли пользователя */
  id_type_access INTEGER NOT NULL, /* тип доступа */
  UNIQUE (id_role, id_type_access),
  CONSTRAINT role_access_pk PRIMARY KEY (id)
);

/* перечень CAD (и других программ) (пред.название type_cad_ref) */
CREATE TABLE programm_ref (
  id SERIAL, /* id наименования софта */
  name VARCHAR(225) NOT NULL UNIQUE, /* наименование CAD (название программы) */
  CONSTRAINT programm_ref_pk PRIMARY KEY (id)
);

/* категории (каталога) */
CREATE TABLE spec_ref (
  id SERIAL, /* id категории каталога */
  spec VARCHAR(100) NOT NULL, /*наименование категории */
  id_spec_parent INTEGER NOT NULL DEFAULT '1', /* id родительского каталога */
  CONSTRAINT spec_ref_pk PRIMARY KEY (id)
);

/* связь каталогов с компанией */
CREATE TABLE spec_to_company (
  id SERIAL, /* id связи */
  id_spec INTEGER NOT NULL, /* связанный с компанией каталог (категория) */
  uuid_company UUID NOT NULL, /* связанная с каталогом (категорией) компания */
  UNIQUE (id_spec, uuid_company),
  CONSTRAINT spec_to_company_pk PRIMARY KEY (id)
);

/* запись изменений данных пользователя */
CREATE TABLE user_history_list (
  id SERIAL, /* id события */
  uuid_user UUID NOT NULL, /* идентификатор профиля к которому относится изменение */
  id_type_of_change INTEGER NOT NULL, /* id изменения (тип изменения) */
  old_data VARCHAR(2000) NOT NULL, /*  обновляемые данные данные */
  changed_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата изменения */
  CONSTRAINT user_history_list_pk PRIMARY KEY (id)
);

/* запись изменений данных компании */
CREATE TABLE company_history_list (
  id SERIAL, /* id события */
  uuid_company UUID NOT NULL, /* идентификатор компании к которой относится изменение */
  id_type_of_change INTEGER NOT NULL, /* id изменения (тип изменения) */
  old_data VARCHAR(2000) NOT NULL, /*  обновляемые данные данные */
  changed_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата изменения */
  CONSTRAINT company_history_list_pk PRIMARY KEY (id)
);

/* запись изменений данных компонента */
CREATE TABLE component_history_list (
  id SERIAL, /* id события */
  uuid_component UUID NOT NULL, /* идентификатор стандарта к которому относится изменение */
  id_type_of_change INTEGER NOT NULL, /* id изменения (тип изменения) */
  old_data VARCHAR(2000) NOT NULL, /*  обновляемые данные данные */
  changed_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата изменения */
  CONSTRAINT component_history_list_pk PRIMARY KEY (id)
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

/* перечень типов изменений */
CREATE TABLE type_of_change_ref (
  id SERIAL, /* id типа изменения */
  type_of_change VARCHAR(100) NOT NULL, /*наименование изменения */
  CONSTRAINT type_of_change_ref_pk PRIMARY KEY (id)
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

/* таблица соответствия программ и расширений файлов */
CREATE TABLE extension_ref (
  id SERIAL, /* id соответствия */
  extension VARCHAR(50) NOT NULL, /* расширение файла, одно расширение может быть у нескольких программ */
  id_programm INTEGER NOT NULL, /* соответствующая программа (CAD) */
  CONSTRAINT extension_ref_pk PRIMARY KEY (id)
);

/* стандарт */
CREATE TABLE standard_ref (
  id SERIAL, /* id стандарта */
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
  -- UNIQUE (classifier, uuid_user, is_delete), /* нельзя дублировать стандарт от одного пользователя */
  CONSTRAINT standard_ref_pk PRIMARY KEY (uuid)
);

/* статус стандарта */
CREATE TABLE standard_status_ref (
  id SERIAL, /* id статуса */
  name VARCHAR(100) NOT NULL UNIQUE, /* к примеру: «Стандарт опубликован» */
  CONSTRAINT standard_status_ref_pk PRIMARY KEY (id)
);

/* компонент */
CREATE TABLE component_ref (
  id SERIAL, /* id компонента */
  uuid UUID NOT NULL UNIQUE,
  uuid_component_parent UUID NOT NULL, /* родительский компонент */
  name VARCHAR(225) NOT NULL, /* наименование компонента */
  description VARCHAR(2000) NOT NULL, /* краткое описание компонента */
  uuid_user UUID NOT NULL, /* идентификатор профиля загрузившего компонент */
  id_type_access INTEGER NOT NULL, /* доступност к компоненту по умолчанию */
  id_component_type INTEGER NOT NULL, /* тип компонента */
  id_actual_status INTEGER NOT NULL, /* номер статуса, к примеру: «актуальный», «архивный», «снято с производства» */
  is_standard INTEGER NOT NULL DEFAULT '0', /* компонент соответствует стандарту */
  is_delete BOOLEAN NOT NULL DEFAULT 'f', /* флаг удаления компонента */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания/загрузки */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата обновления */
  -- UNIQUE (name, uuid_user),
  CONSTRAINT component_ref_pk PRIMARY KEY (uuid)
);

/* доступ к компоненту отдельного пользователя */
CREATE TABLE component_access_to_user (
  id SERIAL, /* id доступа */
  uuid_component UUID NOT NULL, /* идентификатор компонента */
  uuid_user UUID NOT NULL, /* идентификатор профиля */
  id_type_access INTEGER NOT NULL, /* тип доступа к компоненту */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /* флаг актуальности доступа */
  is_delete BOOLEAN NOT NULL DEFAULT 'f', /* флаг удаления доступа */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания доступа */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата обновления */
  CONSTRAINT component_access_to_user_pk PRIMARY KEY (id)
);

/* доступ к компоненту отдельного компании */
CREATE TABLE component_access_to_company (
  id SERIAL, /* id доступа */
  uuid_component UUID NOT NULL, /* идентификатор компонента */
  uuid_company UUID NOT NULL, /* идентификатор компании */
  id_type_access INTEGER NOT NULL, /* тип доступа к компоненту */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /* флаг актуальности доступа */
  is_delete BOOLEAN NOT NULL DEFAULT 'f', /* флаг удаления доступа */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания доступа */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата обновления */
  CONSTRAINT component_access_to_company_pk PRIMARY KEY (id)
);

/* доступ к стандарту отдельного пользователя */
CREATE TABLE standard_access_to_user (
  id SERIAL, /* id доступа */
  uuid_standard UUID NOT NULL, /* идентификатор стандарта */
  uuid_user UUID NOT NULL, /* идентификатор профиля */
  id_type_access INTEGER NOT NULL, /* тип доступа к стандарту */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /* флаг актуальности доступа */
  is_delete BOOLEAN NOT NULL DEFAULT 'f', /* флаг удаления доступа */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания доступа */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата обновления */
  CONSTRAINT standard_access_to_user_pk PRIMARY KEY (id)
);

/* доступ к стандарту отдельного компании */
CREATE TABLE standard_access_to_company (
  id SERIAL, /* id доступа */
  uuid_standard UUID NOT NULL, /* идентификатор стандарта */
  uuid_company UUID NOT NULL, /* идентификатор компании */
  id_type_access INTEGER NOT NULL, /* тип доступа к стандарту */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /* флаг актуальности доступа */
  is_delete BOOLEAN NOT NULL DEFAULT 'f', /* флаг удаления доступа */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания доступа */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата обновления */
  CONSTRAINT standard_access_to_company_pk PRIMARY KEY (id)
);

/* статус компонента */
CREATE TABLE actual_status_ref (
  id SERIAL, /* id статуса */
  name VARCHAR(100) NOT NULL UNIQUE, /* к примеру: «актуальный», «архивный», «снято с производства» */
  CONSTRAINT actual_status_ref_pk PRIMARY KEY (id)
);

/* каталог компонента */
CREATE TABLE spec_to_component (
  id SERIAL, /* id связи компонента с каталогом */
  id_spec INTEGER NOT NULL, /* идентификатор позиции в каталоге */
  uuid_component UUID NOT NULL, /* идентификатор компонента */
  CONSTRAINT spec_to_component_pk PRIMARY KEY (id)
);

/* объект/файл стандарта */
CREATE TABLE file_to_standard (
  id SERIAL, /* id файла стандарта */
  uuid_file UUID NOT NULL, /* идентификатор объекта/файла */
  uuid_standard UUID NOT NULL, /* идентификатор стандарта */
  CONSTRAINT file_to_standard_pk PRIMARY KEY (id)
);

/* объект/файл компонента */
CREATE TABLE file_to_component (
  id SERIAL, /* id файла компонента */
  uuid_file UUID NOT NULL, /* идентификатор объекта/файла */
  uuid_component UUID NOT NULL, /* идентификатор компонента */
  CONSTRAINT file_to_component_pk PRIMARY KEY (id)
);

/* тип компонента (базовый, кастомный) */
CREATE TABLE component_type_ref (
  id SERIAL, /* id типа */
  component_type VARCHAR(100) NOT NULL UNIQUE, /* наименование типа */
  CONSTRAINT component_type_ref_pk PRIMARY KEY (id)
);

/* ключевые слова компонента (тегирование) */
CREATE TABLE component_keyword_ref (
  id SERIAL, /* id тега */
  keyword VARCHAR(10) NOT NULL UNIQUE, /* ключевое слово */
  CONSTRAINT component_keyword_ref_pk PRIMARY KEY (id)
);

/* ключевые слова связанные с компонентом (тегирование) */
CREATE TABLE component_to_keyword (
  id SERIAL, /* id связи тега и компонента */
  uuid_component UUID NOT NULL, /* идентификатор компонента */
  id_component_keyword INTEGER NOT NULL, /* идентификатор ключевого слова (тега) */
  CONSTRAINT component_to_keyword_pk PRIMARY KEY (id)
);

/* типы доступа */
CREATE TABLE type_access_ref (
  id SERIAL, /* id типа доступа */
  name VARCHAR(100) NOT NULL UNIQUE, /* наименование доступа */
  CONSTRAINT type_access_ref_pk PRIMARY KEY (id)
);

/* обсуждение компонента */
CREATE TABLE discussion_component_ref (
  id SERIAL, /* id комментария */
  id_discussion_parent INTEGER NOT NULL, /* id родительского комментария */
  uuid_component UUID NOT NULL, /* идентификатор обсуждаемого компонента */
  uuid_author UUID NOT NULL, /* идентификатор профиля отправителя */
  message_content VARCHAR(4000) NOT NULL, /* сообщение/комментарий */
  is_delete BOOLEAN NOT NULL DEFAULT 'f', /* флаг удаления */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания/редактирования */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата обновления */
  CONSTRAINT discussion_component_ref_pk PRIMARY KEY (id)
);

/* обсуждение компании */
CREATE TABLE discussion_company_ref (
  id SERIAL, /* id комментария */
  id_discussion_parent INTEGER NOT NULL, /* id родительского комментария */
  uuid_company UUID NOT NULL, /* идентификатор обсуждаемой компании */
  uuid_author UUID NOT NULL, /* идентификатор профиля отправителя */
  message_content VARCHAR(4000) NOT NULL, /* сообщение/комментарий */
  is_delete BOOLEAN NOT NULL DEFAULT 'f', /* флаг удаления */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания/редактирования */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата обновления */
  CONSTRAINT discussion_company_ref_pk PRIMARY KEY (id)
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

/* параметр компонента */
CREATE TABLE param_to_component (
  id SERIAL, /* id параметра компонента */
  uuid_component UUID NOT NULL, /* идентификатор компонента */
  id_param INTEGER NOT NULL, /* идентификатор параметра */
  value VARCHAR(100) NOT NULL, /* параметр компонента */
  CONSTRAINT param_to_component_pk PRIMARY KEY (id)
);

/* отслеживание компонента пользователем */
CREATE TABLE component_fav (
  id SERIAL, /* id подписки (начала отслеживания) */
  uuid_component UUID NOT NULL, /* идентификатор компонента для отслеживания */
  uuid_user UUID NOT NULL, /* идентификатор профиля */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /*  флаг актуальности отслеживания */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания */
  CONSTRAINT component_fav_pk PRIMARY KEY (id)
);

/* отслеживание компании пользователем */
CREATE TABLE company_fav (
  id SERIAL, /* id подписки (начала отслеживания) */
  uuid_company UUID NOT NULL, /* идентификатор компании для отслеживания */
  uuid_user UUID NOT NULL, /* идентификатор профиля */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /*  флаг актуальности отслеживания */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания */
  CONSTRAINT company_fav_pk PRIMARY KEY (id)
);

/* отслеживание стандарта пользователем */
CREATE TABLE standard_fav (
  id SERIAL, /* id подписки (начала отслеживания) */
  uuid_standard UUID NOT NULL, /* идентификатор стандарта для отслеживания */
  uuid_user UUID NOT NULL, /* идентификатор профиля */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /*  флаг актуальности отслеживания */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания */
  CONSTRAINT standard_fav_pk PRIMARY KEY (id)
);

/* отслеживание профиля пользователем */
CREATE TABLE user_fav (
  id SERIAL, /* id подписки (начала отслеживания) */
  uuid_user_favorite UUID NOT NULL, /* идентификатор профиля для отслеживания */
  uuid_user_follower UUID NOT NULL, /* идентификатор отслеживающего профиля */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /*  флаг актуальности отслеживания */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания */
  CONSTRAINT user_fav_pk PRIMARY KEY (id)
);

/* регион */
CREATE TABLE region_ref (
  id SERIAL, /* id наименования региона */
  region VARCHAR(100) NOT NULL, /* наименование региона */
  CONSTRAINT region_ref_pk PRIMARY KEY (id)
);

/* список поставщиков компонента (list shippers) */
CREATE TABLE supplier_to_component (
  id SERIAL, /* id записи профиля в поставщики компонента */
  uuid_component UUID NOT NULL, /* идентификатор компонента */
  uuid_company UUID NOT NULL, /* идентификатор компании-поставщика */
  description VARCHAR(255) NOT NULL, /* комментарий к поставщику */
  CONSTRAINT supplier_to_component_pk PRIMARY KEY (id)
);

/* Список модификаций компонента */
CREATE TABLE component_modification_list (
  id SERIAL, /* id компонента */
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

/* объект/файл модификации */
CREATE TABLE file_to_modification (
  id SERIAL, /* id файла модификации */
  uuid_file UUID NOT NULL, /* идентификатор объекта/файла */
  uuid_modification UUID NOT NULL, /* идентификатор модификации */
  CONSTRAINT file_to_modification_pk PRIMARY KEY (id)
);

/* набор файлоы модификации (файлы под САПР) */
CREATE TABLE set_file_to_programm (
  id SERIAL, /* id файла модификации */
  uuid_modification UUID NOT NULL, /* идентификатор модификации */
  uuid_file UUID NOT NULL, /* идентификатор объекта/файла */
  id_programm INTEGER NOT NULL, /* САПР (для быстрой загрузки данных) */
  UNIQUE (uuid_modification, id_programm), /* один набор файлов модификации для одного САПРа */
  CONSTRAINT set_file_to_modification_pk PRIMARY KEY (id)
);

/* параметр модификации */
CREATE TABLE param_to_modification (
  id SERIAL, /* id параметра модификации */
  uuid_modification UUID NOT NULL, /* идентификатор модификации */
  id_param INTEGER NOT NULL, /* идентификатор параметра */
  value VARCHAR(255) NOT NULL, /* параметр компонента */
  CONSTRAINT param_to_modification_pk PRIMARY KEY (id)
);
