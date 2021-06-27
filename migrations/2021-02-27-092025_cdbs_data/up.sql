INSERT INTO actual_status_ref (name) VALUES
    ('актуальный'),
    ('архивный'),
    ('снят с производства');

-- TABLE: component_keyword_ref: keyword (VARCHAR(10))
INSERT INTO component_keyword_ref (keyword) VALUES
    ('tools'),
    ('bolt'),
    ('screw'),
    ('automotive'),
    ('autodesk'),
    ('fusion'),
    ('tech');

INSERT INTO standard_status_ref (name) VALUES
    ('Published'),
    ('Development'),
    ('Withdrawn'),
    ('Deleted');

INSERT INTO standard_ref (
  uuid, uuid_standard_parent, classifier, name, description, specified_tolerance,
  technical_committee, publication_at, uuid_image_file, uuid_user, uuid_company,
  id_type_access,  id_standard_status, id_region, is_delete, created_at, updated_at) VALUES
  ('303ec2aa-2066-42e3-93fb-de4fb9344bcb', '303ec2aa-2066-42e3-93fb-de4fb9344bcb', 'ISO 10511', 'Prevailing torque type hexagon thin nuts (with non-metallic insert)', 'ISO 10511:2012 specifies the characteristics of prevailing torque type hexagon thin nuts (with non-metallic insert) with thread from M3 up to and including M36, in product grade A for threads up to and including M16 and product grade B for threads above M16, and with property classes 04 and 05.', 'Class I', 'ISO/TC 2/SC 12 Fasteners with metric internal thread', '2012-12-01T00:00:00', '3706d1a1-80ae-4367-be39-af7091373811', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', '2cd385e1-8f7e-4908-8235-dfe42938b46d', 1, 1, 13, 'f', now(), now());

INSERT INTO component_ref (uuid, uuid_component_parent, name, description,
  uuid_user, id_type_access, id_component_type, id_actual_status, is_standard,
  is_delete, created_at, updated_at) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 'a5953fd9-7393-4f1e-a899-06b5e159dbf1', 'Reduced shank bolts and screws with coarse thread', 'Continuously improve the product quality and applicability...', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 1, 1, 1, 1, 'f', now(), now()),
  ('e925833e-f8d3-4ecb-bd67-5aa450f9f0ad', 'a5953fd9-7393-4f1e-a899-06b5e159dbf1', 'Knobs 123 Inch', 'Plastic...', '68b8281a-d19c-4d4b-88eb-6fd4a2afde1b', 1, 1, 1, 0, 'f', now(), now());

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

INSERT INTO set_file_to_programm (uuid_modification, uuid_file, id_programm) VALUES
  ('aba22d59-4f6c-44a4-9a37-2d38f0e577a8', '3706d1a1-80ae-4367-be39-af7091373811', 1);

INSERT INTO component_modification_list (uuid, uuid_component, uuid_modification_parent,
  modification_name, description, id_actual_status,
  is_delete, created_at, updated_at) VALUES
  ('aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 'a5953fd9-7393-4f1e-a899-06b5e159dbf1', 'aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 'Head style C - Type H', 'main modification', 1, 'f', now(), now());

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

INSERT INTO supplier_to_component (uuid_component, uuid_company, description) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 'e97ea679-4560-4a9b-ad8b-80d2d191235e', 'Комментарий поставщика');

INSERT INTO discussion_component_ref (id_discussion_parent, uuid_component,
  uuid_author, message_content, is_delete, created_at, updated_at) VALUES
  (1, 'a5953fd9-7393-4f1e-a899-06b5e159dbf1', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 'this message', 'f', now(), now());

INSERT INTO discussion_company_ref (id_discussion_parent, uuid_company,
  uuid_author, message_content, is_delete, created_at, updated_at) VALUES
  (1, '2cd385e1-8f7e-4908-8235-dfe42938b46d', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 'this message', 'f', now(), now());

INSERT INTO extension_ref (extension, id_programm) VALUES
    ('3dm', 1),
    ('3ds', 1),
    ('a2c', 2),
    ('apt', 3),
    ('asm', 4),
    ('asm', 5),
    ('asm', 6),
    ('asp', 7),
    ('asp', 8),
    ('ccd', 9),
    ('cdd', 10),
    ('cdw', 11),
    ('cdw', 12),
    ('cpp', 13),
    ('cr2', 1),
    ('dgn', 1),
    ('dtd', 1),
    ('dwg', 1),
    ('dwg', 1),
    ('dxf', 1),
    ('ics', 1),
    ('igs', 1),
    ('igs', 1),
    ('iso', 1),
    ('iso', 1),
    ('ma', 1),
    ('max', 1),
    ('max', 1),
    ('mb', 1),
    ('mod', 1),
    ('mod', 1),
    ('mod', 1),
    ('mod', 1),
    ('mts', 1),
    ('obj', 1),
    ('part', 1),
    ('pkg', 1),
    ('prj', 1),
    ('pwi', 1),
    ('rnd', 1),
    ('sldasm', 1),
    ('sldprt', 1),
    ('stl', 1),
    ('vcd', 1),
    ('vob', 1),
    ('wm', 1),
    ('wm2d', 1);

INSERT INTO file_ref (uuid, uuid_file_parent, hash, uuid_user, filename,
  id_ext, filesize , path_file, created_at, updated_at) VALUES
    ('bc1c2151-86d0-4656-9c9d-d016dd584297', 'bc1c2151-86d0-4656-9c9d-d016dd584297', E'\\000', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 'filename', 1, 0, 'path/file/file.txt', now(), now()),
    ('3706d1a1-80ae-4367-be39-af7091373811', 'bc1c2151-86d0-4656-9c9d-d016dd584297', E'\\xe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855', '68b8281a-d19c-4d4b-88eb-6fd4a2afde1b', 'file_child_two.pdd', 1, 136, '/sholder/file/f1c5a362-55f9-4edb-ad90-a2ea64d586df', now(), now());

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

INSERT INTO programm_ref (name) VALUES
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
    ('Root_catalog', 1);

INSERT INTO component_access_to_company (uuid_component, uuid_company, id_type_access,
  is_enabled, is_delete, created_at, updated_at) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', '2cd385e1-8f7e-4908-8235-dfe42938b46d', 1, 't', 'f', now(), now());

INSERT INTO component_access_to_user (uuid_component, uuid_user, id_type_access,
  is_enabled, is_delete, created_at, updated_at) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 1, 't', 'f', now(), now());

INSERT INTO standard_access_to_company (uuid_standard, uuid_company, id_type_access,
  is_enabled, is_delete, created_at, updated_at) VALUES
  ('303ec2aa-2066-42e3-93fb-de4fb9344bcb', '2cd385e1-8f7e-4908-8235-dfe42938b46d', 1, 't', 'f', now(), now());

INSERT INTO standard_access_to_user (uuid_standard, uuid_user, id_type_access,
  is_enabled, is_delete, created_at, updated_at) VALUES
  ('303ec2aa-2066-42e3-93fb-de4fb9344bcb', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 1, 't', 'f', now(), now());

INSERT INTO component_fav (uuid_component, uuid_user) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b');

INSERT INTO company_fav (uuid_company, uuid_user) VALUES
  ('2cd385e1-8f7e-4908-8235-dfe42938b46d', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b');

INSERT INTO standard_fav (uuid_standard, uuid_user) VALUES
  ('303ec2aa-2066-42e3-93fb-de4fb9344bcb', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b');

INSERT INTO user_fav (uuid_user_favorite, uuid_user_follower) VALUES
  ('68b8281a-d19c-4d4b-88eb-6fd4a2afde1b', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b');

INSERT INTO type_access_ref (name) VALUES
  ('Полный'),
  ('Частичный'),
  ('Закрыт');

INSERT INTO user_ref (uuid, email, psw_hash, psw_salt,
  firstname, lastname, secondname, username, phone, description, address,
  position, time_zone, uuid_image_file, id_region, id_programm,
  is_email_verified, is_enabled, is_delete, created_at, updated_at) VALUES
  ('31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 'email@email.ru', E'\\xc3747b782c0b5c13cb1257a951d5120cd6a958f3516ba5d40c1db1c1eae99b15', '8%8lDv&TB!295%cNWDmghT5lNDSxTUxUgRY6xNw^hACP!DDDK8IKNLP)0Hr(C7m55BQDr&L%V0F^~3O&J~QPQDfJ$&uDjwUwPShyK0B4yDhXcBe^cPoV@%^gax^%z)92', 'Johm', 'Ivanov', 'Rucovich', 'usernameeee', '+79991234567', 'description for this user', 'Moscow', 'manufacturer', 3, 'bc1c2151-86d0-4656-9c9d-d016dd584297', 1, 1, 'f', 't', 'f', now(), now()),
  ('68b8281a-d19c-4d4b-88eb-6fd4a2afde1b', 'bname@somain.com', E'\\x085f06287c840b5c23578912d5e0cc2e4baf53865e9170a05ed084c1292ab7f4', 'pKFpenRqOFyutR#OAkxb%!bi%mV5q(GPKgHmwQ*bWrcuJHC3k8raBNzUnw7r%^oFKzBf%McZlVBI#O@U1@JApg@rVHEuzlybCWx&BXjrI(41x)8kR9rjURVG9lqr0EIM', 'Vans', 'Bpero', 'Nado', 'albane', 'none', 'none', 'noneadress', 'engineer', 2, 'bc1c2151-86d0-4656-9c9d-d016dd584297', 4, 1, 'f', 't', 'f', now(), now()),
  ('e97ea679-4560-4a9b-ad8b-80d2d1912602', 'testemail@testemail.ru', E'\\xc0a6617d8971cac49489078028b6ea4bd6f7929f6e542a1de5cc4c2a0f6fdfce', 'a13%A1r9kCmDHieCl(^Yt$~traAIlnTM(0#vHvjE&tZ@9Cm2OJKCKENu6&a2pTrd*Z%qQyiYXX@fG2j7XeBLx4FYY9tkSK*B^yV)0s$sQ!y)qBL#!RDxcxZLPEKD5@lg', 'testfirstname', 'testlastname', 'testsecondname', 'testingname', '+1234567890', 'testdescription', 'testaddress', 'testposition', 2, 'bc1c2151-86d0-4656-9c9d-d016dd584297', 5, 1, 'f', 't', 'f', now(), now()),
  ('c3f5f69c-bb54-45d9-bfa7-1d28cc1afa5a', 'bname@somain.com', E'\\xfdeba1b9304208da90c90ddb6fe0ee49787b085636b26f156d6240869bdb3665', 'ThmlF#HCNEX6%##AFGH(%0Tdo0w$5kh(WA9%@KwHe3mrVlhMIj~NxeiiJyh~Ty1t(J3F#lGDWFFJRZnV1&WFMA%Rl5~8qvfW)5WjL&qm%jSuFx1Uslth^a$64YkJjN)q', 'Vans', 'Bpero', 'Nado', 'testusertext', 'none', 'none', 'noneadress', 'noneposition', 6, 'bc1c2151-86d0-4656-9c9d-d016dd584297', 4, 1, 'f', 't', 'f', now(), now());

INSERT INTO user_tokens_ref (uuid_user, token, date_start, date_end, is_enabled) VALUES
  ('31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 'GNLw1GKzykA926Rhdcpy1c6lugZXTd5y', now(), now(), 'f');

INSERT INTO company_ref (uuid, orgname, shortname, inn, phone,
  email, description, address, site_url, time_zone, uuid_user,
  uuid_image_file, id_region, id_type_org, is_supplier, is_email_verified,
  is_enabled, is_delete, created_at, updated_at) VALUES
    ('2cd385e1-8f7e-4908-8235-dfe42938b46d', 'romashka', 'rom-ka', '12345678910', '+79991234567', 'email@email.ru', 'description for this company', 'Moscow', 'https://cadbase.ru', 3, '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 'bc1c2151-86d0-4656-9c9d-d016dd584297', 1, 1, 't', 't', 't', 'f', now(), now()),
    ('e97ea679-4560-4a9b-ad8b-80d2d191235e', 'testorgname', 'testshortname', 'testinn', '+1234567890', 'testemail@testemail.ru', 'testdescription', 'testaddress', 'testsiteUrl', 2, '68b8281a-d19c-4d4b-88eb-6fd4a2afde1b', 'bc1c2151-86d0-4656-9c9d-d016dd584297', 13, 5, 'f', 'f', 't', 'f', now(), now());

INSERT INTO company_represent_ref (uuid, uuid_company, id_region, id_representation_type,
  name, address, phone) VALUES
  ('22a08149-b63a-4f65-a1cd-4a28f85567ec', 'e97ea679-4560-4a9b-ad8b-80d2d191235e', 1, 1, 'Местный офис', 'г. Москва', '+79991234567'),
  ('297e44b3-d36c-4ab5-be16-e2a955aabf13', 'e97ea679-4560-4a9b-ad8b-80d2d191235e', 2, 1, 'additional office', 'Batkov District, Minsk', '+375548418789'),
  ('96df6359-a31e-40ad-aa06-9355abc2cc58', 'e97ea679-4560-4a9b-ad8b-80d2d191235e', 3, 1, 'additional office', 'None str, Kiev', '+380874487556');

INSERT INTO type_company_ref (name, shortname) VALUES
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

INSERT INTO company_member_role (uuid_company, uuid_user, id_role,
  is_enabled, created_at, updated_at) VALUES
  ('2cd385e1-8f7e-4908-8235-dfe42938b46d', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 1, 't', now(), now()),
  ('e97ea679-4560-4a9b-ad8b-80d2d191235e', '68b8281a-d19c-4d4b-88eb-6fd4a2afde1b', 1, 't', now(), now());

INSERT INTO role_member_ref (name) VALUES
  ('Стажер'),
  ('Инженер');

INSERT INTO role_access (id_role, id_type_access) VALUES
  (1, 2),
  (1, 1);

INSERT INTO user_history_list (uuid_user, id_type_of_change, old_data, changed_at) VALUES
  ('31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 1, 'Комментарий к изменению', now());

INSERT INTO company_history_list (uuid_company, id_type_of_change, old_data, changed_at) VALUES
  ('2cd385e1-8f7e-4908-8235-dfe42938b46d', 1, 'Комментарий к изменению', now());

INSERT INTO component_history_list (uuid_component, id_type_of_change, old_data, changed_at) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 1, 'Комментарий к изменению', now());

INSERT INTO standard_history_list (uuid_standard, id_type_of_change, old_data, changed_at) VALUES
  ('303ec2aa-2066-42e3-93fb-de4fb9344bcb', 1, 'Комментарий к изменению', now());

-- TABLE: type_of_change_ref: id (serial), type_of_change (VARCHAR(100))
INSERT INTO type_of_change_ref (type_of_change) VALUES
  ('Изменение типа профиля');

INSERT INTO spec_to_company (id_spec, uuid_company) VALUES
  (2, '2cd385e1-8f7e-4908-8235-dfe42938b46d'),
  (3, 'e97ea679-4560-4a9b-ad8b-80d2d191235e');
