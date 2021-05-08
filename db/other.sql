-- TABLE: user_ref:
-- uuid (UUID), email (VARCHAR(100)), email_verified (INTEGER),
-- psw_hash (BYTEA), psw_salt (VARCHAR(255)), id_type_user (INTEGER),
-- firstname (VARCHAR(100)), lastname (VARCHAR(100)), secondname (VARCHAR(100)),
-- username (VARCHAR(100)), orgname (VARCHAR(255)), shortname (VARCHAR(255)),
-- inn (VARCHAR(30)), phone (VARCHAR(100)), id_name_cad (INTEGER),
-- comment (VARCHAR(2000)), address (VARCHAR(512)), time_zone (INTEGER),
-- position (VARCHAR(255)), site_url (VARCHAR(255)), id_file_info_icon (INTEGER),
-- id_region (INTEGER), created_at (TIMESTAMP),
INSERT INTO user_ref (uuid, email, email_verified, psw_hash, psw_salt,
  id_type_user, firstname, lastname, secondname, username, orgname, shortname,
  inn, phone, id_name_cad, comment, address, time_zone, position, site_url,
  id_file_info_icon, id_region, created_at) VALUES
    ('31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 'email@email.ru', 0, E'\\000',
      '000', 0, 'Johm', 'Ivanov', 'Rucovich', 'usernameeee', 'romashka', 'rom-ka',
      '12345678910', '+7999123456', 0, 'comment for this user', 'Moscow',
      3, 'engineer', 'https://cadbase.ru', 0, 0, now());

insert or update on table "user_ref" violates foreign key constraint "user_ref_fk0

  actual_status_ref
  component_fav_ref
  component_keyword_ref
  component_ref
  component_type_ref
  discussion_ref
  extension_ref
  file_ref
  language_ref
  name_cad_ref
  param_ref
  region_ref
  representation_type_ref
  spec_ref
  type_access_ref
  type_of_change_ref
  type_user_ref
  user_ref
  user_represent_ref
  user_tokens_ref
component_access_to_user
component_modification_list
component_to_keyword
component_to_user
file_to_component
file_to_modification
param_to_component
param_to_modification
param_translate_list
spec_to_component
spec_to_user
spec_translate_list
user_history_list


.3dm Rhino 3D
.3ds Изображение Autodesk 3D Studio
.a2c Alice
.apt Alphacam
.asm Solid Edge
.asm Pro/ENGINEER
.asm Alphacam
.asp Alphacam
.asp EROSION 3D
.ccd Vector Cad-Cam
.cdd CADAM
.cdw Компас
.cdw CADKEY
.cpp Maya OpenGL 3D
.cr2 Poser
.dgn MicroStation
.dtd Design Tools
.dwg AutoCAD
.dwg BravoDRAFT
.dxf AutoCAD
.ics IronCAD
.igs IGES
.igs Indigo Renderer
.iso Arbortext IsoDraw
.iso Cimagraphi
.ma Maya
.max 3D Studio Max
.max OrCAD
.mb Maya
.mod ArchiCAD
.mod CATIA V4
.mod CADdy++ mechanical 2D/3D Очень редко используется
.mod Femap
.mts MetaCreations Streaming 3D Viewer
.obj 3D объектный файл
.part Pro/Engineer
.pkg CoCreate OneSpace
.prj ArcView Shapefile
.pwi PowerInspect
.rnd AutoCAD Autoshade
.sldasm 3D-объекта SolidWorks
.sldprt 3D-объект SolidWorks
.stl Файл стереолитографии
.vcd VisualCADD
.vob Файл объекта Vue d'Esprit
.wm Файл Working Model
.wm2d Файл данных 2D Working Model
