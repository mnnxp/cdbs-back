-- Your SQL goes here
ALTER TABLE standard_ref ADD CONSTRAINT standard_ref_fk0 FOREIGN KEY (parent_standard_uuid) REFERENCES standard_ref(uuid) ON DELETE CASCADE;
ALTER TABLE standard_ref ADD CONSTRAINT standard_ref_fk1 FOREIGN KEY (image_file_uuid) REFERENCES file_ref(uuid) ON DELETE CASCADE;
ALTER TABLE standard_ref ADD CONSTRAINT standard_ref_fk2 FOREIGN KEY (user_uuid) REFERENCES user_ref(uuid) ON DELETE CASCADE;
ALTER TABLE standard_ref ADD CONSTRAINT standard_ref_fk3 FOREIGN KEY (company_uuid) REFERENCES company_ref(uuid) ON DELETE CASCADE;
ALTER TABLE standard_ref ADD CONSTRAINT standard_ref_fk4 FOREIGN KEY (standard_status_id) REFERENCES standard_status_ref(id) ON DELETE CASCADE;
ALTER TABLE standard_ref ADD CONSTRAINT standard_ref_fk5 FOREIGN KEY (type_access_id) REFERENCES type_access_ref(id) ON DELETE CASCADE;
ALTER TABLE standard_ref ADD CONSTRAINT standard_ref_fk6 FOREIGN KEY (region_id) REFERENCES region_ref(id) ON DELETE CASCADE;

ALTER TABLE standard_history_list ADD CONSTRAINT standard_history_list_fk0 FOREIGN KEY (standard_uuid) REFERENCES standard_ref(uuid) ON DELETE CASCADE;
ALTER TABLE standard_history_list ADD CONSTRAINT standard_history_list_fk1 FOREIGN KEY (type_of_change_id) REFERENCES type_of_change_ref(id) ON DELETE CASCADE;

ALTER TABLE file_to_standard ADD CONSTRAINT file_to_standard_fk0 FOREIGN KEY (file_uuid) REFERENCES file_ref(uuid) ON DELETE CASCADE;
ALTER TABLE file_to_standard ADD CONSTRAINT file_to_standard_fk1 FOREIGN KEY (standard_uuid) REFERENCES standard_ref(uuid) ON DELETE CASCADE;

ALTER TABLE standard_status_translate_list ADD CONSTRAINT standard_status_translate_list_fk0 FOREIGN KEY (standard_status_id) REFERENCES standard_status_ref(id) ON DELETE CASCADE;
ALTER TABLE standard_status_translate_list ADD CONSTRAINT standard_status_translate_list_fk1 FOREIGN KEY (lang_id) REFERENCES language_ref(id) ON DELETE CASCADE;

ALTER TABLE spec_to_standard ADD CONSTRAINT spec_to_standard_fk0 FOREIGN KEY (spec_id) REFERENCES spec_ref(id) ON DELETE CASCADE;
ALTER TABLE spec_to_standard ADD CONSTRAINT spec_to_standard_fk1 FOREIGN KEY (standard_uuid) REFERENCES standard_ref(uuid) ON DELETE CASCADE;

ALTER TABLE keyword_to_standard ADD CONSTRAINT keyword_to_standard_fk0 FOREIGN KEY (standard_uuid) REFERENCES standard_ref(uuid) ON DELETE CASCADE;
ALTER TABLE keyword_to_standard ADD CONSTRAINT keyword_to_standard_fk1 FOREIGN KEY (keyword_id) REFERENCES keyword_ref(id) ON DELETE CASCADE;
