INSERT INTO component_ref (uuid, uuid_component_parent, name, description,
  uuid_user, id_type_access, id_component_type, id_actual_status, is_standard,
  is_delete, created_at, updated_at) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 'a5953fd9-7393-4f1e-a899-06b5e159dbf1', 'Reduced shank bolts and screws with coarse thread', 'Continuously improve the product quality and applicability...', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 1, 1, 1, 't', 'f', now(), now()),
  ('e925833e-f8d3-4ecb-bd67-5aa450f9f0ad', 'a5953fd9-7393-4f1e-a899-06b5e159dbf1', 'Knobs 123 Inch', 'Plastic...', '68b8281a-d19c-4d4b-88eb-6fd4a2afde1b', 1, 1, 1, 'f', 'f', now(), now());

INSERT INTO component_history_list (uuid_component, id_type_of_change, old_data, changed_at) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 1, 'Комментарий к изменению', now());

INSERT INTO component_type_ref (id) VALUES
  (1),
  (2);

INSERT INTO component_type_translate_list (id_component_type, id_lang, component_type) VALUES
  (1, 1, 'base'),
  (2, 1, 'own'),
  (1, 2, 'базовый'),
  (2, 2, 'собственный');

INSERT INTO component_to_keyword (uuid_component, id_keyword) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 1),
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 2),
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 3);

INSERT INTO param_to_component (uuid_component, id_param, value) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 9, '1'),
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 10, '0.5'),
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 11, '1.25'),
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 12, '2.51');

INSERT INTO supplier_to_component (uuid_component, uuid_company, description) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 'e97ea679-4560-4a9b-ad8b-80d2d191235e', 'Комментарий поставщика');

INSERT INTO discussion_component_ref (id_discussion_parent, uuid_component,
  uuid_author, message_content, is_delete, created_at, updated_at) VALUES
  (1, 'a5953fd9-7393-4f1e-a899-06b5e159dbf1', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 'this message', 'f', now(), now());

INSERT INTO component_modification_list (uuid, uuid_component, uuid_modification_parent,
  modification_name, description, id_actual_status,
  is_delete, created_at, updated_at) VALUES
  ('aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 'a5953fd9-7393-4f1e-a899-06b5e159dbf1', 'aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 'Head style C - Type H', 'main modification', 1, 'f', now(), now()),
  ('4ce05051-2d7c-456d-927c-bf84a6b08748', 'a5953fd9-7393-4f1e-a899-06b5e159dbf1', 'aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 'Head VS - Type XW', 'second modification', 1, 'f', now(), now());

INSERT INTO param_to_modification (uuid_modification, id_param, value) VALUES
  ('aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 1, '1'),
  ('aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 2, '1'),
  ('aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 3, 'SMS8'),
  ('aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 4, 'SMS8 Self-Drilling and Tapping Screw, #8 Screw, 1/2" Screw'),
  ('aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 5, 'SMS8'),
  ('aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 6, 'Steel'),
  ('aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 7, 'Electrogalvanized'),
  ('aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 8, '187197'),
  ('4ce05051-2d7c-456d-927c-bf84a6b08748', 1, '2'),
  ('4ce05051-2d7c-456d-927c-bf84a6b08748', 2, '5'),
  ('4ce05051-2d7c-456d-927c-bf84a6b08748', 3, 'XV38'),
  ('4ce05051-2d7c-456d-927c-bf84a6b08748', 5, 'XV38'),
  ('4ce05051-2d7c-456d-927c-bf84a6b08748', 6, 'Plastic'),
  ('4ce05051-2d7c-456d-927c-bf84a6b08748', 7, 'Nized'),
  ('4ce05051-2d7c-456d-927c-bf84a6b08748', 8, '187197');

INSERT INTO file_to_component (uuid_component, uuid_file) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 'bc1c2151-86d0-4656-9c9d-d016dd584297');

INSERT INTO file_to_modification (uuid_modification, uuid_file) VALUES
  ('aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 'bc1c2151-86d0-4656-9c9d-d016dd584297');

INSERT INTO set_files_for_program (uuid_modification, id_program) VALUES
  ('aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 1);

INSERT INTO file_to_set_modification (id_set, uuid_file) VALUES
  (1, '3706d1a1-80ae-4367-be39-af7091373811');

INSERT INTO spec_to_component (id_spec, uuid_component) VALUES
  (487, 'a5953fd9-7393-4f1e-a899-06b5e159dbf1');

INSERT INTO license_to_component (uuid_component, id_license) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 1);

INSERT INTO standard_to_component (uuid_standard, uuid_component) VALUES
  ('303ec2aa-2066-42e3-93fb-de4fb9344bcb', 'a5953fd9-7393-4f1e-a899-06b5e159dbf1');
