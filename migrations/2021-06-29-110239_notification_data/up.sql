-- Your SQL goes here
INSERT INTO notification_ref (notification, degree_importance_id,
  created_at, is_read) VALUES
  ('warning, this is 1th notification', 5, now(), 'f');

INSERT INTO degree_importance_ref (id) VALUES
  (1),
  (2),
  (3),
  (4),
  (5);

INSERT INTO degree_importance_translate_list (degree_importance_id, lang_id, degree) VALUES
  (1, 1, 'critical'),
  (2, 1, 'error'),
  (3, 1, 'warning'),
  (4, 1, 'success'),
  (5, 1, 'info');
