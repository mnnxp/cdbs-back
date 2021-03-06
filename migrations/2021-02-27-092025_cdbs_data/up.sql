-- TABLE: actual_status_ref: id (SERIAL), actualstatus (VARCHAR(100))
INSERT INTO actual_status_ref (actualstatus) VALUES
    ('актуальный'),
    ('архивный'),
    ('снят с производства');

-- TABLE: component_fav_ref: id (SERIAL), id_component (INTEGER),
-- id_user (INTEGER), created_at (TIMESTAMP), is_active (INTEGER),
INSERT INTO component_fav_ref (id_component, id_user, created_at, is_active) VALUES
    (1, 1, now(), 1);

-- TABLE: component_keyword_ref: keyword (VARCHAR(10))
INSERT INTO component_keyword_ref (keyword) VALUES
    ('keyword');

-- TABLE: component_ref: id SERIAL, name VARCHAR(225), id_user INTEGER,
-- comment VARCHAR(2000), id_component_parent INTEGER, id_actual_status INTEGER,
-- id_component_type INTEGER, is_delete INTEGER, id_type_access INTEGER,
-- commentchange VARCHAR(2000), is_standard INTEGER, created_at TIMESTAMP
INSERT INTO component_ref (name, id_user, comment, id_component_parent,
  id_actual_status, id_component_type, is_delete, id_type_access,
  commentchange, is_standard, created_at) VALUES
  ('Reduced shank bolts and screws with coarse thread - Head style C - Type H',
    1, 'Continuously improve the product quality and applicability...',
    1, 1, 1, 1, 1, 'no', 1, now());

-- TABLE: component_type_ref: id (SERIAL), component_type (VARCHAR(225))
INSERT INTO component_type_ref (component_type) VALUES
  ('базовый'),
  ('собственный');

-- TABLE: discussion_ref: id (SERIAL), created_at (TIMESTAMP),
-- id_component (INTEGER), id_user_from (INTEGER), id_user_to (INTEGER),
-- comment (VARCHAR(2000)), id_discussion_parent (INTEGER)
INSERT INTO discussion_ref (created_at, id_component, id_user_from,
  id_user_to, comment, id_discussion_parent) VALUES
  (now(), 1, 1, 1, 'this comment', 1);

-- TABLE: extension_ref: id (SERIAL), extension (VARCHAR(10)), id_name_cad (INTEGER)
INSERT INTO extension_ref (extension, id_name_cad) VALUES
    ('.3dm', 1),
    ('.3ds', 1),
    ('.a2c', 2),
    ('.apt', 3),
    ('.asm', 4),
    ('.asm', 5),
    ('.asm', 6),
    ('.asp', 7),
    ('.asp', 8),
    ('.ccd', 9),
    ('.cdd', 10),
    ('.cdw', 11),
    ('.cdw', 12),
    ('.cpp', 13),
    ('.cr2', 1),
    ('.dgn', 1),
    ('.dtd', 1),
    ('.dwg', 1),
    ('.dwg', 1),
    ('.dxf', 1),
    ('.ics', 1),
    ('.igs', 1),
    ('.igs', 1),
    ('.iso', 1),
    ('.iso', 1),
    ('.ma', 1),
    ('.max', 1),
    ('.max', 1),
    ('.mb', 1),
    ('.mod', 1),
    ('.mod', 1),
    ('.mod', 1),
    ('.mod', 1),
    ('.mts', 1),
    ('.obj', 1),
    ('.part', 1),
    ('.pkg', 1),
    ('.prj', 1),
    ('.pwi', 1),
    ('.rnd', 1),
    ('.sldasm', 1),
    ('.sldprt', 1),
    ('.stl', 1),
    ('.vcd', 1),
    ('.vob', 1),
    ('.wm', 1),
    ('.wm2d', 1);

-- TABLE: file_ref: id (serial), id_file (integer), hash (bytea), id_user_create (integer),
--           created_at (Timestamp), filename (varying(225)), id_ext (integer),
--           filesize (double precision), path_file (varying(225)),
INSERT INTO file_ref (id_file, id_user_create, created_at, filename, id_ext, filesize , path_file) VALUES
    -- (1, E'\\000', 1, now(), 'filename', 1, 0, 'path file');
    (1, 1, now(), 'filename', 1, 0, 'path/file/file.txt');

-- TABLE: language_ref: id (SERIAL), lang (VARCHAR(100)), langshort (VARCHAR(10))
INSERT INTO language_ref (lang, langshort) VALUES
    ('Russian', 'RU'),
    ('English', 'EN');

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

-- TABLE: param_ref: id (serial), paramname (VARCHAR(100))
INSERT INTO param_ref (paramname) VALUES
    ('Nominal Ø'),
    ('Pitch (mm)'),
    ('Length (mm)'),
    ('Head height (mm)');

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

-- TABLE: representation_type_ref:  id (serial), representation_type (VARCHAR(100))
INSERT INTO representation_type_ref (representation_type) VALUES
    ('Представительство');

-- TABLE: spec_ref: id (serial), spec (VARCHAR(100)), id_spec_parent (INTEGER)
INSERT INTO spec_ref (spec, id_spec_parent) VALUES
    ('main', 1);

-- TABLE: type_access_ref: id (serial), type_access (VARCHAR(100))
INSERT INTO type_access_ref (type_access) VALUES
  ('Полный'),
  ('Частичный'),
  ('Закрыт');

-- TABLE: type_of_change_ref: id (serial), type_of_change (VARCHAR(100))
INSERT INTO type_of_change_ref (type_of_change) VALUES
  ('Изменение данных профиля');


-- TABLE: type_user_ref: id (serial), typeuser (varying(100)), typeusershort (varying(10))
INSERT INTO type_user_ref (typeuser, typeusershort) VALUES
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

-- TABLE: user_ref:
-- uuid (UUID), email (VARCHAR(100)), email_verified (INTEGER),
-- psw_hash (BYTEA), psw_salt (VARCHAR(255)), id_type_user (INTEGER),
-- firstname (VARCHAR(100)), lastname (VARCHAR(100)), secondname (VARCHAR(100)),
-- nickname (VARCHAR(100)), orgname (VARCHAR(255)), shortname (VARCHAR(255)),
-- inn (VARCHAR(30)), phone (VARCHAR(100)), id_name_cad (INTEGER),
-- comment (VARCHAR(2000)), address (VARCHAR(512)), time_zone (VARCHAR(255)),
-- position (VARCHAR(255)), site_url (VARCHAR(255)), id_file_info_icon (INTEGER),
-- id_region (INTEGER), created_at (TIMESTAMP),
INSERT INTO user_ref (uuid, email, email_verified, psw_hash, psw_salt,
  id_type_user, firstname, lastname, secondname, nickname, orgname, shortname,
  inn, phone, id_name_cad, comment, address, time_zone, position, site_url,
  id_file_info_icon, id_region, created_at) VALUES
    ('31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 'email@email.ru', 1, E'\\000',
      '000', 1, 'Johm', 'Ivanov', 'Rucovich', 'nicknameeee', 'romashka', 'rom-ka',
      '12345678910', '+79991234567', 1, 'comment for this user', 'Moscow',
      'UTC+3', 'engineer', 'https://cadbase.ru', 1, 1, now());


-- TABLE: user_represet_ref: id (serial), id_user (INTEGER),
-- id_region (INTEGER),   id_representation_type (INTEGER),
-- name (VARCHAR(255)),   address (VARCHAR(512)),   phone (VARCHAR(100))
INSERT INTO user_represet_ref (id_user, id_region, id_representation_type,
  name, address, phone) VALUES
  (1, 1, 1, 'Местный офис', 'г. Москва', '+79991234567');

-- TABLE: user_tokens_ref: id (serial), id_user (INTEGER), token (VARCHAR(512)),
-- date_start (TIMESTAMP), date_end (TIMESTAMP)
INSERT INTO user_tokens_ref (id_user, token, date_start, date_end) VALUES
  (1, 'GNLw1GKzykA926Rhdcpy1c6lugZXTd5y', now(), now());
