-- Your SQL goes here
INSERT INTO license_ref (name, keyword, publication_at) VALUES
  ('MIT License', 'mit', now());

INSERT INTO license_permission_ref (id) VALUES
  (1),
  (2),
  (3),
  (4);

INSERT INTO license_permission_translate_list (id_license_permission, id_lang, permission) VALUES
  (1, 1, 'Commercial use'),
  (2, 1, 'Modification'),
  (3, 1, 'Distribution'),
  (4, 1, 'Private use');

INSERT INTO permission_to_license (id_permission, id_license) VALUES
  (1,1),
  (2,1),
  (3,1),
  (4,1);

INSERT INTO license_limitation_ref (id) VALUES
  (1),
  (2);

INSERT INTO license_limitation_translate_list (id_license_limitation, id_lang, limitation) VALUES
  (1, 1, 'Liability'),
  (2, 1, 'Warranty');

INSERT INTO limitation_to_license (id_limitation, id_license) VALUES
  (1,1),
  (2,1);

INSERT INTO license_condition_ref (id) VALUES
  (1);

INSERT INTO license_condition_translate_list (id_license_condition, id_lang, condition) VALUES
  (1, 1, 'License and copyright notice');

INSERT INTO condition_to_license (id_condition, id_license) VALUES
  (1,1);
