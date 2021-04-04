
ALTER TABLE user_ref ADD CONSTRAINT user_ref_fk0 FOREIGN KEY (id_type_user) REFERENCES type_user_ref(id);
ALTER TABLE user_ref ADD CONSTRAINT user_ref_fk1 FOREIGN KEY (id_name_cad) REFERENCES name_cad_ref(id);
ALTER TABLE user_ref ADD CONSTRAINT user_ref_fk2 FOREIGN KEY (uuid_file_info_icon) REFERENCES file_ref(uuid);
ALTER TABLE user_ref ADD CONSTRAINT user_ref_fk3 FOREIGN KEY (id_region) REFERENCES region_ref(id);

ALTER TABLE user_tokens_ref ADD CONSTRAINT user_tokens_ref_fk0 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);

ALTER TABLE spec_ref ADD CONSTRAINT spec_ref_fk0 FOREIGN KEY (id_spec_parent) REFERENCES spec_ref(id);

ALTER TABLE spec_to_user ADD CONSTRAINT spec_to_user_fk0 FOREIGN KEY (id_spec) REFERENCES spec_ref(id);
ALTER TABLE spec_to_user ADD CONSTRAINT spec_to_user_fk1 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);

ALTER TABLE user_history_list ADD CONSTRAINT user_history_list_fk0 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);
ALTER TABLE user_history_list ADD CONSTRAINT user_history_list_fk1 FOREIGN KEY (id_type_of_change) REFERENCES type_of_change_ref(id);

ALTER TABLE file_ref ADD CONSTRAINT file_ref_fk0 FOREIGN KEY (uuid_file_parent) REFERENCES file_ref(uuid);
ALTER TABLE file_ref ADD CONSTRAINT file_ref_fk1 FOREIGN KEY (uuid_user_create) REFERENCES user_ref(uuid);
ALTER TABLE file_ref ADD CONSTRAINT file_ref_fk2 FOREIGN KEY (id_ext) REFERENCES extension_ref(id);

ALTER TABLE extension_ref ADD CONSTRAINT extension_ref_fk0 FOREIGN KEY (id_name_cad) REFERENCES name_cad_ref(id);

ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk0 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);
ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk1 FOREIGN KEY (uuid_component_parent) REFERENCES component_ref(uuid);
ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk2 FOREIGN KEY (id_actual_status) REFERENCES actual_status_ref(id);
ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk3 FOREIGN KEY (id_component_type) REFERENCES component_type_ref(id);
ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk4 FOREIGN KEY (id_type_access) REFERENCES type_access_ref(id);

ALTER TABLE spec_to_component ADD CONSTRAINT spec_to_component_fk0 FOREIGN KEY (id_spec) REFERENCES spec_ref(id);
ALTER TABLE spec_to_component ADD CONSTRAINT spec_to_component_fk1 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid);

ALTER TABLE file_to_component ADD CONSTRAINT file_to_component_fk0 FOREIGN KEY (uuid_file) REFERENCES file_ref(uuid);
ALTER TABLE file_to_component ADD CONSTRAINT file_to_component_fk1 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid);

ALTER TABLE component_to_keyword ADD CONSTRAINT component_to_keyword_fk0 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid);
ALTER TABLE component_to_keyword ADD CONSTRAINT component_to_keyword_fk1 FOREIGN KEY (id_component_keyword) REFERENCES component_keyword_ref(id);

ALTER TABLE discussion_ref ADD CONSTRAINT discussion_ref_fk0 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid);
ALTER TABLE discussion_ref ADD CONSTRAINT discussion_ref_fk1 FOREIGN KEY (uuid_user_from) REFERENCES user_ref(uuid);
ALTER TABLE discussion_ref ADD CONSTRAINT discussion_ref_fk2 FOREIGN KEY (uuid_user_to) REFERENCES user_ref(uuid);
ALTER TABLE discussion_ref ADD CONSTRAINT discussion_ref_fk3 FOREIGN KEY (id_discussion_parent) REFERENCES discussion_ref(id);

ALTER TABLE param_to_component ADD CONSTRAINT param_to_component_fk0 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid);
ALTER TABLE param_to_component ADD CONSTRAINT param_to_component_fk1 FOREIGN KEY (id_param) REFERENCES param_ref(id);

ALTER TABLE component_fav_ref ADD CONSTRAINT component_fav_ref_fk0 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid);
ALTER TABLE component_fav_ref ADD CONSTRAINT component_fav_ref_fk1 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);

ALTER TABLE spec_translate_list ADD CONSTRAINT spec_translate_list_fk0 FOREIGN KEY (id_spec) REFERENCES spec_ref(id);
ALTER TABLE spec_translate_list ADD CONSTRAINT spec_translate_list_fk1 FOREIGN KEY (id_lang) REFERENCES language_ref(id);

ALTER TABLE param_translate_list ADD CONSTRAINT param_translate_list_fk0 FOREIGN KEY (id_param) REFERENCES param_ref(id);
ALTER TABLE param_translate_list ADD CONSTRAINT param_translate_list_fk1 FOREIGN KEY (id_lang) REFERENCES language_ref(id);

ALTER TABLE user_represet_ref ADD CONSTRAINT user_represet_ref_fk0 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);
ALTER TABLE user_represet_ref ADD CONSTRAINT user_represet_ref_fk1 FOREIGN KEY (id_representation_type) REFERENCES representation_type_ref(id);
ALTER TABLE user_represet_ref ADD CONSTRAINT user_represet_ref_fk2 FOREIGN KEY (id_region) REFERENCES region_ref(id);

ALTER TABLE component_access_to_user ADD CONSTRAINT component_access_to_user_fk0 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid);
ALTER TABLE component_access_to_user ADD CONSTRAINT component_access_to_user_fk1 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);
ALTER TABLE component_access_to_user ADD CONSTRAINT component_access_to_user_fk2 FOREIGN KEY (id_type_access) REFERENCES type_access_ref(id);

ALTER TABLE component_to_user ADD CONSTRAINT component_to_user_fk0 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid);
ALTER TABLE component_to_user ADD CONSTRAINT component_to_user_fk1 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);

ALTER TABLE component_modification_list ADD CONSTRAINT component_modification_list_fk0 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid);
ALTER TABLE component_modification_list ADD CONSTRAINT component_modification_list_fk1 FOREIGN KEY (id_name_cad) REFERENCES name_cad_ref(id);
ALTER TABLE component_modification_list ADD CONSTRAINT component_modification_list_fk2 FOREIGN KEY (uuid_modification_parent) REFERENCES component_modification_list(uuid);
ALTER TABLE component_modification_list ADD CONSTRAINT component_modification_list_fk3 FOREIGN KEY (id_actual_status) REFERENCES actual_status_ref(id);

ALTER TABLE file_to_modification ADD CONSTRAINT file_to_modification_fk0 FOREIGN KEY (uuid_modification) REFERENCES component_modification_list(uuid);
ALTER TABLE file_to_modification ADD CONSTRAINT file_to_modification_fk1 FOREIGN KEY (uuid_file) REFERENCES file_ref(uuid);

ALTER TABLE param_to_modification ADD CONSTRAINT param_to_modification_fk0 FOREIGN KEY (uuid_modification) REFERENCES component_modification_list(uuid);
ALTER TABLE param_to_modification ADD CONSTRAINT param_to_modification_fk1 FOREIGN KEY (id_param) REFERENCES param_ref(id);
