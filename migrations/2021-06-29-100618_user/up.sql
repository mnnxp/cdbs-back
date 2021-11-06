-- Your SQL goes here
/* профиль */
CREATE TABLE user_ref (
  uuid UUID NOT NULL PRIMARY KEY,
  email VARCHAR(100) NOT NULL, /*email профиля, на один адрес может быть несколько профилей (закос под reddit) */
  psw_hash BYTEA NOT NULL, /* хеш пароля профиля */
  psw_salt BYTEA NOT NULL, /* соль для пароля профиля */
  firstname VARCHAR(100) NOT NULL, /*Имя */
  lastname VARCHAR(100) NOT NULL, /*Фамилия */
  secondname VARCHAR(100) NOT NULL, /*Отчество */
  username VARCHAR(100) NOT NULL UNIQUE, /* ник профиля (может использоваться для авторизации) */
  phone VARCHAR(100) NOT NULL, /*номер телефона */
  description VARCHAR(2000) NOT NULL, /* информация для связи, подпись */
  address VARCHAR(512) NOT NULL, /*почтовый адрес */
  position VARCHAR(255) NOT NULL, /*роль/должность */
  time_zone VARCHAR(50) NOT NULL, /* временная зона профиля */
  image_file_uuid UUID NOT NULL, /* картинка пользователя */
  region_id INTEGER NOT NULL, /* регион */
  program_id INTEGER NOT NULL DEFAULT '1', /* САПР «по умолчанию» (для быстрой загрузки данных) */
  type_access_id INTEGER NOT NULL DEFAULT '1', /* тип доступности */
  is_email_verified BOOLEAN NOT NULL DEFAULT 'f', /* подтверждение email */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /* флаг активности пользователь */
  is_delete BOOLEAN NOT NULL DEFAULT 'f', /* флаг удаления пользователя */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания профиля */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW() /* дата обновления профиля */
);

/* токен сессии клиента */
CREATE TABLE user_token_ref (
  user_uuid UUID NOT NULL, /* идентификатор пользователя */
  token VARCHAR(4096) NOT NULL, /* токен пользователя */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания токена */
  expiration_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата окончания действия токена */
  CHECK(created_at<expiration_at),
  CONSTRAINT user_token_ref_pk PRIMARY KEY (user_uuid, token)
);

/* запись изменений данных пользователя */
CREATE TABLE user_history_list (
  id SERIAL UNIQUE, /* id события */
  user_uuid UUID NOT NULL, /* идентификатор профиля к которому относится изменение */
  type_of_change_id INTEGER NOT NULL, /* id изменения (тип изменения) */
  old_data VARCHAR(2000) NOT NULL, /*  обновляемые данные данные */
  changed_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата изменения */
  CONSTRAINT user_history_list_pk PRIMARY KEY (id)
);

/* доступ к компоненту отдельного пользователя */
CREATE TABLE user_access_to_component (
  -- id SERIAL, /* id доступа */
  component_uuid UUID NOT NULL, /* идентификатор компонента */
  user_uuid UUID NOT NULL, /* идентификатор профиля */
  type_access_id INTEGER NOT NULL, /* тип доступа к компоненту */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /* флаг актуальности доступа */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания доступа */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата обновления */
  CONSTRAINT user_access_to_component_pk PRIMARY KEY (component_uuid, user_uuid)
);

/* доступ к стандарту отдельного пользователя */
CREATE TABLE user_access_to_standard (
  -- id SERIAL, /* id доступа */
  standard_uuid UUID NOT NULL, /* идентификатор стандарта */
  user_uuid UUID NOT NULL, /* идентификатор профиля */
  type_access_id INTEGER NOT NULL, /* тип доступа к стандарту */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /* флаг актуальности доступа */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания доступа */
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата обновления */
  CONSTRAINT user_access_to_standard_pk PRIMARY KEY (standard_uuid, user_uuid)
);

/* отслеживание профиля пользователем */
CREATE TABLE user_fav (
  -- id SERIAL, /* id подписки (начала отслеживания) */
  user_favorite_uuid UUID NOT NULL, /* идентификатор профиля для отслеживания */
  user_follower_uuid UUID NOT NULL, /* идентификатор отслеживающего профиля */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /*  флаг актуальности отслеживания */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания */
  CONSTRAINT user_fav_pk PRIMARY KEY (user_favorite_uuid, user_follower_uuid)
);

/* отслеживание компании пользователем */
CREATE TABLE company_fav (
  -- id SERIAL, /* id подписки (начала отслеживания) */
  company_uuid UUID NOT NULL, /* идентификатор компании для отслеживания */
  user_uuid UUID NOT NULL, /* идентификатор профиля */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /*  флаг актуальности отслеживания */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания */
  CONSTRAINT company_fav_pk PRIMARY KEY (company_uuid, user_uuid)
);

/* отслеживание компонента пользователем */
CREATE TABLE component_fav (
  -- id SERIAL, /* id подписки (начала отслеживания) */
  component_uuid UUID NOT NULL, /* идентификатор компонента для отслеживания */
  user_uuid UUID NOT NULL, /* идентификатор профиля */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /*  флаг актуальности отслеживания */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания */
  CONSTRAINT component_fav_pk PRIMARY KEY (component_uuid, user_uuid)
);

/* отслеживание стандарта пользователем */
CREATE TABLE standard_fav (
  -- id SERIAL, /* id подписки (начала отслеживания) */
  standard_uuid UUID NOT NULL, /* идентификатор стандарта для отслеживания */
  user_uuid UUID NOT NULL, /* идентификатор профиля */
  is_enabled BOOLEAN NOT NULL DEFAULT 't', /*  флаг актуальности отслеживания */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания */
  CONSTRAINT standard_fav_pk PRIMARY KEY (standard_uuid, user_uuid)
);

/* доступ к стандарту отдельного пользователя */
CREATE TABLE notification_to_user (
  notification_id INTEGER NOT NULL, /* идентификатор уведомления */
  user_uuid UUID NOT NULL, /* идентификатор профиля */
  is_read BOOLEAN NOT NULL DEFAULT 'f', /* статус прочтения */
  CONSTRAINT notification_to_user_pk PRIMARY KEY (notification_id, user_uuid)
);

/* сертификаты пользователя */
CREATE TABLE user_certificate_ref (
  file_uuid UUID NOT NULL, /* файл сертификата */
  user_uuid UUID NOT NULL, /* пользователь которому выдан сертификат */
  description VARCHAR(100) NOT NULL, /* информация, дополнение */
  CONSTRAINT user_certificate_ref_pk PRIMARY KEY (file_uuid, user_uuid)
);
