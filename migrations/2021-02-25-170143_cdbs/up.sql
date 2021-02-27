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

-- TABLE: type_org_ref: id (serial), typeorg (varying(100)), typeorgshort (varying(10))
INSERT INTO type_org_ref (typeorg, typeorgshort) VALUES
    ('Физическое лицо', 'Физ.лицо'),
    ('Индивидуальный предприниматель', 'ИП'),
    ('Акционерные общества', 'АО'),
    ('Публичные акционерные общества', 'ПАО'),
    ('Непубличные акционерные общества', 'НАО'),
    ('Общества с ограниченной ответственностью', 'ООО'),
    ('Хозяйственные партнерства', 'Х.парт-ва'),
    ('Производственные кооперативы (артели)', 'Артель'),
    ('Сельскохозяйственные производственные кооперативы', 'Сельхоз'),
    ('Кооперативные хозяйства (коопхозы)', 'Коопхоз'),
    ('Прочие юридические лица, являющиеся коммерческими организациями', 'Прочие');

-- TABLE: name_cad_ref: id (serial), name_cad (varying(225))
INSERT INTO name_cad_ref (name_cad) VALUES
    ('AutoCAD'),
    ('BricsCAD'),
    ('CATIA V4'),
    ('CATIA V5'),
    ('COLLADA'),
    ('Creo'),
    ('DesignSpark Mechanical'),
    ('DraftSight'),
    ('DXF - 2D'),
    ('DXF - 3D'),
    ('EMF'),
    ('FUSION 360'),
    ('GstarCAD'),
    ('HiCAD'),
    ('HOOPS'),
    ('IGES'),
    ('Inventor'),
    ('Inventor LT'),
    ('IRONCAD'),
    ('KOMPAS-3D'),
    ('Mechanical Desktop'),
    ('NX'),
    ('OBJ'),
    ('OFF'),
    ('Panda3D'),
    ('Parasolid 11.1'),
    ('Pro/Engineer Neutral'),
    ('Revit'),
    ('SketchUp'),
    ('Solid Edge'),
    ('SOLIDWORKS'),
    ('SpaceClaim'),
    ('STEP AP203'),
    ('STEP AP214'),
    ('STEP AP242'),
    ('STL'),
    ('T-FLEX'),
    ('Tekla'),
    ('TENADO CAD 3D'),
    ('Three.js'),
    ('TopSolid'),
    ('TurboCAD'),
    ('Universal 3D'),
    ('VDA-FS'),
    ('VRML'),
    ('VTK'),
    ('VX CAD/CAM'),
    ('WMF'),
    ('ZW3D');

    -- TABLE: region_ref: id (serial), region (varying(100))
    INSERT INTO region_ref (region) VALUES
      ('Республика Адыгея (Адыгея)'),
      ('Республика Башкортостан'),
      ('Республика Бурятия'),
      ('Республика Алтай'),
      ('Республика Дагестан'),
      ('Республика Ингушетия'),
      ('Кабардино-Балкарская Республика'),
      ('Республика Калмыкия'),
      ('Карачаево-Черкесская Республика'),
      ('Республика Карелия'),
      ('Республика Коми'),
      ('Республика Марий Эл'),
      ('Республика Мордовия'),
      ('Республика Саха (Якутия)'),
      ('Республика Северная Осетия - Алания'),
      ('Республика Татарстан (Татарстан)'),
      ('Республика Тыва'),
      ('Удмуртская Республика'),
      ('Республика Хакасия'),
      ('Чеченская Республика'),
      ('Чувашская Республика - Чувашия'),
      ('Алтайский край'),
      ('Краснодарский край'),
      ('Красноярский край'),
      ('Приморский край'),
      ('Ставропольский край'),
      ('Хабаровский край'),
      ('Амурская область'),
      ('Архангельская область'),
      ('Астраханская область'),
      ('Белгородская область'),
      ('Брянская область'),
      ('Владимирская область'),
      ('Волгоградская область'),
      ('Вологодская область'),
      ('Воронежская область'),
      ('Ивановская область'),
      ('Иркутская область'),
      ('Калининградская область'),
      ('Калужская область'),
      ('Камчатский край'),
      ('Кемеровская область - Кузбасс'),
      ('Кировская область'),
      ('Костромская область'),
      ('Курганская область'),
      ('Курская область'),
      ('Ленинградская область'),
      ('Липецкая область'),
      ('Магаданская область'),
      ('Московская область'),
      ('Мурманская область'),
      ('Нижегородская область'),
      ('Новгородская область'),
      ('Новосибирская область'),
      ('Омская область'),
      ('Оренбургская область'),
      ('Орловская область'),
      ('Пензенская область'),
      ('Пермский край'),
      ('Псковская область'),
      ('Ростовская область'),
      ('Рязанская область'),
      ('Самарская область'),
      ('Саратовская область'),
      ('Сахалинская область'),
      ('Свердловская область'),
      ('Смоленская область'),
      ('Тамбовская область'),
      ('Тверская область'),
      ('Томская область'),
      ('Тульская область'),
      ('Тюменская область'),
      ('Ульяновская область'),
      ('Челябинская область'),
      ('Забайкальский край'),
      ('Ярославская область'),
      ('г. Москва'),
      ('Санкт-Петербург'),
      ('Еврейская автономная область'),
      ('Ненецкий автономный округ'),
      ('Ханты-Мансийский автономный округ - Югра'),
      ('Чукотский автономный округ'),
      ('Ямало-Ненецкий автономный округ'),
      ('Республика Крым'),
      ('Севастополь'),
      ('Иные территории, включая город и космодром Байконур');

-- TABLE: user_ref:
-- uuid (UUID), email (VARCHAR(100)), email_verified (INTEGER),
-- psw_hash (BYTEA), psw_salt (VARCHAR(255)), id_type_org (INTEGER),
-- firstname (VARCHAR(100)), lastname (VARCHAR(100)), secondname (VARCHAR(100)),
-- nickname (VARCHAR(100)), orgname (VARCHAR(255)), shortname (VARCHAR(255)),
-- inn (VARCHAR(30)), phone (VARCHAR(100)), id_name_cad (INTEGER),
-- comment (VARCHAR(2000)), address (VARCHAR(512)), time_zone (VARCHAR(255)),
-- position (VARCHAR(255)), site_url (VARCHAR(255)), id_file_info_icon (INTEGER),
-- id_region (INTEGER), created_at (TIMESTAMP),
INSERT INTO user_ref (uuid, email, email_verified, psw_hash, psw_salt,
  id_type_org, firstname, lastname, secondname, nickname, orgname, shortname,
  inn, phone, id_name_cad, comment, address, time_zone, position, site_url,
  id_file_info_icon, id_region, created_at) VALUES
    ('31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 'email@email.ru', 0, E'\\000',
      '000', 0, 'Johm', 'Ivanov', 'Rucovich', 'nicknameeee', 'romashka', 'rom-ka',
      '12345678910', '+7999123456', 0, 'comment for this user', 'Moscow',
      'UTC+3', 'engineer', 'https://cadbase.ru', 0, 0, now());

-- TABLE: file_ref: id (serial), id_file (integer), hash (bytea), id_user_create (integer),
--           created_at (Timestamp), filename (varying(225)), id_ext (integer),
--           filesize (double precision), path_file (varying(225)),
INSERT INTO file_ref (id_file, hash, id_user_create, created_at, filename, id_ext, filesize , path_file) VALUES
    (0, E'\\000', 0, now(), 0, 0, 0, 0);



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
