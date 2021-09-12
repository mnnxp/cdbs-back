-- Your SQL goes here
/* Уведомление */
CREATE TABLE notification_ref (
  id SERIAL, /* id уведомления */
  notification VARCHAR(2000) NOT NULL, /* наименование уведомления */
  degree_importance_id INTEGER NOT NULL, /* степень важности уведомления */
  generated_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата публикации */
  is_read BOOLEAN NOT NULL DEFAULT 'f', /* статус прочтения */
  CONSTRAINT notification_ref_pk PRIMARY KEY (id)
);

CREATE TABLE degree_importance_ref (
  id SERIAL, /* id уведомления */
  CONSTRAINT degree_importance_ref_pk PRIMARY KEY (id)
);

CREATE TABLE degree_importance_translate_list (
  degree_importance_id INTEGER NOT NULL, /* id уведомления */
  lang_id INTEGER NOT NULL, /* идентификатор языка перевода */
  degree VARCHAR(50) NOT NULL, /* наименование степени важности в переводе */
  UNIQUE(lang_id, degree),
  CONSTRAINT degree_importance_translate_list_pk PRIMARY KEY (degree_importance_id, lang_id)
);
