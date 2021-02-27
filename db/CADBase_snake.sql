/* + */
/* профиль */
CREATE TABLE user_ref (
  id SERIAL PRIMARY KEY, /* id профиля */
  uuid UUID NOT NULL,
  email VARCHAR(100) NOT NULL, /*email профиля, на один адрес может быть несколько профилей (закос под reddit) */
  email_verified INTEGER NOT NULL DEFAULT '0', /* подтверждение email */
  psw_hash BYTEA NOT NULL, /* хеш пароля профиля */
  psw_salt VARCHAR(255) NOT NULL, /* соль для пароля профиля */
  id_type_org INTEGER NOT NULL DEFAULT '0', /* тип профиля (физ. лицо, юр. лицо, ип) */
  firstname VARCHAR(100) NOT NULL, /*Имя */
  lastname VARCHAR(100) NOT NULL, /*Фамилия */
  secondname VARCHAR(100) NOT NULL, /*Отчество */
  nickname VARCHAR(100) NOT NULL UNIQUE, /* ник профиля (может использоваться для авторизации) */
  orgname VARCHAR(255) NOT NULL, /* наименование организации (для юр.лиц) */
  shortname VARCHAR(255) NOT NULL, /* сокращённое наименование организации (для юр.лиц) */
  inn VARCHAR(30) NOT NULL UNIQUE, /*инн профиля */
  phone VARCHAR(100) NOT NULL, /*номер телефона */
  id_name_cad INTEGER NOT NULL DEFAULT '0', /* САПР «по умолчанию» (для быстрой загрузки данных) */
  comment VARCHAR(2000) NOT NULL, /* информация для связи, подпись */
  address VARCHAR(512) NOT NULL, /*почтовый адрес */
  time_zone VARCHAR(255) NOT NULL, /*часовой пояс профиля */
  position VARCHAR(255) NOT NULL, /*роль/должность */
  site_url VARCHAR(255) NOT NULL, /* URL адрес сайта профиля */
  id_file_info_icon INTEGER NOT NULL DEFAULT '0', /* картинка пользователя */
  id_region INTEGER NOT NULL DEFAULT '0', /* регион */
  created_at TIMESTAMP NOT NULL DEFAULT NOW() /* дата создания профиля */
);

/* токен сессии клиента */
CREATE TABLE user_tokens_ref (
  id SERIAL, /* id токена */
  id_user INTEGER NOT NULL, /* идентификатор пользователя */
  token VARCHAR(512) NOT NULL, /* токен пользователя */
  date_start TIMESTAMP NOT NULL DEFAULT NOW() NOT NULL, /* дата создания токена */
  date_end TIMESTAMP NOT NULL DEFAULT NOW() NOT NULL, /* дата окончания действия токена */
  CONSTRAINT user_tokens_ref_pk PRIMARY KEY (id)
);

/* локальное представительство профиля */
CREATE TABLE user_represet_ref (
  id SERIAL, /* id представительства */
  id_user INTEGER NOT NULL, /* id профиля (чьё представительства) */
  id_region INTEGER NOT NULL DEFAULT '0', /* регион представительства */
  id_representation_type INTEGER NOT NULL DEFAULT '0', /* тип представительства */
  name VARCHAR(255) NOT NULL, /* наименование представительства */
  address VARCHAR(512) NOT NULL, /* почтовый адрес представительства */
  phone VARCHAR(100) NOT NULL, /* телефон представительства */
  CONSTRAINT user_represet_ref_pk PRIMARY KEY (id)
);

/* тип представительства профиля */
CREATE TABLE representation_type_ref (
  id SERIAL, /* id типа представительства */
  representation_type VARCHAR(100) NOT NULL UNIQUE, /* наименование типа представительства */
  CONSTRAINT representation_type_ref_pk PRIMARY KEY (id)
);

/* + */
/* тип профиля */
CREATE TABLE type_org_ref (
  id SERIAL, /* id типа профиля*/
  typeorg VARCHAR(100) NOT NULL UNIQUE, /* полное наименование (прим. юридическое лицо) */
  typeorgshort VARCHAR(10) NOT NULL UNIQUE, /* сокращенное наименование (прим. юр. лицо) */
  CONSTRAINT type_org_ref_pk PRIMARY KEY (id)
);

/* + */
/* категории (каталога) */
CREATE TABLE spec_ref (
  id SERIAL, /* id категории каталога */
  spec VARCHAR(100) NOT NULL, /*наименование категории */
  id_spec_parent INTEGER NOT NULL DEFAULT '0', /* id родительского каталога */
  CONSTRAINT spec_ref_pk PRIMARY KEY (id)
);

/* + */
/* перечень CAD (и других программ) (пред.название type_cad_ref) */
CREATE TABLE name_cad_ref (
  id SERIAL, /* id наименования софта */
  name_cad VARCHAR(225) NOT NULL UNIQUE, /* наименование CAD (название программы) */
  CONSTRAINT name_cad_ref_pk PRIMARY KEY (id)
);

/* + */
/* связь каталогов с профилем */
CREATE TABLE spec_to_user (
  id SERIAL, /* id связи */
  id_spec INTEGER NOT NULL, /* связанный с профилем каталог (категория) */
  id_user INTEGER NOT NULL, /* связанный с каталогом (категорией) профиль */
  CONSTRAINT spec_to_user_pk PRIMARY KEY (id)
);

/* + */
/* запись изменений данных профиля */
CREATE TABLE user_history_list (
  id SERIAL, /* id события */
  id_user INTEGER NOT NULL, /* идентификатор профиля к которому относится изменение */
  datechange TIMESTAMP NOT NULL DEFAULT NOW(), /* дата изменения */
  id_type_of_change INTEGER NOT NULL, /* id изменения (тип изменения) */
  commentchange VARCHAR(2000) NOT NULL, /*  комментарий с вносимыми изменениями */
  CONSTRAINT user_history_list_pk PRIMARY KEY (id)
);

/* + */
/* перечень типов изменений */
CREATE TABLE type_of_change_ref (
  id SERIAL, /* id типа изменения */
  type_of_change VARCHAR(100) NOT NULL, /*наименование изменения */
  CONSTRAINT type_of_change_ref_pk PRIMARY KEY (id)
);

/* + */
/* информация о файле (изображении) */
CREATE TABLE file_ref (
  id SERIAL, /* id файла */
  id_file INTEGER NOT NULL, /* идентификатор объекта/файла */
  hash BYTEA NOT NULL, /* хеш значение объекта/файла */
  id_user_create INTEGER NOT NULL, /* идентификатор профиля загрузившего файл */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания/загрузки */
  filename VARCHAR(225) NOT NULL, /* наименование файла */
  id_ext INTEGER NOT NULL, /* расширение файла (используется для определения CAD) */
  filesize FLOAT NOT NULL, /* размер файла */
  path_file VARCHAR(225) NOT NULL, /* путь к файлу */
  CONSTRAINT file_ref_pk PRIMARY KEY (id)
);

/* + */
/* таблица соответствия CAD и расширений файлов */
CREATE TABLE extension_ref (
  id SERIAL, /* id соответствия */
  extension VARCHAR(10) NOT NULL, /* расширение файла, одно расширение может быть у нескольких программ */
  id_name_cad INTEGER NOT NULL UNIQUE, /* соответствующая программа (CAD) */
  CONSTRAINT extension_ref_pk PRIMARY KEY (id)
);

/* + */
/* компонент */
CREATE TABLE component_ref (
  id SERIAL, /* id компонента */
  name VARCHAR(225) NOT NULL UNIQUE, /* наименование компонента */
  id_user INTEGER NOT NULL, /* идентификатор профиля загрузившего компонент */
  comment VARCHAR(2000) NOT NULL, /* краткое описание компонента */
  id_component_parent INTEGER NOT NULL DEFAULT '0', /* родительский компонент */
  id_actual_status INTEGER NOT NULL, /* номер статуса, к примеру: «актуальный», «архивный», «снято с производства» */
  id_component_type INTEGER NOT NULL, /* тип компонента (базовый/кастомный) */
  is_delete INTEGER NOT NULL DEFAULT '0', /* флаг удаления компонента */
  id_type_access INTEGER NOT NULL, /* тип доступности компонента */
  commentchange VARCHAR(2000) NOT NULL, /*  комментарий с вносимыми изменениями */
  is_standard INTEGER NOT NULL DEFAULT '0', /* компонент соответствует стандарту */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания/загрузки */
  CONSTRAINT component_ref_pk PRIMARY KEY (id)
);

/* доступ к компоненту отдельного пользователя */
CREATE TABLE component_access_to_user (
  id SERIAL, /* id доступа */
  id_component INTEGER NOT NULL, /* идентификатор компонента */
  id_user INTEGER NOT NULL, /* идентификатор профиля с доступом */
  id_type_access INTEGER NOT NULL, /* тип доступа профиля к компоненту */
  is_actual INTEGER NOT NULL DEFAULT '0', /* флаг актуальности доступа */
  is_delete INTEGER NOT NULL DEFAULT '0', /* флаг удаления доступа */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания доступа */
  CONSTRAINT component_access_to_user_pk PRIMARY KEY (id)
);

/* статус компонента */
CREATE TABLE actual_status_ref (
  id SERIAL, /* id статуса */
  actualstatus VARCHAR(100) NOT NULL UNIQUE, /* к примеру: «актуальный», «архивный», «снято с производства» */
  CONSTRAINT actual_status_ref_pk PRIMARY KEY (id)
);

/* + */
/* каталог компонента */
CREATE TABLE spec_to_component (
  id SERIAL, /* id связи компонента с каталогом */
  id_spec INTEGER NOT NULL, /* идентификатор позиции в каталоге */
  id_component INTEGER NOT NULL, /* идентификатор компонента */
  CONSTRAINT spec_to_component_pk PRIMARY KEY (id)
);

/* + */
/* объект/файл компонента */
CREATE TABLE file_to_component (
  id SERIAL, /* id файла компонента */
  id_file INTEGER NOT NULL, /* идентификатор объекта/файла */
  id_component INTEGER NOT NULL, /* идентификатор компонента */
  CONSTRAINT file_to_component_pk PRIMARY KEY (id)
);

/* + */
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

/* + */
/* ключевые слова связанные с компонентом (тегирование) */
CREATE TABLE component_to_keyword (
  id SERIAL, /* id связи тега и компонента */
  id_component INTEGER NOT NULL, /* идентификатор компонента */
  id_component_keyword INTEGER NOT NULL, /* идентификатор ключевого слова (тега) */
  CONSTRAINT component_to_keyword_pk PRIMARY KEY (id)
);

/* + */
/* типы доступа */
CREATE TABLE type_access_ref (
  id SERIAL, /* id типа доступа */
  type_access VARCHAR(100) NOT NULL UNIQUE, /* наименование доступа */
  CONSTRAINT type_access_ref_pk PRIMARY KEY (id)
);

/* обсуждение компонента */
CREATE TABLE discussion_ref (
  id SERIAL, /* id комментария */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания/редактирования */
  id_component INTEGER NOT NULL, /* идентификатор компонента */
  id_user_from INTEGER NOT NULL, /* идентификатор профиля отправителя */
  id_user_to INTEGER NOT NULL, /* идентификатор профиля адресата */
  comment VARCHAR(2000) NOT NULL, /* сообщение/комментарий */
  id_discussion_parent INTEGER NOT NULL, /* id родительского комментария */
  CONSTRAINT discussion_ref_pk PRIMARY KEY (id)
);

/* + */
/* параметры для модификации */
CREATE TABLE param_ref (
  id SERIAL, /* id параметра (характеристики) */
  paramname VARCHAR(100) NOT NULL UNIQUE, /* наименование парметра модификации*/
  CONSTRAINT param_ref_pk PRIMARY KEY (id)
);

/* + */
/* перевод параметра (язык) */
CREATE TABLE language_ref (
  id SERIAL, /* id языка */
  lang VARCHAR(100) NOT NULL UNIQUE, /* полное наименование языка */
  langshort VARCHAR(10) NOT NULL UNIQUE, /* краткое наименование языка */
  CONSTRAINT language_ref_pk PRIMARY KEY (id)
);

/* + */
/* перевод параметра (перевод) */
CREATE TABLE param_translate_list (
  id SERIAL, /* id перевода */
  id_param INTEGER NOT NULL, /* идентификатор параметра */
  id_lang INTEGER NOT NULL, /* идентификатор языка перевода */
  param VARCHAR(100) NOT NULL, /* перевод параметра */
  CONSTRAINT param_translate_list_pk PRIMARY KEY (id)
);

/* + */
/* перевод раздела каталога */
CREATE TABLE spec_translate_list (
  id SERIAL, /* id перевода */
  id_spec INTEGER NOT NULL, /* идентификатор раздела */
  id_lang INTEGER NOT NULL, /* идентификатор языка перевода */
  spec VARCHAR(255) NOT NULL, /* перевод раздела */
  CONSTRAINT spec_translate_list_pk PRIMARY KEY (id)
);

/* + */
/* параметр компонента */
CREATE TABLE param_to_component (
  id SERIAL, /* id параметра компонента */
  id_component INTEGER NOT NULL, /* идентификатор компонента */
  id_param INTEGER NOT NULL, /* идентификатор параметра */
  value VARCHAR(100) NOT NULL, /* параметр компонента */
  CONSTRAINT param_to_component_pk PRIMARY KEY (id)
);

/* + */
/* отслеживание компонента */
CREATE TABLE component_fav_ref (
  id SERIAL, /* id подписки (начала отслеживания) */
  id_component INTEGER NOT NULL, /* идентификатор компонента для отслеживания */
  id_user INTEGER NOT NULL, /* идентификатор профиля */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания/редактирования */
  is_active INTEGER NOT NULL DEFAULT '1', /*  флаг актуальности отслеживания */
  CONSTRAINT component_fav_ref_pk PRIMARY KEY (id)
);

/* +add+ */
/* регион */
CREATE TABLE region_ref (
  id SERIAL, /* id наименования региона */
  region VARCHAR(100) NOT NULL, /* наименование региона */
  CONSTRAINT region_ref_pk PRIMARY KEY (id)
);

/* +add+ */
/* список поставщиков компонента (list shippers) */
CREATE TABLE component_to_user (
  id SERIAL, /* id записи профиля в поставщики компонента */
  id_component INTEGER NOT NULL, /* идентификатор компонента */
  id_user INTEGER NOT NULL, /* идентификатор профиля поставщика */
  comment VARCHAR(255) NOT NULL, /* комментарий к поставщику */
  CONSTRAINT component_to_user_pk PRIMARY KEY (id)
);

/* +add+ */
/* Список модификаций компонента */
CREATE TABLE component_modification_list (
  id SERIAL, /* id компонента */
  id_component INTEGER NOT NULL, /* идентификатор компонента */
  modification_name VARCHAR(100) NOT NULL, /* наименование модификации */
  created_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата создания/загрузки */
  id_name_cad INTEGER NOT NULL, /* соответствующая программа (CAD) */
  comment VARCHAR(2000) NOT NULL, /*  комментарий к модификации */
  id_modification_parent INTEGER NOT NULL DEFAULT '0', /* родительская модификация */
  commentchange VARCHAR(2000) NOT NULL, /*  комментарий с вносимыми изменениями */
  id_actual_status INTEGER NOT NULL, /* номер статуса, к примеру: «актуальный», «архивный», «снято с производства» */
  is_delete INTEGER NOT NULL DEFAULT '0', /* флаг удаления компонента */
  CONSTRAINT component_modification_list_pk PRIMARY KEY (id)
);

/* +add+ */
/* объект/файл модификации */
CREATE TABLE file_to_modification (
  id SERIAL, /* id файла модификации */
  id_modification INTEGER NOT NULL, /* идентификатор модификации */
  id_file INTEGER NOT NULL, /* идентификатор объекта/файла */
  CONSTRAINT file_to_modification_pk PRIMARY KEY (id)
);

/* +add+ */
/* параметр модификации */
CREATE TABLE param_to_modification (
  id SERIAL, /* id параметра модификации */
  id_modification INTEGER NOT NULL, /* идентификатор модификации */
  id_param INTEGER NOT NULL, /* идентификатор параметра */
  value VARCHAR(255) NOT NULL, /* параметр компонента */
  CONSTRAINT param_to_modification_pk PRIMARY KEY (id)
);
