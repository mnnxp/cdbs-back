-- Your SQL goes here
ALTER TABLE extension_ref ADD CONSTRAINT extension_ref_fk0 FOREIGN KEY (id_program) REFERENCES program_ref(id);

ALTER TABLE file_ref ADD CONSTRAINT file_ref_fk0 FOREIGN KEY (uuid_file_parent) REFERENCES file_ref(uuid);
ALTER TABLE file_ref ADD CONSTRAINT file_ref_fk1 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);
ALTER TABLE file_ref ADD CONSTRAINT file_ref_fk2 FOREIGN KEY (id_ext) REFERENCES extension_ref(id);

ALTER TABLE param_translate_list ADD CONSTRAINT param_translate_list_fk0 FOREIGN KEY (id_param) REFERENCES param_ref(id);
ALTER TABLE param_translate_list ADD CONSTRAINT param_translate_list_fk1 FOREIGN KEY (id_lang) REFERENCES language_ref(id);

ALTER TABLE spec_translate_list ADD CONSTRAINT spec_translate_list_fk0 FOREIGN KEY (id_spec) REFERENCES spec_ref(id);
ALTER TABLE spec_translate_list ADD CONSTRAINT spec_translate_list_fk1 FOREIGN KEY (id_lang) REFERENCES language_ref(id);

ALTER TABLE spec_ref ADD CONSTRAINT spec_ref_fk0 FOREIGN KEY (id_spec_parent) REFERENCES spec_ref(id);
