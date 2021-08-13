-- Your SQL goes here
/* Уведомление */
CREATE TABLE notification_ref (
  id SERIAL UNIQUE, /* id уведомления */
  notification VARCHAR(2000) NOT NULL, /* наименование уведомления */
  id_degree_importance INTEGER NOT NULL, /* степень важности уведомления */
  generated_at TIMESTAMP NOT NULL DEFAULT NOW(), /* дата публикации */
  is_read BOOLEAN NOT NULL DEFAULT 'f', /* статус прочтения */
  CONSTRAINT notification_ref_pk PRIMARY KEY (id)
);

CREATE TABLE degree_importance_ref (
  id SERIAL UNIQUE, /* id уведомления */
  id_lang INTEGER NOT NULL, /* идентификатор языка перевода */
  degree VARCHAR(50) NOT NULL, /* наименование степени важности в переводе */
  UNIQUE(id_lang, degree),
  CONSTRAINT degree_importance_ref_pk PRIMARY KEY (id)
);
