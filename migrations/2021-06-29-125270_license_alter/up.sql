-- Your SQL goes here
ALTER TABLE permission_to_license ADD CONSTRAINT permission_to_license_fk0 FOREIGN KEY (id_permission) REFERENCES license_permission_ref(id) ON DELETE CASCADE;
ALTER TABLE permission_to_license ADD CONSTRAINT permission_to_license_fk1 FOREIGN KEY (id_license) REFERENCES license_ref(id) ON DELETE CASCADE;

ALTER TABLE limitation_to_license ADD CONSTRAINT limitation_to_license_fk0 FOREIGN KEY (id_limitation) REFERENCES license_limitation_ref(id) ON DELETE CASCADE;
ALTER TABLE limitation_to_license ADD CONSTRAINT limitation_to_license_fk1 FOREIGN KEY (id_license) REFERENCES license_ref(id) ON DELETE CASCADE;

ALTER TABLE condition_to_license ADD CONSTRAINT condition_to_license_fk0 FOREIGN KEY (id_condition) REFERENCES license_condition_ref(id) ON DELETE CASCADE;
ALTER TABLE condition_to_license ADD CONSTRAINT condition_to_license_fk1 FOREIGN KEY (id_license) REFERENCES license_ref(id) ON DELETE CASCADE;

ALTER TABLE license_permission_translate_list ADD CONSTRAINT license_permission_translate_list_fk0 FOREIGN KEY (id_license_permission) REFERENCES license_permission_ref(id);
ALTER TABLE license_permission_translate_list ADD CONSTRAINT license_permission_translate_list_fk1 FOREIGN KEY (id_lang) REFERENCES language_ref(id);

ALTER TABLE license_limitation_translate_list ADD CONSTRAINT license_limitation_translate_list_fk0 FOREIGN KEY (id_license_limitation) REFERENCES license_limitation_ref(id);
ALTER TABLE license_limitation_translate_list ADD CONSTRAINT license_limitation_translate_list_fk1 FOREIGN KEY (id_lang) REFERENCES language_ref(id);

ALTER TABLE license_condition_translate_list ADD CONSTRAINT license_condition_translate_list_fk0 FOREIGN KEY (id_license_condition) REFERENCES license_condition_ref(id);
ALTER TABLE license_condition_translate_list ADD CONSTRAINT license_condition_translate_list_fk1 FOREIGN KEY (id_lang) REFERENCES language_ref(id);
