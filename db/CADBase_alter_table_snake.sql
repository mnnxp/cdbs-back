ALTER TABLE client_ref ADD CONSTRAINT client_ref_fk0 FOREIGN KEY (id_type_org) REFERENCES type_org_ref(id);
ALTER TABLE client_ref ADD CONSTRAINT client_ref_fk1 FOREIGN KEY (id_name_cad) REFERENCES name_cad_ref(id);
ALTER TABLE client_ref ADD CONSTRAINT client_ref_fk2 FOREIGN KEY (id_file_info_icon) REFERENCES file_ref(id);
ALTER TABLE client_ref ADD CONSTRAINT client_ref_fk3 FOREIGN KEY (id_region) REFERENCES region_ref(id);

ALTER TABLE client_tokens_ref ADD CONSTRAINT client_tokens_ref_fk0 FOREIGN KEY (id_client) REFERENCES client_ref(id);

ALTER TABLE spec_ref ADD CONSTRAINT spec_ref_fk0 FOREIGN KEY (id_spec_parent) REFERENCES spec_ref(id);

ALTER TABLE spec2_client ADD CONSTRAINT spec2_client_fk0 FOREIGN KEY (id_spec) REFERENCES spec_ref(id);
ALTER TABLE spec2_client ADD CONSTRAINT spec2_client_fk1 FOREIGN KEY (id_client) REFERENCES client_ref(id);

ALTER TABLE client_history_list ADD CONSTRAINT client_history_list_fk0 FOREIGN KEY (id_client) REFERENCES client_ref(id);
ALTER TABLE client_history_list ADD CONSTRAINT client_history_list_fk1 FOREIGN KEY (id_type_of_change) REFERENCES type_of_change_ref(id);

ALTER TABLE file_ref ADD CONSTRAINT file_ref_fk0 FOREIGN KEY (id_file) REFERENCES file_ref(id);
ALTER TABLE file_ref ADD CONSTRAINT file_ref_fk1 FOREIGN KEY (id_client_create) REFERENCES client_ref(id);
ALTER TABLE file_ref ADD CONSTRAINT file_ref_fk2 FOREIGN KEY (id_ext) REFERENCES extension_ref(id);

ALTER TABLE extension_ref ADD CONSTRAINT extension_ref_fk0 FOREIGN KEY (id_name_cad) REFERENCES name_cad_ref(id);

ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk0 FOREIGN KEY (id_client) REFERENCES client_ref(id);
ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk1 FOREIGN KEY (id_component_parent) REFERENCES component_ref(id);
ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk2 FOREIGN KEY (id_actual_status) REFERENCES actual_status_ref(id);
ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk3 FOREIGN KEY (id_component_type) REFERENCES component_type_ref(id);
ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk4 FOREIGN KEY (id_type_access) REFERENCES type_access_ref(id);

ALTER TABLE spec2_component ADD CONSTRAINT spec2_component_fk0 FOREIGN KEY (id_spec) REFERENCES spec_ref(id);
ALTER TABLE spec2_component ADD CONSTRAINT spec2_component_fk1 FOREIGN KEY (id_component) REFERENCES component_ref(id);

ALTER TABLE file2_component ADD CONSTRAINT file2_component_fk0 FOREIGN KEY (id_file) REFERENCES file_ref(id);
ALTER TABLE file2_component ADD CONSTRAINT file2_component_fk1 FOREIGN KEY (id_component) REFERENCES component_ref(id);

ALTER TABLE component2_keyword ADD CONSTRAINT component2_keyword_fk0 FOREIGN KEY (id_component) REFERENCES component_ref(id);
ALTER TABLE component2_keyword ADD CONSTRAINT component2_keyword_fk1 FOREIGN KEY (id_component_keyword) REFERENCES component_keyword_ref(id);

ALTER TABLE discussion_ref ADD CONSTRAINT discussion_ref_fk0 FOREIGN KEY (id_component) REFERENCES component_ref(id);
ALTER TABLE discussion_ref ADD CONSTRAINT discussion_ref_fk1 FOREIGN KEY (id_client_from) REFERENCES client_ref(id);
ALTER TABLE discussion_ref ADD CONSTRAINT discussion_ref_fk2 FOREIGN KEY (id_client_to) REFERENCES client_ref(id);
ALTER TABLE discussion_ref ADD CONSTRAINT discussion_ref_fk3 FOREIGN KEY (id_discussion_parent) REFERENCES discussion_ref(id);

ALTER TABLE param2_component ADD CONSTRAINT param2_component_fk0 FOREIGN KEY (id_component) REFERENCES component_ref(id);
ALTER TABLE param2_component ADD CONSTRAINT param2_component_fk1 FOREIGN KEY (id_param) REFERENCES param_ref(id);

ALTER TABLE component_fav_ref ADD CONSTRAINT component_fav_ref_fk0 FOREIGN KEY (id_component) REFERENCES component_ref(id);
ALTER TABLE component_fav_ref ADD CONSTRAINT component_fav_ref_fk1 FOREIGN KEY (id_client) REFERENCES client_ref(id);

ALTER TABLE spec_translate_list ADD CONSTRAINT spec_translate_list_fk0 FOREIGN KEY (id_spec) REFERENCES spec_ref(id);
ALTER TABLE spec_translate_list ADD CONSTRAINT spec_translate_list_fk1 FOREIGN KEY (id_lang) REFERENCES language_ref(id);

ALTER TABLE param_translate_list ADD CONSTRAINT param_translate_list_fk0 FOREIGN KEY (id_param) REFERENCES param_ref(id);
ALTER TABLE param_translate_list ADD CONSTRAINT param_translate_list_fk1 FOREIGN KEY (id_lang) REFERENCES language_ref(id);

ALTER TABLE client_represet_ref ADD CONSTRAINT client_represet_ref_fk0 FOREIGN KEY (id_client) REFERENCES client_ref(id);
ALTER TABLE client_represet_ref ADD CONSTRAINT client_represet_ref_fk1 FOREIGN KEY (id_representation_type) REFERENCES representation_type_ref(id);
ALTER TABLE client_represet_ref ADD CONSTRAINT client_represet_ref_fk2 FOREIGN KEY (id_region) REFERENCES region_ref(id);

ALTER TABLE component_access2_client ADD CONSTRAINT component_access2_client_fk0 FOREIGN KEY (id_component) REFERENCES component_ref(id);
ALTER TABLE component_access2_client ADD CONSTRAINT component_access2_client_fk1 FOREIGN KEY (id_client) REFERENCES client_ref(id);
ALTER TABLE component_access2_client ADD CONSTRAINT component_access2_client_fk2 FOREIGN KEY (id_type_access) REFERENCES type_access_ref(id);

ALTER TABLE component2_client ADD CONSTRAINT component2_client_fk0 FOREIGN KEY (id_component) REFERENCES component_ref(id);
ALTER TABLE component2_client ADD CONSTRAINT component2_client_fk1 FOREIGN KEY (id_client) REFERENCES client_ref(id);

ALTER TABLE component_modification_list ADD CONSTRAINT component_modification_list_fk0 FOREIGN KEY (id_component) REFERENCES component_ref(id);
ALTER TABLE component_modification_list ADD CONSTRAINT component_modification_list_fk1 FOREIGN KEY (id_name_cad) REFERENCES name_cad_ref(id);
ALTER TABLE component_modification_list ADD CONSTRAINT component_modification_list_fk2 FOREIGN KEY (id_modification_parent) REFERENCES component_modification_list(id);
ALTER TABLE component_modification_list ADD CONSTRAINT component_modification_list_fk3 FOREIGN KEY (id_actual_status) REFERENCES actual_status_ref(id);

ALTER TABLE file2_modification ADD CONSTRAINT file2_modification_fk0 FOREIGN KEY (id_modification) REFERENCES component_modification_list(id);
ALTER TABLE file2_modification ADD CONSTRAINT file2_modification_fk1 FOREIGN KEY (id_file) REFERENCES file_ref(id);

ALTER TABLE param2_modification ADD CONSTRAINT param2_modification_fk0 FOREIGN KEY (id_modification) REFERENCES component_modification_list(id);
ALTER TABLE param2_modification ADD CONSTRAINT param2_modification_fk1 FOREIGN KEY (id_param) REFERENCES param_ref(id);
