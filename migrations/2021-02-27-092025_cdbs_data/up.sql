-- TABLE: actual_status_ref: id (SERIAL), actualstatus (VARCHAR(100))
INSERT INTO actual_status_ref (actualstatus) VALUES
    ('актуальный'),
    ('архивный'),
    ('снят с производства');

-- TABLE: component_fav_ref: id (SERIAL), id_component (INTEGER),
-- id_user (INTEGER), created_at (TIMESTAMP), is_active (INTEGER),
INSERT INTO component_fav_ref (uuid_component, uuid_user, created_at, is_active) VALUES
    ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', now(), 1);

-- TABLE: component_keyword_ref: keyword (VARCHAR(10))
INSERT INTO component_keyword_ref (keyword) VALUES
    ('tools'),
    ('bolt'),
    ('screw'),
    ('automotive'),
    ('autodesk'),
    ('fusion'),
    ('tech');

-- TABLE: component_ref: id SERIAL, name VARCHAR(225), id_user INTEGER,
-- comment VARCHAR(2000), id_component_parent INTEGER, id_actual_status INTEGER,
-- id_component_type INTEGER, is_delete INTEGER, id_type_access INTEGER,
-- commentchange VARCHAR(2000), is_standard INTEGER, created_at TIMESTAMP
INSERT INTO component_ref (uuid, name, uuid_user, comment, uuid_component_parent,
  id_actual_status, id_component_type, is_delete, id_type_access,
  commentchange, is_standard, created_at) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 'Reduced shank bolts and screws with coarse thread',
    '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 'Continuously improve the product quality and applicability...',
    'a5953fd9-7393-4f1e-a899-06b5e159dbf1', 1, 1, 0, 1, 'no', 1, now()),
  ('e925833e-f8d3-4ecb-bd67-5aa450f9f0ad', 'Knobs 123 Inch',
    '68b8281a-d19c-4d4b-88eb-6fd4a2afde1b', 'Plastic...',
    'a5953fd9-7393-4f1e-a899-06b5e159dbf1', 1, 1, 0, 1, 'Not change', 0, now());

  -- TABLE: spec_to_component: id (SERIAL), id_spec (INTEGER),
  -- id_component (INTEGER)
  INSERT INTO spec_to_component (id_spec, uuid_component) VALUES
    (1, 'a5953fd9-7393-4f1e-a899-06b5e159dbf1');

  -- TABLE: component_to_keyword: id (SERIAL), id_component (INTEGER),
  -- id_component_keyword (INTEGER)
  INSERT INTO component_to_keyword (uuid_component, id_component_keyword) VALUES
    ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 1),
    ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 2),
    ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 3);

-- TABLE: param_to_component: id SERIAL, id_component INTEGER,
-- id_param INTEGER, value VARCHAR(255)
INSERT INTO param_to_component (uuid_component, id_param, value) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 9, '1'),
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 10, '0.5'),
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 11, '1.25'),
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 12, '2.51');

-- TABLE: component_type_ref: id (SERIAL), component_type (VARCHAR(225))
INSERT INTO component_type_ref (component_type) VALUES
  ('базовый'),
  ('собственный');

-- TABLE: file_to_component: id (SERIAL), id_component (INTEGER),
-- uuid_file (INTEGER)
INSERT INTO file_to_component (uuid_component, uuid_file) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 'bc1c2151-86d0-4656-9c9d-d016dd584297');

-- TABLE: file_to_modification: id (SERIAL), id_modification (INTEGER),
-- uuid_file (INTEGER)
INSERT INTO file_to_modification (uuid_modification, uuid_file) VALUES
  ('aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 'bc1c2151-86d0-4656-9c9d-d016dd584297');

-- TABLE: component_modification_list: id SERIAL, id_component INTEGER,
-- modification_name VARCHAR(100), created_at TIMESTAMP, id_name_cad INTEGER,
-- comment VARCHAR(2000), id_modification_parent INTEGER, commentchange VARCHAR(2000),
-- id_actual_status INTEGER, is_delete INTEGER)
INSERT INTO component_modification_list (uuid, uuid_component, modification_name,
  created_at, id_name_cad, comment, uuid_modification_parent, commentchange,
  id_actual_status, is_delete) VALUES
  ('aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 'a5953fd9-7393-4f1e-a899-06b5e159dbf1',
    'Head style C - Type H', now(), 1, 'main modification', 'aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 'comment change', 1, 0);

-- TABLE: param_to_modification: id SERIAL, id_modification INTEGER,
-- id_param INTEGER, value VARCHAR(255)
INSERT INTO param_to_modification (uuid_modification, id_param, value) VALUES
  ('aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 1, '1'),
  ('aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 2, '1'),
  ('aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 3, 'SMS8'),
  ('aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 4, 'SMS8 Self-Drilling and Tapping Screw, #8 Screw, 1/2" Screw'),
  ('aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 5, 'SMS8'),
  ('aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 6, 'Steel'),
  ('aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 7, 'Electrogalvanized'),
  ('aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 8, '187197');

-- TABLE: component_to_user: id (SERIAL), id_component (INTEGER),
-- id_user (INTEGER), comment (VARCHAR(255)),
INSERT INTO component_to_user (uuid_component, uuid_user, comment) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 'Комментарий поставщика');

-- TABLE: discussion_ref: id (SERIAL), created_at (TIMESTAMP),
-- id_component (INTEGER), id_user_from (INTEGER), id_user_to (INTEGER),
-- comment (VARCHAR(2000)), id_discussion_parent (INTEGER)
INSERT INTO discussion_ref (created_at, uuid_component, uuid_user_from,
  uuid_user_to, comment, id_discussion_parent) VALUES
  (now(), 'a5953fd9-7393-4f1e-a899-06b5e159dbf1', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b',
  '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 'this comment', 1);

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

-- TABLE: : id (serial), uuid_file_parent (integer), hash (bytea), id_user_create (integer),
--           created_at (Timestamp), filename (varying(225)), id_ext (integer),
--           filesize (double precision), path_file (varying(225)),
INSERT INTO file_ref (uuid, uuid_file_parent, hash, uuid_user_create, created_at, filename, id_ext, filesize , path_file) VALUES
    ('bc1c2151-86d0-4656-9c9d-d016dd584297', 'bc1c2151-86d0-4656-9c9d-d016dd584297', E'\\000', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', now(), 'filename', 1, 0, 'path/file/file.txt'),
    ('3706d1a1-80ae-4367-be39-af7091373811', 'bc1c2151-86d0-4656-9c9d-d016dd584297', E'\\xe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855',
      '68b8281a-d19c-4d4b-88eb-6fd4a2afde1b', now(), 'file_child_two.pdd', 1, 136, '/sholder/file/f1c5a362-55f9-4edb-ad90-a2ea64d586df');

-- TABLE: language_ref: id (SERIAL), lang (VARCHAR(100)), langshort (VARCHAR(10))
INSERT INTO language_ref (lang, langshort) VALUES
    ('Russian', 'RU'),
    ('English', 'EN');

-- TABLE: spec_translate_list: id (serial), id_spec (INTEGER), id_lang (INTEGER),
-- spec (VARCHAR(225))
INSERT INTO spec_translate_list (id_spec, id_lang, spec) VALUES
  (1, 1, 'Root');

-- TABLE: param_translate_list: id (serial), id_param (INTEGER), id_lang (INTEGER),
-- param (VARCHAR(225))
INSERT INTO param_translate_list (id_param, id_lang, param) VALUES
  (1, 1, 'Индекс');

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
    ('Index'),
    ('Selector'),
    ('Part Number'),
    ('Description'),
    ('E-Shop link'),
    ('Material'),
    ('Finish'),
    ('Article Number'),
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
    ('main', 1),
    ('Components', 1),
    ('Construction', 2),
    ('Tools', 2);

-- TABLE: component_access_to_user: id SERIAL, id_component INTEGER, id_user INTEGER,
-- id_type_access INTEGER, is_actual INTEGER, is_delete INTEGER, created_at TIMESTAMP
INSERT INTO component_access_to_user (uuid_component, uuid_user, id_type_access,
is_actual, is_delete, created_at) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 1, 1, 0, now());

-- TABLE: type_access_ref: id (serial), type_access (VARCHAR(100))
INSERT INTO type_access_ref (type_access) VALUES
  ('Полный'),
  ('Частичный'),
  ('Закрыт');

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
-- comment (VARCHAR(2000)), address (VARCHAR(512)), time_zone (INTEGER),
-- position (VARCHAR(255)), site_url (VARCHAR(255)), uuid_file_info_icon (INTEGER),
-- id_region (INTEGER), created_at (TIMESTAMP),
INSERT INTO user_ref (uuid, email, email_verified, psw_hash, psw_salt,
  id_type_user, is_supplier, firstname, lastname, secondname, nickname, orgname, shortname,
  inn, phone, id_name_cad, comment, address, time_zone, position, site_url,
  uuid_file_info_icon, id_region, created_at) VALUES
    ('31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 'email@email.ru', 1,
      E'\\xc3747b782c0b5c13cb1257a951d5120cd6a958f3516ba5d40c1db1c1eae99b15',
      '8%8lDv&TB!295%cNWDmghT5lNDSxTUxUgRY6xNw^hACP!DDDK8IKNLP)0Hr(C7m55BQDr&L%V0F^~3O&J~QPQDfJ$&uDjwUwPShyK0B4yDhXcBe^cPoV@%^gax^%z)92',
      2, 1, 'Johm', 'Ivanov', 'Rucovich', 'nicknameeee', 'romashka', 'rom-ka', '12345678910', '+79991234567',
      1, 'comment for this user', 'Moscow',
      3, 'manufacturer', 'https://cadbase.ru',
      'bc1c2151-86d0-4656-9c9d-d016dd584297', 1, now()),
    ('68b8281a-d19c-4d4b-88eb-6fd4a2afde1b', 'bname@somain.com', 0,
      E'\\x085f06287c840b5c23578912d5e0cc2e4baf53865e9170a05ed084c1292ab7f4',
      'pKFpenRqOFyutR#OAkxb%!bi%mV5q(GPKgHmwQ*bWrcuJHC3k8raBNzUnw7r%^oFKzBf%McZlVBI#O@U1@JApg@rVHEuzlybCWx&BXjrI(41x)8kR9rjURVG9lqr0EIM',
      1, 0, 'Vans', 'Bpero', 'Nado', 'albane', 'none', 'none', 'none', 'none',
      4, 'none', 'noneadress',
      2, 'engineer', 'none',
      'bc1c2151-86d0-4656-9c9d-d016dd584297', 1, now()),
    ('e97ea679-4560-4a9b-ad8b-80d2d1912602', 'testemail@testemail.ru', 0,
      E'\\xc0a6617d8971cac49489078028b6ea4bd6f7929f6e542a1de5cc4c2a0f6fdfce',
      'a13%A1r9kCmDHieCl(^Yt$~traAIlnTM(0#vHvjE&tZ@9Cm2OJKCKENu6&a2pTrd*Z%qQyiYXX@fG2j7XeBLx4FYY9tkSK*B^yV)0s$sQ!y)qBL#!RDxcxZLPEKD5@lg',
      3, 0, 'testfirstname', 'testlastname', 'testsecondname', 'testingname', 'testorgname', 'testshortname', 'testinn', '+1234567890',
      5, 'testcomment', 'testaddress',
      2, 'testposition', 'testsiteUrl',
      'bc1c2151-86d0-4656-9c9d-d016dd584297', 13, now()),
    ('c3f5f69c-bb54-45d9-bfa7-1d28cc1afa5a', 'bname@somain.com', 0,
      E'\\xfdeba1b9304208da90c90ddb6fe0ee49787b085636b26f156d6240869bdb3665',
      'ThmlF#HCNEX6%##AFGH(%0Tdo0w$5kh(WA9%@KwHe3mrVlhMIj~NxeiiJyh~Ty1t(J3F#lGDWFFJRZnV1&WFMA%Rl5~8qvfW)5WjL&qm%jSuFx1Uslth^a$64YkJjN)q',
      1, 0, 'Vans', 'Bpero', 'Nado', 'testusertext', 'none', 'none', 'none', 'none',
      4, 'none', 'noneadress',
      6, 'noneposition', 'none',
      'bc1c2151-86d0-4656-9c9d-d016dd584297', 1, now());

-- TABLE: user_represet_ref: id (serial), id_user (INTEGER),
-- id_region (INTEGER),   id_representation_type (INTEGER),
-- name (VARCHAR(255)),   address (VARCHAR(512)),   phone (VARCHAR(100))
INSERT INTO user_represet_ref (uuid, uuid_user, id_region, id_representation_type,
  name, address, phone) VALUES
  ('22a08149-b63a-4f65-a1cd-4a28f85567ec', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 1, 1, 'Местный офис', 'г. Москва', '+79991234567'),
  ('297e44b3-d36c-4ab5-be16-e2a955aabf13', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 2, 1, 'additional office', 'Batkov District, Minsk', '+375548418789'),
  ('96df6359-a31e-40ad-aa06-9355abc2cc58', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 3, 1, 'additional office', 'None str, Kiev', '+380874487556');

-- TABLE: user_tokens_ref: id (serial), id_user (INTEGER), token (VARCHAR(512)),
-- date_start (TIMESTAMP), date_end (TIMESTAMP)
INSERT INTO user_tokens_ref (uuid_user, token, date_start, date_end) VALUES
  ('31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 'GNLw1GKzykA926Rhdcpy1c6lugZXTd5y', now(), now());

-- TABLE: user_history_list: id (serial), id_user (INTEGER), datechange (TIMESTAMP),
-- id_type_of_change (INTEGER), commentchange (VARCHAR(2000))
INSERT INTO user_history_list (uuid_user, datechange, id_type_of_change, commentchange) VALUES
  ('31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', now(), 1, 'Комментарий к изменению');

-- TABLE: type_of_change_ref: id (serial), type_of_change (VARCHAR(100))
INSERT INTO type_of_change_ref (type_of_change) VALUES
  ('Изменение типа профиля');

-- TABLE: spec_to_user: id (serial), id_spec (INTEGER), id_user (INTEGER),
INSERT INTO spec_to_user (id_spec, uuid_user) VALUES
  (2, '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b'),
  (3, '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b');

-- TABLE: spec_ref: id (serial), spec (VARCHAR(100)), id_spec_parent (INTEGER),
INSERT INTO spec_ref (spec, id_spec_parent) VALUES
  ('Root_catalog', 1);
