-- Your SQL goes here
ALTER TABLE standard_ref ADD CONSTRAINT standard_ref_fk0 FOREIGN KEY (uuid_standard_parent) REFERENCES standard_ref(uuid);
ALTER TABLE standard_ref ADD CONSTRAINT standard_ref_fk1 FOREIGN KEY (uuid_image_file) REFERENCES file_ref(uuid);
ALTER TABLE standard_ref ADD CONSTRAINT standard_ref_fk2 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid) ON DELETE CASCADE;
ALTER TABLE standard_ref ADD CONSTRAINT standard_ref_fk3 FOREIGN KEY (uuid_company) REFERENCES company_ref(uuid);
ALTER TABLE standard_ref ADD CONSTRAINT standard_ref_fk4 FOREIGN KEY (id_standard_status) REFERENCES standard_status_ref(id);
ALTER TABLE standard_ref ADD CONSTRAINT standard_ref_fk5 FOREIGN KEY (id_type_access) REFERENCES type_access_ref(id);
ALTER TABLE standard_ref ADD CONSTRAINT standard_ref_fk6 FOREIGN KEY (id_region) REFERENCES region_ref(id);

ALTER TABLE standard_history_list ADD CONSTRAINT standard_history_list_fk0 FOREIGN KEY (uuid_standard) REFERENCES standard_ref(uuid);
ALTER TABLE standard_history_list ADD CONSTRAINT standard_history_list_fk1 FOREIGN KEY (id_type_of_change) REFERENCES type_of_change_ref(id);

ALTER TABLE file_to_standard ADD CONSTRAINT file_to_standard_fk0 FOREIGN KEY (uuid_file) REFERENCES file_ref(uuid) ON DELETE CASCADE;
ALTER TABLE file_to_standard ADD CONSTRAINT file_to_standard_fk1 FOREIGN KEY (uuid_standard) REFERENCES standard_ref(uuid) ON DELETE CASCADE;

ALTER TABLE standard_status_translate_list ADD CONSTRAINT standard_status_translate_list_fk0 FOREIGN KEY (id_standard_status) REFERENCES standard_status_ref(id) ON DELETE CASCADE;
ALTER TABLE standard_status_translate_list ADD CONSTRAINT standard_status_translate_list_fk1 FOREIGN KEY (id_lang) REFERENCES language_ref(id);

ALTER TABLE spec_to_standard ADD CONSTRAINT spec_to_standard_fk0 FOREIGN KEY (id_spec) REFERENCES spec_ref(id) ON DELETE CASCADE;
ALTER TABLE spec_to_standard ADD CONSTRAINT spec_to_standard_fk1 FOREIGN KEY (uuid_standard) REFERENCES standard_ref(uuid) ON DELETE CASCADE;

ALTER TABLE standard_to_keyword ADD CONSTRAINT standard_to_keyword_fk0 FOREIGN KEY (uuid_standard) REFERENCES standard_ref(uuid) ON DELETE CASCADE;
ALTER TABLE standard_to_keyword ADD CONSTRAINT standard_to_keyword_fk1 FOREIGN KEY (id_keyword) REFERENCES keyword_ref(id) ON DELETE CASCADE;
