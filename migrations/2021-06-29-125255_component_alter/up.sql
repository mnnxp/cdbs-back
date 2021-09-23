-- Your SQL goes here
ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk0 FOREIGN KEY (parent_component_uuid) REFERENCES component_ref(uuid) ON DELETE CASCADE;
ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk1 FOREIGN KEY (user_uuid) REFERENCES user_ref(uuid) ON DELETE CASCADE;
ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk2 FOREIGN KEY (type_access_id) REFERENCES type_access_ref(id) ON DELETE CASCADE;
ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk3 FOREIGN KEY (component_type_id) REFERENCES component_type_ref(id) ON DELETE CASCADE;
ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk4 FOREIGN KEY (actual_status_id) REFERENCES actual_status_ref(id) ON DELETE CASCADE;

ALTER TABLE component_history_list ADD CONSTRAINT component_history_list_fk0 FOREIGN KEY (component_uuid) REFERENCES component_ref(uuid) ON DELETE CASCADE;
ALTER TABLE component_history_list ADD CONSTRAINT standard_history_list_fk1 FOREIGN KEY (type_of_change_id) REFERENCES type_of_change_ref(id) ON DELETE CASCADE;

ALTER TABLE component_type_translate_list ADD CONSTRAINT component_type_translate_list_fk0 FOREIGN KEY (component_type_id) REFERENCES component_type_ref(id) ON DELETE CASCADE;
ALTER TABLE component_type_translate_list ADD CONSTRAINT component_type_translate_list_fk1 FOREIGN KEY (lang_id) REFERENCES language_ref(id) ON DELETE CASCADE;

ALTER TABLE keyword_to_component ADD CONSTRAINT keyword_to_component_fk0 FOREIGN KEY (component_uuid) REFERENCES component_ref(uuid) ON DELETE CASCADE;
ALTER TABLE keyword_to_component ADD CONSTRAINT keyword_to_component_fk1 FOREIGN KEY (keyword_id) REFERENCES keyword_ref(id) ON DELETE CASCADE;

ALTER TABLE param_to_component ADD CONSTRAINT param_to_component_fk0 FOREIGN KEY (component_uuid) REFERENCES component_ref(uuid) ON DELETE CASCADE;
ALTER TABLE param_to_component ADD CONSTRAINT param_to_component_fk1 FOREIGN KEY (param_id) REFERENCES param_ref(id) ON DELETE CASCADE;

ALTER TABLE supplier_to_component ADD CONSTRAINT supplier_to_component_fk0 FOREIGN KEY (component_uuid) REFERENCES component_ref(uuid) ON DELETE CASCADE;
ALTER TABLE supplier_to_component ADD CONSTRAINT supplier_to_component_fk1 FOREIGN KEY (company_uuid) REFERENCES company_ref(uuid) ON DELETE CASCADE;

ALTER TABLE discussion_component_ref ADD CONSTRAINT discussion_component_ref_fk0 FOREIGN KEY (parent_discussion_id) REFERENCES discussion_component_ref(id) ON DELETE CASCADE;
ALTER TABLE discussion_component_ref ADD CONSTRAINT discussion_component_ref_fk1 FOREIGN KEY (component_uuid) REFERENCES component_ref(uuid) ON DELETE CASCADE;
ALTER TABLE discussion_component_ref ADD CONSTRAINT discussion_component_ref_fk2 FOREIGN KEY (author_uuid) REFERENCES user_ref(uuid) ON DELETE CASCADE;

ALTER TABLE component_modification_list ADD CONSTRAINT component_modification_list_fk0 FOREIGN KEY (component_uuid) REFERENCES component_ref(uuid) ON DELETE CASCADE;
ALTER TABLE component_modification_list ADD CONSTRAINT component_modification_list_fk1 FOREIGN KEY (parent_modification_uuid) REFERENCES component_modification_list(uuid) ON DELETE CASCADE;
ALTER TABLE component_modification_list ADD CONSTRAINT component_modification_list_fk2 FOREIGN KEY (actual_status_id) REFERENCES actual_status_ref(id) ON DELETE CASCADE;

ALTER TABLE param_to_modification ADD CONSTRAINT param_to_modification_fk0 FOREIGN KEY (modification_uuid) REFERENCES component_modification_list(uuid) ON DELETE CASCADE;
ALTER TABLE param_to_modification ADD CONSTRAINT param_to_modification_fk1 FOREIGN KEY (param_id) REFERENCES param_ref(id) ON DELETE CASCADE;

ALTER TABLE file_to_component ADD CONSTRAINT file_to_component_fk0 FOREIGN KEY (file_uuid) REFERENCES file_ref(uuid) ON DELETE CASCADE;
ALTER TABLE file_to_component ADD CONSTRAINT file_to_component_fk1 FOREIGN KEY (component_uuid) REFERENCES component_ref(uuid) ON DELETE CASCADE;

ALTER TABLE file_to_modification ADD CONSTRAINT file_to_modification_fk0 FOREIGN KEY (modification_uuid) REFERENCES component_modification_list(uuid) ON DELETE CASCADE;
ALTER TABLE file_to_modification ADD CONSTRAINT file_to_modification_fk1 FOREIGN KEY (file_uuid) REFERENCES file_ref(uuid) ON DELETE CASCADE;

ALTER TABLE fileset_for_program ADD CONSTRAINT fileset_for_program_fk0 FOREIGN KEY (modification_uuid) REFERENCES component_modification_list(uuid) ON DELETE CASCADE;
ALTER TABLE fileset_for_program ADD CONSTRAINT fileset_for_program_fk1 FOREIGN KEY (program_id) REFERENCES program_ref(id) ON DELETE CASCADE;

ALTER TABLE modification_file_from_fileset ADD CONSTRAINT modification_file_from_fileset_fk0 FOREIGN KEY (fileset_uuid) REFERENCES fileset_for_program(uuid) ON DELETE CASCADE;
ALTER TABLE modification_file_from_fileset ADD CONSTRAINT modification_file_from_fileset_fk1 FOREIGN KEY (file_uuid) REFERENCES file_ref(uuid) ON DELETE CASCADE;

ALTER TABLE spec_to_component ADD CONSTRAINT spec_to_component_fk0 FOREIGN KEY (spec_id) REFERENCES spec_ref(id) ON DELETE CASCADE;
ALTER TABLE spec_to_component ADD CONSTRAINT spec_to_component_fk1 FOREIGN KEY (component_uuid) REFERENCES component_ref(uuid) ON DELETE CASCADE;

ALTER TABLE license_to_component ADD CONSTRAINT license_to_component_fk0 FOREIGN KEY (component_uuid) REFERENCES component_ref(uuid) ON DELETE CASCADE;
ALTER TABLE license_to_component ADD CONSTRAINT license_to_component_fk1 FOREIGN KEY (license_id) REFERENCES license_ref(id) ON DELETE CASCADE;

ALTER TABLE standard_to_component ADD CONSTRAINT standard_to_component_fk0 FOREIGN KEY (component_uuid) REFERENCES component_ref(uuid) ON DELETE CASCADE;
ALTER TABLE standard_to_component ADD CONSTRAINT standard_to_component_fk1 FOREIGN KEY (standard_uuid) REFERENCES standard_ref(uuid) ON DELETE CASCADE;
