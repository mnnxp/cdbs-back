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
  filename VARCHAR(100) NOT NULL, /* наименование файла */
  id_ext INTEGER NOT NULL, /* расширение файла (используется для определения CAD) */
  filesize FLOAT NOT NULL, /* размер файла */
  path VARCHAR(100) NOT NULL, /* путь к файлу */
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

ALTER TABLE user_ref ADD CONSTRAINT user_ref_fk0 FOREIGN KEY (id_type_org) REFERENCES type_org_ref(id);
ALTER TABLE user_ref ADD CONSTRAINT user_ref_fk1 FOREIGN KEY (id_name_cad) REFERENCES name_cad_ref(id);
ALTER TABLE user_ref ADD CONSTRAINT user_ref_fk2 FOREIGN KEY (id_file_info_icon) REFERENCES file_ref(id);
ALTER TABLE user_ref ADD CONSTRAINT user_ref_fk3 FOREIGN KEY (id_region) REFERENCES region_ref(id);

ALTER TABLE user_tokens_ref ADD CONSTRAINT user_tokens_ref_fk0 FOREIGN KEY (id_user) REFERENCES user_ref(id);

ALTER TABLE spec_ref ADD CONSTRAINT spec_ref_fk0 FOREIGN KEY (id_spec_parent) REFERENCES spec_ref(id);

ALTER TABLE spec_to_user ADD CONSTRAINT spec_to_user_fk0 FOREIGN KEY (id_spec) REFERENCES spec_ref(id);
ALTER TABLE spec_to_user ADD CONSTRAINT spec_to_user_fk1 FOREIGN KEY (id_user) REFERENCES user_ref(id);

ALTER TABLE user_history_list ADD CONSTRAINT user_history_list_fk0 FOREIGN KEY (id_user) REFERENCES user_ref(id);
ALTER TABLE user_history_list ADD CONSTRAINT user_history_list_fk1 FOREIGN KEY (id_type_of_change) REFERENCES type_of_change_ref(id);

ALTER TABLE file_ref ADD CONSTRAINT file_ref_fk0 FOREIGN KEY (id_file) REFERENCES file_ref(id);
ALTER TABLE file_ref ADD CONSTRAINT file_ref_fk1 FOREIGN KEY (id_user_create) REFERENCES user_ref(id);
ALTER TABLE file_ref ADD CONSTRAINT file_ref_fk2 FOREIGN KEY (id_ext) REFERENCES extension_ref(id);

ALTER TABLE extension_ref ADD CONSTRAINT extension_ref_fk0 FOREIGN KEY (id_name_cad) REFERENCES name_cad_ref(id);

ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk0 FOREIGN KEY (id_user) REFERENCES user_ref(id);
ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk1 FOREIGN KEY (id_component_parent) REFERENCES component_ref(id);
ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk2 FOREIGN KEY (id_actual_status) REFERENCES actual_status_ref(id);
ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk3 FOREIGN KEY (id_component_type) REFERENCES component_type_ref(id);
ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk4 FOREIGN KEY (id_type_access) REFERENCES type_access_ref(id);

ALTER TABLE spec_to_component ADD CONSTRAINT spec_to_component_fk0 FOREIGN KEY (id_spec) REFERENCES spec_ref(id);
ALTER TABLE spec_to_component ADD CONSTRAINT spec_to_component_fk1 FOREIGN KEY (id_component) REFERENCES component_ref(id);

ALTER TABLE file_to_component ADD CONSTRAINT file_to_component_fk0 FOREIGN KEY (id_file) REFERENCES file_ref(id);
ALTER TABLE file_to_component ADD CONSTRAINT file_to_component_fk1 FOREIGN KEY (id_component) REFERENCES component_ref(id);

ALTER TABLE component_to_keyword ADD CONSTRAINT component_to_keyword_fk0 FOREIGN KEY (id_component) REFERENCES component_ref(id);
ALTER TABLE component_to_keyword ADD CONSTRAINT component_to_keyword_fk1 FOREIGN KEY (id_component_keyword) REFERENCES component_keyword_ref(id);

ALTER TABLE discussion_ref ADD CONSTRAINT discussion_ref_fk0 FOREIGN KEY (id_component) REFERENCES component_ref(id);
ALTER TABLE discussion_ref ADD CONSTRAINT discussion_ref_fk1 FOREIGN KEY (id_user_from) REFERENCES user_ref(id);
ALTER TABLE discussion_ref ADD CONSTRAINT discussion_ref_fk2 FOREIGN KEY (id_user_to) REFERENCES user_ref(id);
ALTER TABLE discussion_ref ADD CONSTRAINT discussion_ref_fk3 FOREIGN KEY (id_discussion_parent) REFERENCES discussion_ref(id);

ALTER TABLE param_to_component ADD CONSTRAINT param_to_component_fk0 FOREIGN KEY (id_component) REFERENCES component_ref(id);
ALTER TABLE param_to_component ADD CONSTRAINT param_to_component_fk1 FOREIGN KEY (id_param) REFERENCES param_ref(id);

ALTER TABLE component_fav_ref ADD CONSTRAINT component_fav_ref_fk0 FOREIGN KEY (id_component) REFERENCES component_ref(id);
ALTER TABLE component_fav_ref ADD CONSTRAINT component_fav_ref_fk1 FOREIGN KEY (id_user) REFERENCES user_ref(id);

ALTER TABLE spec_translate_list ADD CONSTRAINT spec_translate_list_fk0 FOREIGN KEY (id_spec) REFERENCES spec_ref(id);
ALTER TABLE spec_translate_list ADD CONSTRAINT spec_translate_list_fk1 FOREIGN KEY (id_lang) REFERENCES language_ref(id);

ALTER TABLE param_translate_list ADD CONSTRAINT param_translate_list_fk0 FOREIGN KEY (id_param) REFERENCES param_ref(id);
ALTER TABLE param_translate_list ADD CONSTRAINT param_translate_list_fk1 FOREIGN KEY (id_lang) REFERENCES language_ref(id);

ALTER TABLE user_represet_ref ADD CONSTRAINT user_represet_ref_fk0 FOREIGN KEY (id_user) REFERENCES user_ref(id);
ALTER TABLE user_represet_ref ADD CONSTRAINT user_represet_ref_fk1 FOREIGN KEY (id_representation_type) REFERENCES representation_type_ref(id);
ALTER TABLE user_represet_ref ADD CONSTRAINT user_represet_ref_fk2 FOREIGN KEY (id_region) REFERENCES region_ref(id);

ALTER TABLE component_access_to_user ADD CONSTRAINT component_access_to_user_fk0 FOREIGN KEY (id_component) REFERENCES component_ref(id);
ALTER TABLE component_access_to_user ADD CONSTRAINT component_access_to_user_fk1 FOREIGN KEY (id_user) REFERENCES user_ref(id);
ALTER TABLE component_access_to_user ADD CONSTRAINT component_access_to_user_fk2 FOREIGN KEY (id_type_access) REFERENCES type_access_ref(id);

ALTER TABLE component_to_user ADD CONSTRAINT component_to_user_fk0 FOREIGN KEY (id_component) REFERENCES component_ref(id);
ALTER TABLE component_to_user ADD CONSTRAINT component_to_user_fk1 FOREIGN KEY (id_user) REFERENCES user_ref(id);

ALTER TABLE component_modification_list ADD CONSTRAINT component_modification_list_fk0 FOREIGN KEY (id_component) REFERENCES component_ref(id);
ALTER TABLE component_modification_list ADD CONSTRAINT component_modification_list_fk1 FOREIGN KEY (id_name_cad) REFERENCES name_cad_ref(id);
ALTER TABLE component_modification_list ADD CONSTRAINT component_modification_list_fk2 FOREIGN KEY (id_modification_parent) REFERENCES component_modification_list(id);
ALTER TABLE component_modification_list ADD CONSTRAINT component_modification_list_fk3 FOREIGN KEY (id_actual_status) REFERENCES actual_status_ref(id);

ALTER TABLE file_to_modification ADD CONSTRAINT file_to_modification_fk0 FOREIGN KEY (id_modification) REFERENCES component_modification_list(id);
ALTER TABLE file_to_modification ADD CONSTRAINT file_to_modification_fk1 FOREIGN KEY (id_file) REFERENCES file_ref(id);

ALTER TABLE param_to_modification ADD CONSTRAINT param_to_modification_fk0 FOREIGN KEY (id_modification) REFERENCES component_modification_list(id);
ALTER TABLE param_to_modification ADD CONSTRAINT param_to_modification_fk1 FOREIGN KEY (id_param) REFERENCES param_ref(id);

COMMENT ON TABLE user_ref IS 'профиль';
COMMENT ON COLUMN user_ref.id IS 'id профиля';
COMMENT ON COLUMN user_ref.email IS 'email профиля, на один адрес может быть несколько профилей (закос под reddit)';
COMMENT ON COLUMN user_ref.email_verified IS 'подтверждение email';
COMMENT ON COLUMN user_ref.psw IS 'пароль профиля';
COMMENT ON COLUMN user_ref.id_type_org  IS 'тип профиля (физ. лицо, юр. лицо, ип)';
COMMENT ON COLUMN user_ref.firstname IS 'Имя';
COMMENT ON COLUMN user_ref.lastname IS 'Фамилия';
COMMENT ON COLUMN user_ref.secondname IS 'Отчество';
COMMENT ON COLUMN user_ref.nickname IS 'ник профиля (может использоваться для авторизации)';
COMMENT ON COLUMN user_ref.orgname IS 'наименование организации (для юр.лиц)';
COMMENT ON COLUMN user_ref.shortname IS 'сокращённое наименование организации (для юр.лиц)';
COMMENT ON COLUMN user_ref.inn IS 'инн профиля';
COMMENT ON COLUMN user_ref.phone IS 'номер телефона';
COMMENT ON COLUMN user_ref.id_name_cad IS 'САПР «по умолчанию» (для быстрой загрузки данных)';
COMMENT ON COLUMN user_ref.comment IS 'информация для связи, подпись';
COMMENT ON COLUMN user_ref.address IS 'почтовый адрес';
COMMENT ON COLUMN user_ref.time_zone IS 'часовой пояс профиля';
COMMENT ON COLUMN user_ref.position IS 'роль/должность';
COMMENT ON COLUMN user_ref.site_url IS 'URL адрес сайта профиля';
COMMENT ON COLUMN user_ref.id_file_info_icon IS 'картинка пользователя';
COMMENT ON COLUMN user_ref.id_region IS 'регион пользователя';

COMMENT ON TABLE user_tokens_ref IS 'токен сессии клиента';
COMMENT ON COLUMN user_tokens_ref.id IS 'id токена';
COMMENT ON COLUMN user_tokens_ref.id_user IS 'идентификатор пользователя';
COMMENT ON COLUMN user_tokens_ref.token IS 'токен пользователя';
COMMENT ON COLUMN user_tokens_ref.date_start IS 'дата создания токена';
COMMENT ON COLUMN user_tokens_ref.date_end IS 'дата окончания действия токена';

COMMENT ON TABLE user_represet_ref IS 'локальное представительство профиля';
COMMENT ON COLUMN user_represet_ref.id IS 'id представительства';
COMMENT ON COLUMN user_represet_ref.id_user IS 'id профиля (чьё представительства)';
COMMENT ON COLUMN user_represet_ref.id_region IS 'регион представительства';
COMMENT ON COLUMN user_represet_ref.id_representation_type IS 'тип представительства';
COMMENT ON COLUMN user_represet_ref.name IS 'наименование представительства';
COMMENT ON COLUMN user_represet_ref.address IS 'почтовый адрес представительства';
COMMENT ON COLUMN user_represet_ref.phone IS 'телефон представительства';

COMMENT ON TABLE representation_type_ref IS 'тип представительства профиля';
COMMENT ON COLUMN representation_type_ref.id IS 'id типа представительства';
COMMENT ON COLUMN representation_type_ref._representation_type IS 'наименование типа представительств';

COMMENT ON TABLE type_org_ref IS 'тип профиля';
COMMENT ON COLUMN type_org_ref.id IS 'id типа профиля';
COMMENT ON COLUMN type_org_ref.typeorg IS 'полное наименование (прим. "юридическое лицо")';
COMMENT ON COLUMN type_org_ref.typeorgshort IS 'сокращенное наименование (прим. "юр. лицо")';

COMMENT ON TABLE spec_ref IS 'категории (каталога)';
COMMENT ON COLUMN spec_ref.id IS 'id категории каталога';
COMMENT ON COLUMN spec_ref.spec IS 'наименование категории';
COMMENT ON COLUMN spec_ref.id_spec_parent IS 'id родительского каталога';

COMMENT ON TABLE name_cad_ref IS 'перечень CAD (и других программ)';
COMMENT ON COLUMN name_cad_ref.id IS 'id наименования софта';
COMMENT ON COLUMN name_cad_ref._name_cad IS 'наименование CAD (название программы)';

COMMENT ON TABLE spec_to_user IS 'связь каталогов с профилем';
COMMENT ON COLUMN spec_to_user.id IS 'id связи';
COMMENT ON COLUMN spec_to_user.id_spec IS 'связанный с профилем каталог (категория)';
COMMENT ON COLUMN spec_to_user.id_user IS 'связанный с каталогом (категорией) профиль';

COMMENT ON TABLE user_history_list IS 'запись изменений данных профиля';
COMMENT ON COLUMN user_history_list.id IS 'id события';
COMMENT ON COLUMN user_history_list.id_user IS 'идентификатор профиля к которому относится изменение';
COMMENT ON COLUMN user_history_list.datechange IS 'дата изменения ';
COMMENT ON COLUMN user_history_list.id_type_of_change IS 'id изменения (тип изменения)';
COMMENT ON COLUMN user_history_list.commentchange IS 'комментарий с вносимыми изменениями';

COMMENT ON TABLE type_of_change_ref IS 'перечень типов изменений';
COMMENT ON COLUMN type_of_change_ref.id IS 'id типа изменения';
COMMENT ON COLUMN type_of_change_ref._type_of_change IS 'наименование изменения';

COMMENT ON TABLE file_ref IS 'информация о файле (изображении)';
COMMENT ON COLUMN file_ref.id IS 'id файла';
COMMENT ON COLUMN file_ref.id_file IS 'идентификатор объекта/файла';
COMMENT ON COLUMN file_ref.hash IS 'хеш значение объекта/файла';
COMMENT ON COLUMN file_ref.id_user_create IS 'идентификатор профиля загрузившего файл';
COMMENT ON COLUMN file_ref.created_at IS 'дата создания/загрузки';
COMMENT ON COLUMN file_ref.filename IS 'наименование файла';
COMMENT ON COLUMN file_ref.id_ext IS 'расширение файла (используется для определения CAD)';
COMMENT ON COLUMN file_ref.filesize IS 'размер файла';
COMMENT ON COLUMN file_ref.path IS 'путь к файлу';

COMMENT ON TABLE extension_ref IS 'таблица соответствия CAD и расширений файлов';
COMMENT ON COLUMN extension_ref.id IS 'id соответствия';
COMMENT ON COLUMN extension_ref.extension IS 'расширение файла, одно расширение может быть у нескольких программ';
COMMENT ON COLUMN extension_ref.id_name_cad IS 'соответствующая программа (CAD)';

COMMENT ON TABLE component_ref IS 'компонент';
COMMENT ON COLUMN component_ref.id IS 'id компонента';
COMMENT ON COLUMN component_ref.name IS 'наименование компонента';
COMMENT ON COLUMN component_ref.id_user IS 'идентификатор профиля загрузившего компонент';
COMMENT ON COLUMN component_ref.comment IS 'краткое описание компонента';
COMMENT ON COLUMN component_ref.id_name_cad IS 'соответствующая программа (CAD)';
COMMENT ON COLUMN component_ref.id_component_parent IS 'родительский компонент';
COMMENT ON COLUMN component_ref.id_component_status IS 'номер статуса, к примеру: «актуальный», «архивный», «снято с производства»';
COMMENT ON COLUMN component_ref.id_component_type IS 'тип компонента (базовый/кастомный)';
COMMENT ON COLUMN component_ref.is_delete IS 'флаг удаления компонента';
COMMENT ON COLUMN component_ref.id_type_access IS 'тип доступности компонента';
COMMENT ON COLUMN component_ref.commentchange IS 'комментарий с вносимыми изменениями';
COMMENT ON COLUMN component_ref.is_standard IS 'компонент соответствует стандарту';
COMMENT ON COLUMN component_ref.created_at IS 'дата создания/загрузки';

COMMENT ON TABLE component_access_to_user IS 'доступ к компоненту отдельного пользователя';
COMMENT ON COLUMN component_access_to_user.id IS 'id доступа';
COMMENT ON COLUMN component_access_to_user.id_component IS 'идентификатор компонента';
COMMENT ON COLUMN component_access_to_user.id_user IS 'идентификатор профиля с доступом';
COMMENT ON COLUMN component_access_to_user.id_type_access IS 'тип доступа профиля к компоненту';
COMMENT ON COLUMN component_access_to_user.is_actual IS 'флаг актуальности доступа';
COMMENT ON COLUMN component_access_to_user.is_delete IS 'флаг удаления доступа';
COMMENT ON COLUMN component_access_to_user.created_at IS 'дата создания доступа';

COMMENT ON TABLE actual_status_ref IS 'статус компонента';
COMMENT ON COLUMN actual_status_ref.id IS 'id статуса';
COMMENT ON COLUMN actual_status_ref.componentstatus IS 'к примеру: «актуальный», «архивный», «снято с производства»';

COMMENT ON TABLE spec_to_component IS 'каталог компонента';
COMMENT ON COLUMN spec_to_component.id IS 'id связи компонента с каталогом';
COMMENT ON COLUMN spec_to_component.id_spec IS 'идентификатор позиции в каталоге';
COMMENT ON COLUMN spec_to_component.id_component IS 'идентификатор компонента';

COMMENT ON TABLE file_to_component IS 'объект/файл компонента';
COMMENT ON COLUMN file_to_component.id IS 'id файла компонента';
COMMENT ON COLUMN file_to_component._c_o_l_l IS 'идентификатор объекта/файла';
COMMENT ON COLUMN file_to_component._c_o_l_l IS 'идентификатор компонента';

COMMENT ON TABLE component_type_ref IS 'тип компонента (базовый, кастомный)';
COMMENT ON COLUMN component_type_ref.id IS 'id типа';
COMMENT ON COLUMN component_type_ref._component_type IS 'наименование типа';

COMMENT ON TABLE component_keyword_ref IS 'ключевые слова компонента (тегирование)';
COMMENT ON COLUMN component_keyword_ref.id IS 'id тега';
COMMENT ON COLUMN component_keyword_ref.keyword IS 'ключевое слово';

COMMENT ON TABLE component_to_keyword IS 'ключевые слова связанные с компонентом (тегирование)';
COMMENT ON COLUMN component_to_keyword.id IS 'id связи тега и компонента';
COMMENT ON COLUMN component_to_keyword.id_component IS 'идентификатор компонента';
COMMENT ON COLUMN component_to_keyword.id_component_keyword IS 'идентификатор ключевого слова (тега)';

COMMENT ON TABLE type_access_ref IS 'типы доступа';
COMMENT ON COLUMN type_access_ref.id IS 'id типа доступа';
COMMENT ON COLUMN type_access_ref._type_access IS 'наименование доступа';

COMMENT ON TABLE discussion_ref IS 'обсуждение компонента';
COMMENT ON COLUMN discussion_ref.id IS 'id комментария';
COMMENT ON COLUMN discussion_ref.created_at IS 'дата создания/редактирования';
COMMENT ON COLUMN discussion_ref.id_component IS 'идентификатор компонента';
COMMENT ON COLUMN discussion_ref.id_user_from IS 'идентификатор профиля отправителя';
COMMENT ON COLUMN discussion_ref.id_user_to IS 'идентификатор профиля адресата';
COMMENT ON COLUMN discussion_ref.comment IS 'сообщение/комментарий';
COMMENT ON COLUMN discussion_ref.id_discussion_parent IS 'id родительского комментария';

COMMENT ON TABLE param_ref IS 'параметры для компонента';
COMMENT ON COLUMN param_ref.id IS 'id параметра (характеристики)';
COMMENT ON COLUMN param_ref.paramname IS 'наименование параметра';

COMMENT ON TABLE language_ref IS 'перевод параметра (язык)';
COMMENT ON COLUMN language_ref.id IS 'id языка';
COMMENT ON COLUMN language_ref.lang IS 'полное наименование языка';
COMMENT ON COLUMN language_ref.langshort IS 'краткое наименование языка';

COMMENT ON TABLE param_translate_list IS 'перевод параметра (перевод)';
COMMENT ON COLUMN param_translate_list.id IS 'id перевода';
COMMENT ON COLUMN param_translate_list.id_param IS 'идентификатор параметра';
COMMENT ON COLUMN param_translate_list.id_lang IS 'идентификатор языка перевода';
COMMENT ON COLUMN param_translate_list.param IS 'перевод параметра';

COMMENT ON TABLE spec_translate_list IS 'перевод раздела каталога';
COMMENT ON COLUMN spec_translate_list.id IS 'id перевода';
COMMENT ON COLUMN spec_translate_list.id_spec IS 'идентификатор раздела';
COMMENT ON COLUMN spec_translate_list.id_lang IS 'идентификатор языка перевода';
COMMENT ON COLUMN spec_translate_list.spec IS 'перевод раздела';

COMMENT ON TABLE param_to_component IS 'параметр компонента';
COMMENT ON COLUMN param_to_component.id IS 'id параметра компонента';
COMMENT ON COLUMN param_to_component.id_component IS 'идентификатор компонента';
COMMENT ON COLUMN param_to_component.id_param IS 'идентификатор параметра';
COMMENT ON COLUMN param_to_component.value IS 'параметр компонента';

COMMENT ON TABLE component_fav_ref IS 'отслеживание компонента';
COMMENT ON COLUMN component_fav_ref.id IS 'id подписки (начала отслеживания)';
COMMENT ON COLUMN component_fav_ref.id_component IS 'идентификатор компонента для отслеживания';
COMMENT ON COLUMN component_fav_ref.id_user IS 'идентификатор профиля';
COMMENT ON COLUMN component_fav_ref.created_at IS 'дата создания/редактирования';
COMMENT ON COLUMN component_fav_ref.is_active IS 'флаг актуальности отслеживания';

COMMENT ON TABLE region_ref IS 'регион'
COMMENT ON COLUMN region_ref.id IS 'id наименования региона'
COMMENT ON COLUMN region_ref.region IS 'наименование региона'

COMMENT ON TABLE component_to_user IS 'список поставщиков компонента'
COMMENT ON COLUMN component_to_user.id IS 'id записи профиля в поставщики компонента'
COMMENT ON COLUMN component_to_user.id_component IS 'идентификатор компонента'
COMMENT ON COLUMN component_to_user.id_user IS 'идентификатор профиля поставщика'
COMMENT ON COLUMN component_to_user.comment IS 'комментарий к поставщику'

COMMENT ON TABLE component_modification_list IS 'список модификаций компонента'
COMMENT ON COLUMN component_modification_list.id IS 'id компонента'
COMMENT ON COLUMN component_modification_list.id_component IS 'идентификатор компонента'
COMMENT ON COLUMN component_modification_list._modification_name IS 'наименование модификации'
COMMENT ON COLUMN component_modification_list.created_at IS 'дата создания/загрузки'
COMMENT ON COLUMN component_modification_list.id_name_cad IS 'соответствующая программа (CAD)'
COMMENT ON COLUMN component_modification_list.comment IS 'комментарий к модификации'
COMMENT ON COLUMN component_modification_list.id_modification_parent IS 'родительская модификация'
COMMENT ON COLUMN component_modification_list.commentchange IS 'комментарий с вносимыми изменениями'
COMMENT ON COLUMN component_modification_list.id_actual_status IS 'номер статуса, к примеру: «актуальный», «архивный», «снято с производства»'
COMMENT ON COLUMN component_modification_list.is_delete IS 'флаг удаления компонента'

COMMENT ON TABLE file_to_modification IS 'объект/файл модификации'
COMMENT ON COLUMN file_to_modification.id IS 'id файла модификации'
COMMENT ON COLUMN file_to_modification.id_modification IS 'идентификатор модификации'
COMMENT ON COLUMN file_to_modification.id_file IS 'идентификатор объекта/файла'

COMMENT ON TABLE param_to_modification IS 'параметр модификации'
COMMENT ON COLUMN param_to_modification.id IS 'id параметра модификации'
COMMENT ON COLUMN param_to_modification.id_modification IS 'идентификатор модификации'
COMMENT ON COLUMN param_to_modification.id_param IS 'идентификатор параметра'
COMMENT ON COLUMN param_to_modification.value IS 'параметр компонента'
