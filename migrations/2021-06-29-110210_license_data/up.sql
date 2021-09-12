-- Your SQL goes here
INSERT INTO license_ref (name, keyword, publication_at) VALUES
  ('MIT License', 'mit', now());

INSERT INTO license_permission_ref (id) VALUES
  (1),
  (2),
  (3),
  (4);

INSERT INTO license_permission_translate_list (permission_license_id, lang_id, permission) VALUES
  (1, 1, 'Commercial use'),
  (2, 1, 'Modification'),
  (3, 1, 'Distribution'),
  (4, 1, 'Private use');

INSERT INTO permission_to_license (permission_id, license_id) VALUES
  (1,1),
  (2,1),
  (3,1),
  (4,1);

INSERT INTO license_limitation_ref (id) VALUES
  (1),
  (2);

INSERT INTO license_limitation_translate_list (limitation_license_id, lang_id, limitation) VALUES
  (1, 1, 'Liability'),
  (2, 1, 'Warranty');

INSERT INTO limitation_to_license (limitation_id, license_id) VALUES
  (1,1),
  (2,1);

INSERT INTO license_condition_ref (id) VALUES
  (1);

INSERT INTO license_condition_translate_list (condition_license_id, lang_id, condition) VALUES
  (1, 1, 'License and copyright notice');

INSERT INTO condition_to_license (condition_id, license_id) VALUES
  (1,1);
