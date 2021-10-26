-- Your SQL goes here
INSERT INTO program_ref (name) VALUES
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

INSERT INTO extension_ref (extension, program_id) VALUES
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

INSERT INTO file_ref (uuid, parent_file_uuid, hash, user_uuid, filename, content_type, id_ext, filesize , path_file, created_at, updated_at) VALUES
  ('bc1c2151-86d0-4656-9c9d-d016dd584297', 'bc1c2151-86d0-4656-9c9d-d016dd584297', E'\\x', '68b8281a-d19c-4d4b-88eb-6fd4a2afde1b', 'default_favicon.png', 'image/png', 1, 166294, '7471c721a4d84a6b87b8af42e4c4494c/ce2301af77854d96b5ccc1182e24b751', '2021-10-26 22:16:49.008719', '2021-10-26 22:32:58.157966'),
  ('ae496786-33f9-4727-a0fb-9c6702a3f30a', 'bc1c2151-86d0-4656-9c9d-d016dd584297', E'\\xe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855', '68b8281a-d19c-4d4b-88eb-6fd4a2afde1b', 'file_child_one.pdd', 'text/plain', 1, 136, '/sholder/file/f1c5a362-55f9-4edb-ad90-a2ea64d586df', now(), now()),
  ('a7ea73ef-1033-4cc7-b65b-93c1e9464360', 'bc1c2151-86d0-4656-9c9d-d016dd584297', E'\\xe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855', '68b8281a-d19c-4d4b-88eb-6fd4a2afde1b', 'file_child_two.pdd', 'text/plain', 1, 365, '/sholder/file/f1c5a362-55f9-4edb-ad90-a2ea64d586df', now(), now()),
  ('9a227a5d-c54e-496a-a1ef-c5d49d8bd0a2', 'bc1c2151-86d0-4656-9c9d-d016dd584297', E'\\xe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855', '68b8281a-d19c-4d4b-88eb-6fd4a2afde1b', 'file_child_three.pdd', 'text/plain', 1, 256, '/sholder/file/f1c5a362-55f9-4edb-ad90-a2ea64d586df', now(), now()),
  ('3706d1a1-80ae-4367-be39-af7091373811', 'bc1c2151-86d0-4656-9c9d-d016dd584297', E'\\xe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855', '68b8281a-d19c-4d4b-88eb-6fd4a2afde1b', 'file_child_four.pdd', 'text/plain', 1, 175, '/sholder/file/f1c5a362-55f9-4edb-ad90-a2ea64d586df', now(), now());

INSERT INTO actual_status_ref (id) VALUES
  (1),
  (2),
  (3),
  (4),
  (5);

INSERT INTO actual_status_translate_list (actual_status_id, lang_id, name) VALUES
  (1, 1, 'Developed'),
  (2, 1, 'Tested'),
  (3, 1, 'Planned'),
  (4, 1, 'Launched'),
  (5, 1, 'Discontinued'),
  (1, 2, 'Разработка'),
  (2, 2, 'Тестирование'),
  (3, 2, 'Подготовка'),
  (4, 2, 'Производство'),
  (5, 2, 'Снято с производства');

INSERT INTO type_access_ref (id) VALUES
  (1),
  (2),
  (3);

INSERT INTO type_access_translate_list (type_access_id, lang_id, name) VALUES
  (1, 1, 'Private'),
  (2, 1, 'Protected'),
  (3, 1, 'Public'),
  (1, 2, 'Закрыт'),
  (2, 2, 'Частичный'),
  (3, 2, 'Открыт');

INSERT INTO param_ref (id) VALUES
  (1),
  (2),
  (3),
  (4),
  (5),
  (6),
  (7),
  (8),
  (9),
  (10),
  (11),
  (12);

INSERT INTO param_translate_list (param_id, lang_id, paramname) VALUES
  (1, 1, 'Index'),
  (2, 1, 'Selector'),
  (3, 1, 'Part Number'),
  (4, 1, 'Description'),
  (5, 1, 'E-Link'),
  (6, 1, 'Material'),
  (7, 1, 'Finish'),
  (8, 1, 'Article Number'),
  (9, 1, 'Nominal Ø'),
  (10, 1, 'Pitch (mm)'),
  (11, 1, 'Length (mm)'),
  (12, 1, 'Head height (mm)'),
  (1, 2, 'Индекс'),
  (2, 2, 'Селектор'),
  (3, 2, 'Номер части'),
  (4, 2, 'Описание'),
  (5, 2, 'Электронная ссылка'),
  (6, 2, 'Материал'),
  (7, 2, 'Заканчивать'),
  (8, 2, 'Номер статьи'),
  (9, 2, 'Номинальный Ø'),
  (10, 2, 'Шаг (мм)'),
  (11, 2, 'Длина (мм)'),
  (12, 2, 'Высота головки (мм)');

INSERT INTO language_ref (lang, langshort) VALUES
  ('English', 'EN'),
  ('Русский', 'RU');

INSERT INTO region_ref (id) VALUES
  (1),
  (2),
  (3),
  (4),
  (5),
  (6),
  (7),
  (8),
  (9),
  (10),
  (11),
  (12),
  (13);

INSERT INTO region_translate_list (region_id, lang_id, region) VALUES
  (1, 1, 'The Republic of Adygea (Adygea)'),
  (2, 1, 'Republic of Bashkortostan'),
  (3, 1, 'The Republic of Buryatia'),
  (4, 1, 'Altai Republic'),
  (5, 1, 'Republic of Crimea'),
  (6, 1, 'Chuy valley'),
  (7, 1, 'IssykKul region'),
  (8, 1, 'Naryn region'),
  (9, 1, 'Talas region'),
  (10, 1, 'JalalAdab region'),
  (11, 1, 'Osh region'),
  (12, 1, 'Batken region'),
  (13, 1, 'Other'),
  (1, 2, 'Республика Адыгея (Адыгея)'),
  (2, 2, 'Республика Башкортостан'),
  (3, 2, 'Республика Бурятия'),
  (4, 2, 'Республика Алтай'),
  (5, 2, 'Республика Крым'),
  (6, 2, 'Чуйская долина'),
  (7, 2, 'ИссыкКульская область'),
  (8, 2, 'Нарынская область'),
  (9, 2, 'Таласская область'),
  (10, 2, 'ДжалалАдабская область'),
  (11, 2, 'Ошская область'),
  (12, 2, 'Баткенская область'),
  (13, 2, 'Другой');

INSERT INTO type_of_change_ref (id) VALUES
  (1);

INSERT INTO type_of_change_translate_list (type_of_change_id, lang_id, type_of_change) VALUES
  (1, 1, 'data update'),
  (1, 2, 'обновление данных');

INSERT INTO spec_ref (id, parent_spec_id) VALUES
  (1, 1);

INSERT INTO spec_translate_list (spec_id, lang_id, spec) VALUES
  (1, 1, 'ROOT'),
  (1, 2, 'КОРЕНЬ');


INSERT INTO keyword_ref (keyword) VALUES
  ('tools'),
  ('bolt'),
  ('screw'),
  ('automotive'),
  ('autodesk'),
  ('fusion'),
  ('tech'),
  ('iso');

INSERT INTO storage_access_ref (application_key_id, application_key, expiration_at, bucket, region, endpoint) VALUES
  (
    'SCWY16N9VEZGBRGWYFHC',
    '92bfa452-c53f-4afa-b6a4-82e621b34053',
    now()+interval '1000' day,
    'cdbs-test',
    'pl-waw',
    'https://s3.pl-waw.scw.cloud'
  );
