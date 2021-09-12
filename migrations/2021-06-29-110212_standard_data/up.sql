-- Your SQL goes here

INSERT INTO standard_ref (
  uuid, parent_standard_uuid, classifier, name, description, specified_tolerance,
  technical_committee, publication_at, image_file_uuid, user_uuid, company_uuid,
  type_access_id,  standard_status_id, region_id, is_delete, created_at, updated_at) VALUES
  ('303ec2aa-2066-42e3-93fb-de4fb9344bcb', '303ec2aa-2066-42e3-93fb-de4fb9344bcb', 'ISO 10511', 'Prevailing torque type hexagon thin nuts (with non-metallic insert)', 'ISO 10511:2012 specifies the characteristics of prevailing torque type hexagon thin nuts (with non-metallic insert) with thread from M3 up to and including M36, in product grade A for threads up to and including M16 and product grade B for threads above M16, and with property classes 04 and 05.', 'Class I', 'ISO/TC 2/SC 12 Fasteners with metric internal thread', '2012-12-01T00:00:00', '3706d1a1-80ae-4367-be39-af7091373811', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', '2cd385e1-8f7e-4908-8235-dfe42938b46d', 1, 1, 13, 'f', now(), now());

INSERT INTO standard_history_list (standard_uuid, type_of_change_id, old_data, changed_at) VALUES
  ('303ec2aa-2066-42e3-93fb-de4fb9344bcb', 1, 'Комментарий к изменению', now());

INSERT INTO standard_status_ref (id) VALUES
  (1),
  (2),
  (3),
  (4);

INSERT INTO standard_status_translate_list (standard_status_id, lang_id, name) VALUES
  (1, 1, 'Published'),
  (2, 1, 'Development'),
  (3, 1, 'Withdrawn'),
  (4, 1, 'Deleted');

INSERT INTO file_to_standard (file_uuid, standard_uuid) VALUES
  ('bc1c2151-86d0-4656-9c9d-d016dd584297', '303ec2aa-2066-42e3-93fb-de4fb9344bcb');

INSERT INTO spec_to_standard (spec_id, standard_uuid) VALUES
  (1, '303ec2aa-2066-42e3-93fb-de4fb9344bcb');

INSERT INTO keyword_to_standard (standard_uuid, keyword_id) VALUES
  ('303ec2aa-2066-42e3-93fb-de4fb9344bcb', 8);
