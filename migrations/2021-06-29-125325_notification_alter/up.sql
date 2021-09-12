-- Your SQL goes here
ALTER TABLE notification_ref ADD CONSTRAINT notification_ref_fk0 FOREIGN KEY (id_degree_importance) REFERENCES degree_importance_ref(id);

ALTER TABLE degree_importance_translate_list ADD CONSTRAINT degree_importance_translate_list_fk0 FOREIGN KEY (id_degree_importance) REFERENCES degree_importance_ref(id) ON DELETE CASCADE;
ALTER TABLE degree_importance_translate_list ADD CONSTRAINT degree_importance_translate_list_fk1 FOREIGN KEY (lang_id) REFERENCES language_ref(id);
