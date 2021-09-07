-- Your SQL goes here/* компания */
CREATE TABLE company_ref (
  uuid UUID NOT NULL UNIQUE PRIMARY KEY,
  orgname VARCHAR(255) NOT NULL, /* наименование организации (для юр.лиц) */
  shortname VARCHAR(255) NOT NULL, /* сокращённое наименование организации (для юр.лиц) */
  inn VARCHAR(30) NOT NULL, /*инн компании */
  phone VARCHAR(100) NOT NULL, /*номер телефона */
  email VARCHAR(100) NOT NULL, /* email компании */
  description VARCHAR(4000) NOT NULL, /* информация, описание */
  address VARCHAR(512) NOT NULL, /*почтовый адрес */
  site_url VARCHAR(255) NOT NULL, /* URL адрес сайта компании */
  time_zone VARCHAR(50) NOT NULL, /* временная зона компании */
  uuid_user UUID NOT NULL, /* uuuid профиля - владельца */
  uuid_image_file UUID NOT NULL, /* логотип компании */
  id_region INTEGER NOT NULL, /* регион */
  id_company_type INTEGER NOT NULL DEFAULT '1', /* тип компании (ао, пао, ооо, ип) */
  is_supplier BOOLEAN NOT NULL DEFAULT 'f',  /* роль пользователя: поставщик/заказчик */
  is_email_verified BOOLEAN NOT NULL DEFAULT 'f', /* подтверждение email */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /* флаг активности пользователь */
  is_delete BOOLEAN NOT NULL DEFAULT 'f', /* флаг удаления пользователя */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания компании */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата обновления компании */
  UNIQUE (orgname, inn, uuid_user) /* пользователь не может создавать компании с одним названием и инн */
);

/* тип компании */
CREATE TABLE company_type_ref (
  id SERIAL, /* id типа компании */
  CONSTRAINT company_type_ref_pk PRIMARY KEY (id)
);

CREATE TABLE company_type_translate_list (
  id_company_type INTEGER NOT NULL, /* id типа компании */
  id_lang INTEGER NOT NULL, /* идентификатор языка перевода */
  name VARCHAR(255) NOT NULL, /* полное наименование (прим. юридическое лицо) в переводе */
  shortname VARCHAR(50) NOT NULL, /* сокращенное наименование (прим. юр. лицо) в переводе */
  UNIQUE(id_lang, name, shortname),
  CONSTRAINT company_type_translate_list_pk PRIMARY KEY (id_company_type, id_lang)
);

/* локальное представительство профиля */
CREATE TABLE company_represent_ref (
  uuid UUID NOT NULL UNIQUE,
  uuid_company UUID NOT NULL, /* uuid компании (чьё представительства) */
  id_region INTEGER NOT NULL DEFAULT '1', /* регион представительства */
  id_representation_type INTEGER NOT NULL DEFAULT '1', /* тип представительства */
  name VARCHAR(255) NOT NULL, /* наименование представительства */
  address VARCHAR(512) NOT NULL, /* почтовый адрес представительства */
  phone VARCHAR(100) NOT NULL, /* телефон представительства */
  CONSTRAINT company_represent_ref_pk PRIMARY KEY (uuid)
);

/* члены компании и их роли */
CREATE TABLE company_member_role (
  uuid_company UUID NOT NULL, /* uuid компании */
  uuid_user UUID NOT NULL, /* uuid профиля */
  id_role INTEGER NOT NULL, /* идентификатор роли пользователя */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /* член компании активен */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата обновления */
  CONSTRAINT company_member_role_pk PRIMARY KEY (uuid_company, uuid_user, id_role)
);

/* связь каталогов с компанией */
CREATE TABLE spec_to_company (
  id_spec INTEGER NOT NULL, /* связанный с компанией каталог (категория) */
  uuid_company UUID NOT NULL, /* связанная с каталогом (категорией) компания */
  CONSTRAINT spec_to_company_pk PRIMARY KEY (id_spec, uuid_company)
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

/* тип представительства компании */
CREATE TABLE representation_type_ref (
  id SERIAL, /* id типа представительства */
  CONSTRAINT representation_type_ref_pk PRIMARY KEY (id)
);

CREATE TABLE representation_type_translate_list (
  id_representation_type INTEGER NOT NULL, /* id типа представительства */
  id_lang INTEGER NOT NULL, /* идентификатор языка перевода */
  representation_type VARCHAR(100) NOT NULL UNIQUE, /* наименование типа представительства в переводе */
  UNIQUE(id_lang, representation_type),
  CONSTRAINT representation_type_translate_list_pk PRIMARY KEY (id_representation_type, id_lang)
);

/* роль члена */
CREATE TABLE role_member_ref (
  id SERIAL, /* id записи */
  CONSTRAINT role_member_ref_pk PRIMARY KEY (id)
);

CREATE TABLE role_member_translate_list (
  id_role_member INTEGER NOT NULL, /* id роли */
  id_lang INTEGER NOT NULL, /* идентификатор языка перевода */
  name VARCHAR(50) NOT NULL, /* наименование роли в переводе */
  UNIQUE(id_lang, name),
  CONSTRAINT role_member_translate_list_pk PRIMARY KEY (id_role_member, id_lang)
);

/* уровень доступа роли */
CREATE TABLE role_access (
  id_role INTEGER NOT NULL, /* идентификатор роли пользователя */
  id_type_access INTEGER NOT NULL, /* тип доступа */
  CONSTRAINT role_access_pk PRIMARY KEY (id_role, id_type_access)
);

/* доступ к компоненту отдельного компании */
CREATE TABLE company_access_to_component (
  uuid_component UUID NOT NULL, /* идентификатор компонента */
  uuid_company UUID NOT NULL, /* идентификатор компании */
  id_type_access INTEGER NOT NULL, /* тип доступа к компоненту */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /* флаг актуальности доступа */
  is_delete BOOLEAN NOT NULL DEFAULT 'f', /* флаг удаления доступа */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания доступа */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата обновления */
  CONSTRAINT company_access_to_component_pk PRIMARY KEY (uuid_component, uuid_company)
);

/* доступ к стандарту отдельной компании */
CREATE TABLE company_access_to_standard (
  uuid_standard UUID NOT NULL, /* идентификатор стандарта */
  uuid_company UUID NOT NULL, /* идентификатор компании */
  id_type_access INTEGER NOT NULL, /* тип доступа к стандарту */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /* флаг актуальности доступа */
  is_delete BOOLEAN NOT NULL DEFAULT 'f', /* флаг удаления доступа */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания доступа */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата обновления */
  CONSTRAINT company_access_to_standard_pk PRIMARY KEY (uuid_standard, uuid_company)
);

/* сертификаты компани */
CREATE TABLE company_certificate_ref (
  uuid_file UUID NOT NULL, /* файл сертификата */
  uuid_company UUID NOT NULL, /* компания которой выдан сертификат */
  description VARCHAR(100) NOT NULL, /* информация, дополнение */
  CONSTRAINT company_certificate_ref_pk PRIMARY KEY (uuid_file, uuid_company)
);
