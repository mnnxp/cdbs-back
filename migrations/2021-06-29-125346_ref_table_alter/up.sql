-- Your SQL goes here
ALTER TABLE extension_ref ADD CONSTRAINT extension_ref_fk0 FOREIGN KEY (id_program) REFERENCES program_ref(id);

ALTER TABLE file_ref ADD CONSTRAINT file_ref_fk0 FOREIGN KEY (uuid_file_parent) REFERENCES file_ref(uuid);
ALTER TABLE file_ref ADD CONSTRAINT file_ref_fk1 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid) ON DELETE CASCADE;
ALTER TABLE file_ref ADD CONSTRAINT file_ref_fk2 FOREIGN KEY (id_ext) REFERENCES extension_ref(id);

ALTER TABLE actual_status_translate_list ADD CONSTRAINT actual_status_translate_list_fk0 FOREIGN KEY (id_actual_status) REFERENCES actual_status_ref(id);
ALTER TABLE actual_status_translate_list ADD CONSTRAINT actual_status_translate_list_fk1 FOREIGN KEY (id_lang) REFERENCES language_ref(id);

ALTER TABLE type_access_translate_list ADD CONSTRAINT type_access_translate_list_fk0 FOREIGN KEY (id_type_access) REFERENCES type_access_ref(id);
ALTER TABLE type_access_translate_list ADD CONSTRAINT type_access_translate_list_fk1 FOREIGN KEY (id_lang) REFERENCES language_ref(id);

ALTER TABLE param_translate_list ADD CONSTRAINT param_translate_list_fk0 FOREIGN KEY (id_param) REFERENCES param_ref(id);
ALTER TABLE param_translate_list ADD CONSTRAINT param_translate_list_fk1 FOREIGN KEY (id_lang) REFERENCES language_ref(id);

ALTER TABLE region_translate_list ADD CONSTRAINT region_translate_list_fk0 FOREIGN KEY (id_region) REFERENCES region_ref(id);
ALTER TABLE region_translate_list ADD CONSTRAINT region_translate_list_fk1 FOREIGN KEY (id_lang) REFERENCES language_ref(id);

ALTER TABLE spec_ref ADD CONSTRAINT spec_ref_fk0 FOREIGN KEY (id_spec_parent) REFERENCES spec_ref(id);

ALTER TABLE spec_translate_list ADD CONSTRAINT spec_translate_list_fk0 FOREIGN KEY (id_spec) REFERENCES spec_ref(id);
ALTER TABLE spec_translate_list ADD CONSTRAINT spec_translate_list_fk1 FOREIGN KEY (id_lang) REFERENCES language_ref(id);

ALTER TABLE type_of_change_translate_list ADD CONSTRAINT type_of_change_translate_list_fk0 FOREIGN KEY (id_type_of_change) REFERENCES type_of_change_ref(id);
ALTER TABLE type_of_change_translate_list ADD CONSTRAINT type_of_change_translate_list_fk1 FOREIGN KEY (id_lang) REFERENCES language_ref(id);
