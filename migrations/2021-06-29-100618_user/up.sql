-- Your SQL goes here
/* профиль */
CREATE TABLE user_ref (
  uuid UUID NOT NULL PRIMARY KEY,
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
  time_zone VARCHAR(50) NOT NULL, /* временная зона профиля */
  uuid_image_file UUID NOT NULL, /* картинка пользователя */
  id_region INTEGER NOT NULL, /* регион */
  id_program INTEGER NOT NULL DEFAULT '1', /* САПР «по умолчанию» (для быстрой загрузки данных) */
  id_type_access INTEGER NOT NULL DEFAULT '1', /* тип доступности */
  is_email_verified BOOLEAN NOT NULL DEFAULT 'f', /* подтверждение email */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /* флаг активности пользователь */
  is_delete BOOLEAN NOT NULL DEFAULT 'f', /* флаг удаления пользователя */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания профиля */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW() /* дата обновления профиля */
);

/* токен сессии клиента */
CREATE TABLE user_token_ref (
  uuid_user UUID NOT NULL, /* идентификатор пользователя */
  token VARCHAR(4096) NOT NULL, /* токен пользователя */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания токена */
  expiration_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата окончания действия токена */
  CHECK(created_at<expiration_at),
  CONSTRAINT user_token_ref_pk PRIMARY KEY (uuid_user, token)
);

/* ключи пользователя для доступа к хранилищу */
CREATE TABLE user_storage_access_ref (
  uuid_user UUID NOT NULL, /* идентификатор пользователя */
  application_key_id VARCHAR(512) NOT NULL, /* id ключа пользователя */
  application_key VARCHAR(512) NOT NULL, /* ключ пользователя */
  key_expiration_at TIMESTAMP NOT NULL, /* дата окончания действия ключа */
  bucket_id VARCHAR(512) NOT NULL, /* id корзины в хранилище */
  api_url VARCHAR(512) NOT NULL, /* url доступа к api */
  authorization_token VARCHAR(512) NOT NULL, /* токена пользователя */
  token_expiration_at TIMESTAMP NOT NULL, /* дата окончания действия токена */
  CONSTRAINT user_storage_access_ref_pk PRIMARY KEY (
    uuid_user,
    application_key_id,
    application_key
  )
);

/* запись изменений данных пользователя */
CREATE TABLE user_history_list (
  id SERIAL UNIQUE, /* id события */
  uuid_user UUID NOT NULL, /* идентификатор профиля к которому относится изменение */
  id_type_of_change INTEGER NOT NULL, /* id изменения (тип изменения) */
  old_data VARCHAR(2000) NOT NULL, /*  обновляемые данные данные */
  changed_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата изменения */
  CONSTRAINT user_history_list_pk PRIMARY KEY (id)
);

/* доступ к компоненту отдельного пользователя */
CREATE TABLE user_access_to_component (
  -- id SERIAL, /* id доступа */
  uuid_component UUID NOT NULL, /* идентификатор компонента */
  uuid_user UUID NOT NULL, /* идентификатор профиля */
  id_type_access INTEGER NOT NULL, /* тип доступа к компоненту */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /* флаг актуальности доступа */
  is_delete BOOLEAN NOT NULL DEFAULT 'f', /* флаг удаления доступа */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания доступа */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата обновления */
  CONSTRAINT user_access_to_component_pk PRIMARY KEY (uuid_component, uuid_user)
);

/* доступ к стандарту отдельного пользователя */
CREATE TABLE user_access_to_standard (
  -- id SERIAL, /* id доступа */
  uuid_standard UUID NOT NULL, /* идентификатор стандарта */
  uuid_user UUID NOT NULL, /* идентификатор профиля */
  id_type_access INTEGER NOT NULL, /* тип доступа к стандарту */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /* флаг актуальности доступа */
  is_delete BOOLEAN NOT NULL DEFAULT 'f', /* флаг удаления доступа */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания доступа */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата обновления */
  CONSTRAINT user_access_to_standard_pk PRIMARY KEY (uuid_standard, uuid_user)
);

/* отслеживание профиля пользователем */
CREATE TABLE user_fav (
  -- id SERIAL, /* id подписки (начала отслеживания) */
  uuid_user_favorite UUID NOT NULL, /* идентификатор профиля для отслеживания */
  uuid_user_follower UUID NOT NULL, /* идентификатор отслеживающего профиля */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /*  флаг актуальности отслеживания */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания */
  CONSTRAINT user_fav_pk PRIMARY KEY (uuid_user_favorite, uuid_user_follower)
);

/* отслеживание компании пользователем */
CREATE TABLE company_fav (
  -- id SERIAL, /* id подписки (начала отслеживания) */
  uuid_company UUID NOT NULL, /* идентификатор компании для отслеживания */
  uuid_user UUID NOT NULL, /* идентификатор профиля */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /*  флаг актуальности отслеживания */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания */
  CONSTRAINT company_fav_pk PRIMARY KEY (uuid_company, uuid_user)
);

/* отслеживание компонента пользователем */
CREATE TABLE component_fav (
  -- id SERIAL, /* id подписки (начала отслеживания) */
  uuid_component UUID NOT NULL, /* идентификатор компонента для отслеживания */
  uuid_user UUID NOT NULL, /* идентификатор профиля */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /*  флаг актуальности отслеживания */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания */
  CONSTRAINT component_fav_pk PRIMARY KEY (uuid_component, uuid_user)
);

/* отслеживание стандарта пользователем */
CREATE TABLE standard_fav (
  -- id SERIAL, /* id подписки (начала отслеживания) */
  uuid_standard UUID NOT NULL, /* идентификатор стандарта для отслеживания */
  uuid_user UUID NOT NULL, /* идентификатор профиля */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /*  флаг актуальности отслеживания */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания */
  CONSTRAINT standard_fav_pk PRIMARY KEY (uuid_standard, uuid_user)
);

/* доступ к стандарту отдельного пользователя */
CREATE TABLE notification_to_user (
  id SERIAL UNIQUE, /* id уведомления */
  id_notification INTEGER NOT NULL, /* идентификатор уведомления */
  uuid_user UUID NOT NULL, /* идентификатор профиля */
  CONSTRAINT notification_to_user_pk PRIMARY KEY (id_notification, uuid_user)
);

/* сертификаты пользователя */
CREATE TABLE user_certificate_ref (
  uuid_file UUID NOT NULL, /* файл сертификата */
  uuid_user UUID NOT NULL, /* пользователь которому выдан сертификат */
  description VARCHAR(100) NOT NULL, /* информация, дополнение */
  CONSTRAINT user_certificate_ref_pk PRIMARY KEY (uuid_file, uuid_user)
);
