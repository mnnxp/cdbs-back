INSERT INTO component_ref (uuid, parent_component_uuid, name, description,
  user_uuid, type_access_id, component_type_id, actual_status_id, is_standard,
  is_delete, created_at, updated_at) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 'a5953fd9-7393-4f1e-a899-06b5e159dbf1', 'Reduced shank bolts and screws with coarse thread', 'Continuously improve the product quality and applicability...', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 1, 1, 1, 't', 'f', now(), now()),
  ('e925833e-f8d3-4ecb-bd67-5aa450f9f0ad', 'a5953fd9-7393-4f1e-a899-06b5e159dbf1', 'Knobs 123 Inch', 'Plastic...', '68b8281a-d19c-4d4b-88eb-6fd4a2afde1b', 1, 1, 1, 'f', 'f', now(), now());

INSERT INTO component_history_list (component_uuid, type_of_change_id, old_data, changed_at) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 1, 'Комментарий к изменению', now());

INSERT INTO component_type_ref (id) VALUES
  (1),
  (2);

INSERT INTO component_type_translate_list (component_type_id, lang_id, component_type) VALUES
  (1, 1, 'base'),
  (2, 1, 'own'),
  (1, 2, 'базовый'),
  (2, 2, 'собственный');

INSERT INTO keyword_to_component (component_uuid, keyword_id) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 1),
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 2),
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 3);

INSERT INTO param_to_component (component_uuid, param_id, value) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 9, '1'),
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 10, '0.5'),
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 11, '1.25'),
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 12, '2.51');

INSERT INTO supplier_to_component (component_uuid, company_uuid, description) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 'e97ea679-4560-4a9b-ad8b-80d2d191235e', 'Комментарий поставщика');

INSERT INTO discussion_component_ref (parent_discussion_id, component_uuid,
  author_uuid, message_content, is_delete, created_at, updated_at) VALUES
  (1, 'a5953fd9-7393-4f1e-a899-06b5e159dbf1', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 'this message', 'f', now(), now());

INSERT INTO component_modification_list (uuid, component_uuid, parent_modification_uuid,
  modification_name, description, actual_status_id,
  is_delete, created_at, updated_at) VALUES
  ('aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 'a5953fd9-7393-4f1e-a899-06b5e159dbf1', 'aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 'Head style C - Type H', 'main modification', 1, 'f', now(), now()),
  ('4ce05051-2d7c-456d-927c-bf84a6b08748', 'a5953fd9-7393-4f1e-a899-06b5e159dbf1', 'aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 'Head VS - Type XW', 'second modification', 1, 'f', now(), now());

INSERT INTO param_to_modification (modification_uuid, param_id, value) VALUES
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

INSERT INTO file_to_component (component_uuid, file_uuid) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 'bc1c2151-86d0-4656-9c9d-d016dd584297');

INSERT INTO file_to_modification (modification_uuid, file_uuid) VALUES
  ('aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 'bc1c2151-86d0-4656-9c9d-d016dd584297');

INSERT INTO fileset_for_program (uuid, modification_uuid, program_id) VALUES
  ('5de37b5d-75af-4323-b5b4-2cf1e849baa2', 'aba22d59-4f6c-44a4-9a37-2d38f0e577a8', 1);

INSERT INTO file_of_modification_set (fileset_uuid, file_uuid) VALUES
  ('5de37b5d-75af-4323-b5b4-2cf1e849baa2', '3706d1a1-80ae-4367-be39-af7091373811');

INSERT INTO spec_to_component (spec_id, component_uuid) VALUES
  (487, 'a5953fd9-7393-4f1e-a899-06b5e159dbf1');

INSERT INTO license_to_component (component_uuid, license_id) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', 1);

INSERT INTO standard_to_component (standard_uuid, component_uuid) VALUES
  ('303ec2aa-2066-42e3-93fb-de4fb9344bcb', 'a5953fd9-7393-4f1e-a899-06b5e159dbf1');
