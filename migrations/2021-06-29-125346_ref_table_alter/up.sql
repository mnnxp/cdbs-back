-- Your SQL goes here
ALTER TABLE extension_ref ADD CONSTRAINT extension_ref_fk0 FOREIGN KEY (program_id) REFERENCES program_ref(id);

ALTER TABLE file_ref ADD CONSTRAINT file_ref_fk0 FOREIGN KEY (parent_file_uuid) REFERENCES file_ref(uuid);
ALTER TABLE file_ref ADD CONSTRAINT file_ref_fk1 FOREIGN KEY (user_uuid) REFERENCES user_ref(uuid) ON DELETE CASCADE;
ALTER TABLE file_ref ADD CONSTRAINT file_ref_fk2 FOREIGN KEY (id_ext) REFERENCES extension_ref(id);

ALTER TABLE actual_status_translate_list ADD CONSTRAINT actual_status_translate_list_fk0 FOREIGN KEY (actual_status_id) REFERENCES actual_status_ref(id) ON DELETE CASCADE;
ALTER TABLE actual_status_translate_list ADD CONSTRAINT actual_status_translate_list_fk1 FOREIGN KEY (lang_id) REFERENCES language_ref(id);

ALTER TABLE type_access_translate_list ADD CONSTRAINT type_access_translate_list_fk0 FOREIGN KEY (type_access_id) REFERENCES type_access_ref(id) ON DELETE CASCADE;
ALTER TABLE type_access_translate_list ADD CONSTRAINT type_access_translate_list_fk1 FOREIGN KEY (lang_id) REFERENCES language_ref(id);

ALTER TABLE param_translate_list ADD CONSTRAINT param_translate_list_fk0 FOREIGN KEY (param_id) REFERENCES param_ref(id) ON DELETE CASCADE;
ALTER TABLE param_translate_list ADD CONSTRAINT param_translate_list_fk1 FOREIGN KEY (lang_id) REFERENCES language_ref(id);

ALTER TABLE region_translate_list ADD CONSTRAINT region_translate_list_fk0 FOREIGN KEY (region_id) REFERENCES region_ref(id) ON DELETE CASCADE;
ALTER TABLE region_translate_list ADD CONSTRAINT region_translate_list_fk1 FOREIGN KEY (lang_id) REFERENCES language_ref(id);

ALTER TABLE spec_ref ADD CONSTRAINT spec_ref_fk0 FOREIGN KEY (parent_spec_id) REFERENCES spec_ref(id) ON DELETE CASCADE;

ALTER TABLE spec_translate_list ADD CONSTRAINT spec_translate_list_fk0 FOREIGN KEY (spec_id) REFERENCES spec_ref(id) ON DELETE CASCADE;
ALTER TABLE spec_translate_list ADD CONSTRAINT spec_translate_list_fk1 FOREIGN KEY (lang_id) REFERENCES language_ref(id);

ALTER TABLE type_of_change_translate_list ADD CONSTRAINT type_of_change_translate_list_fk0 FOREIGN KEY (type_of_change_id) REFERENCES type_of_change_ref(id) ON DELETE CASCADE;
ALTER TABLE type_of_change_translate_list ADD CONSTRAINT type_of_change_translate_list_fk1 FOREIGN KEY (lang_id) REFERENCES language_ref(id);
