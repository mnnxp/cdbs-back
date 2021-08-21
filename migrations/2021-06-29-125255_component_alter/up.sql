-- Your SQL goes here
ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk0 FOREIGN KEY (uuid_component_parent) REFERENCES component_ref(uuid);
ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk1 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid) ON DELETE CASCADE;
ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk2 FOREIGN KEY (id_type_access) REFERENCES type_access_ref(id);
ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk3 FOREIGN KEY (id_component_type) REFERENCES component_type_ref(id);
ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk4 FOREIGN KEY (id_actual_status) REFERENCES actual_status_ref(id);

ALTER TABLE component_history_list ADD CONSTRAINT component_history_list_fk0 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid);
ALTER TABLE component_history_list ADD CONSTRAINT standard_history_list_fk1 FOREIGN KEY (id_type_of_change) REFERENCES type_of_change_ref(id);

ALTER TABLE component_type_translate_list ADD CONSTRAINT component_type_translate_list_fk0 FOREIGN KEY (id_component_type) REFERENCES component_type_ref(id) ON DELETE CASCADE;
ALTER TABLE component_type_translate_list ADD CONSTRAINT component_type_translate_list_fk1 FOREIGN KEY (id_lang) REFERENCES language_ref(id);

ALTER TABLE keyword_to_component ADD CONSTRAINT keyword_to_component_fk0 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid) ON DELETE CASCADE;
ALTER TABLE keyword_to_component ADD CONSTRAINT keyword_to_component_fk1 FOREIGN KEY (id_keyword) REFERENCES keyword_ref(id) ON DELETE CASCADE;

ALTER TABLE param_to_component ADD CONSTRAINT param_to_component_fk0 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid) ON DELETE CASCADE;
ALTER TABLE param_to_component ADD CONSTRAINT param_to_component_fk1 FOREIGN KEY (id_param) REFERENCES param_ref(id) ON DELETE CASCADE;

ALTER TABLE supplier_to_component ADD CONSTRAINT supplier_to_component_fk0 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid) ON DELETE CASCADE;
ALTER TABLE supplier_to_component ADD CONSTRAINT supplier_to_component_fk1 FOREIGN KEY (uuid_company) REFERENCES company_ref(uuid) ON DELETE CASCADE;

ALTER TABLE discussion_component_ref ADD CONSTRAINT discussion_component_ref_fk0 FOREIGN KEY (id_discussion_parent) REFERENCES discussion_component_ref(id);
ALTER TABLE discussion_component_ref ADD CONSTRAINT discussion_component_ref_fk1 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid);
ALTER TABLE discussion_component_ref ADD CONSTRAINT discussion_component_ref_fk2 FOREIGN KEY (uuid_author) REFERENCES user_ref(uuid);

ALTER TABLE component_modification_list ADD CONSTRAINT component_modification_list_fk0 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid);
ALTER TABLE component_modification_list ADD CONSTRAINT component_modification_list_fk1 FOREIGN KEY (uuid_modification_parent) REFERENCES component_modification_list(uuid);
ALTER TABLE component_modification_list ADD CONSTRAINT component_modification_list_fk2 FOREIGN KEY (id_actual_status) REFERENCES actual_status_ref(id);

ALTER TABLE param_to_modification ADD CONSTRAINT param_to_modification_fk0 FOREIGN KEY (uuid_modification) REFERENCES component_modification_list(uuid) ON DELETE CASCADE;
ALTER TABLE param_to_modification ADD CONSTRAINT param_to_modification_fk1 FOREIGN KEY (id_param) REFERENCES param_ref(id) ON DELETE CASCADE;

ALTER TABLE file_to_component ADD CONSTRAINT file_to_component_fk0 FOREIGN KEY (uuid_file) REFERENCES file_ref(uuid) ON DELETE CASCADE;
ALTER TABLE file_to_component ADD CONSTRAINT file_to_component_fk1 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid) ON DELETE CASCADE;

ALTER TABLE file_to_modification ADD CONSTRAINT file_to_modification_fk0 FOREIGN KEY (uuid_modification) REFERENCES component_modification_list(uuid) ON DELETE CASCADE;
ALTER TABLE file_to_modification ADD CONSTRAINT file_to_modification_fk1 FOREIGN KEY (uuid_file) REFERENCES file_ref(uuid) ON DELETE CASCADE;

ALTER TABLE set_files_for_program ADD CONSTRAINT set_files_for_program_fk0 FOREIGN KEY (uuid_modification) REFERENCES component_modification_list(uuid);
ALTER TABLE set_files_for_program ADD CONSTRAINT set_files_for_program_fk1 FOREIGN KEY (id_program) REFERENCES program_ref(id);

ALTER TABLE file_to_set_modification ADD CONSTRAINT file_to_set_modification_fk0 FOREIGN KEY (id_set) REFERENCES set_files_for_program(id) ON DELETE CASCADE;
ALTER TABLE file_to_set_modification ADD CONSTRAINT file_to_set_modification_fk1 FOREIGN KEY (uuid_file) REFERENCES file_ref(uuid) ON DELETE CASCADE;

ALTER TABLE spec_to_component ADD CONSTRAINT spec_to_component_fk0 FOREIGN KEY (id_spec) REFERENCES spec_ref(id) ON DELETE CASCADE;
ALTER TABLE spec_to_component ADD CONSTRAINT spec_to_component_fk1 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid) ON DELETE CASCADE;

ALTER TABLE license_to_component ADD CONSTRAINT license_to_component_fk0 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid) ON DELETE CASCADE;
ALTER TABLE license_to_component ADD CONSTRAINT license_to_component_fk1 FOREIGN KEY (id_license) REFERENCES license_ref(id) ON DELETE CASCADE;

ALTER TABLE standard_to_component ADD CONSTRAINT standard_to_component_fk0 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid) ON DELETE CASCADE;
ALTER TABLE standard_to_component ADD CONSTRAINT standard_to_component_fk1 FOREIGN KEY (uuid_standard) REFERENCES standard_ref(uuid) ON DELETE CASCADE;
