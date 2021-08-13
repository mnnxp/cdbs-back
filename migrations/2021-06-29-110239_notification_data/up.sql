-- Your SQL goes here
INSERT INTO notification_ref (notification, id_degree_importance,
  generated_at, is_read) VALUES
  ('warning, this is 1th notification', 5, now(), 'f');

INSERT INTO degree_importance_ref (id_lang, degree) VALUES
  (1, 'critical'),
  (1, 'error'),
  (1, 'warning'),
  (1, 'success'),
  (1, 'info');
