-- Your SQL goes here
INSERT INTO license_ref (name, keyword, description, permission, limitation,
  condition, publication_at) VALUES
  ('MIT License', 'mit', 'A short and simple permissive license with conditions only requiring preservation of copyright and license notices. Licensed works, modifications, and larger works may be distributed under different terms and without source code.',
    1, 1, 1, now());

INSERT INTO license_permission_ref (permission) VALUES
  ('Commercial use'),
  ('Modification'),
  ('Distribution'),
  ('Private use');

INSERT INTO permission_to_license (id_permission, id_license) VALUES
  (1,1),
  (2,1),
  (3,1),
  (4,1);

INSERT INTO license_limitation_ref (limitation) VALUES
  ('Liability'),
  ('Warranty');

INSERT INTO limitation_to_license (id_limitation, id_license) VALUES
  (1,1),
  (2,1);

INSERT INTO license_condition_ref (condition) VALUES
  ('License and copyright notice');

INSERT INTO condition_to_license (id_condition, id_license) VALUES
  (1,1);
