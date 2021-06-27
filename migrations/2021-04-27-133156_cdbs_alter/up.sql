-- ALTER for database cdbs
ALTER TABLE user_ref ADD CONSTRAINT user_ref_fk0 FOREIGN KEY (uuid_image_file) REFERENCES file_ref(uuid);
ALTER TABLE user_ref ADD CONSTRAINT user_ref_fk1 FOREIGN KEY (id_region) REFERENCES region_ref(id);
ALTER TABLE user_ref ADD CONSTRAINT user_ref_fk2 FOREIGN KEY (id_programm) REFERENCES programm_ref(id);

ALTER TABLE user_tokens_ref ADD CONSTRAINT user_tokens_ref_fk0 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);

ALTER TABLE company_ref ADD CONSTRAINT company_ref_fk0 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);
ALTER TABLE company_ref ADD CONSTRAINT company_ref_fk1 FOREIGN KEY (uuid_image_file) REFERENCES file_ref(uuid);
ALTER TABLE company_ref ADD CONSTRAINT company_ref_fk2 FOREIGN KEY (id_region) REFERENCES region_ref(id);
ALTER TABLE company_ref ADD CONSTRAINT company_ref_fk3 FOREIGN KEY (id_type_org) REFERENCES type_company_ref(id);

ALTER TABLE spec_ref ADD CONSTRAINT spec_ref_fk0 FOREIGN KEY (id_spec_parent) REFERENCES spec_ref(id);

ALTER TABLE spec_to_company ADD CONSTRAINT spec_to_company_fk0 FOREIGN KEY (id_spec) REFERENCES spec_ref(id);
ALTER TABLE spec_to_company ADD CONSTRAINT spec_to_company_fk1 FOREIGN KEY (uuid_company) REFERENCES company_ref(uuid);

ALTER TABLE user_history_list ADD CONSTRAINT user_history_list_fk0 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);
ALTER TABLE user_history_list ADD CONSTRAINT user_history_list_fk1 FOREIGN KEY (id_type_of_change) REFERENCES type_of_change_ref(id);

ALTER TABLE company_history_list ADD CONSTRAINT company_history_list_fk0 FOREIGN KEY (uuid_company) REFERENCES company_ref(uuid);
ALTER TABLE company_history_list ADD CONSTRAINT company_history_list_fk1 FOREIGN KEY (id_type_of_change) REFERENCES type_of_change_ref(id);

ALTER TABLE component_history_list ADD CONSTRAINT component_history_list_fk0 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid);
ALTER TABLE component_history_list ADD CONSTRAINT standard_history_list_fk1 FOREIGN KEY (id_type_of_change) REFERENCES type_of_change_ref(id);

ALTER TABLE standard_history_list ADD CONSTRAINT standard_history_list_fk0 FOREIGN KEY (uuid_standard) REFERENCES standard_ref(uuid);
ALTER TABLE standard_history_list ADD CONSTRAINT standard_history_list_fk1 FOREIGN KEY (id_type_of_change) REFERENCES type_of_change_ref(id);

ALTER TABLE file_ref ADD CONSTRAINT file_ref_fk0 FOREIGN KEY (uuid_file_parent) REFERENCES file_ref(uuid);
ALTER TABLE file_ref ADD CONSTRAINT file_ref_fk1 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);
ALTER TABLE file_ref ADD CONSTRAINT file_ref_fk2 FOREIGN KEY (id_ext) REFERENCES extension_ref(id);

ALTER TABLE extension_ref ADD CONSTRAINT extension_ref_fk0 FOREIGN KEY (id_programm) REFERENCES programm_ref(id);

ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk0 FOREIGN KEY (uuid_component_parent) REFERENCES component_ref(uuid);
ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk1 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);
ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk2 FOREIGN KEY (id_type_access) REFERENCES type_access_ref(id);
ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk3 FOREIGN KEY (id_component_type) REFERENCES component_type_ref(id);
ALTER TABLE component_ref ADD CONSTRAINT component_ref_fk4 FOREIGN KEY (id_actual_status) REFERENCES actual_status_ref(id);

ALTER TABLE spec_to_component ADD CONSTRAINT spec_to_component_fk0 FOREIGN KEY (id_spec) REFERENCES spec_ref(id);
ALTER TABLE spec_to_component ADD CONSTRAINT spec_to_component_fk1 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid);

ALTER TABLE file_to_component ADD CONSTRAINT file_to_component_fk0 FOREIGN KEY (uuid_file) REFERENCES file_ref(uuid);
ALTER TABLE file_to_component ADD CONSTRAINT file_to_component_fk1 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid);

ALTER TABLE file_to_standard ADD CONSTRAINT file_to_standard_fk0 FOREIGN KEY (uuid_file) REFERENCES file_ref(uuid);
ALTER TABLE file_to_standard ADD CONSTRAINT file_to_standard_fk1 FOREIGN KEY (uuid_standard) REFERENCES standard_ref(uuid);

ALTER TABLE component_to_keyword ADD CONSTRAINT component_to_keyword_fk0 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid);
ALTER TABLE component_to_keyword ADD CONSTRAINT component_to_keyword_fk1 FOREIGN KEY (id_component_keyword) REFERENCES component_keyword_ref(id);

ALTER TABLE discussion_component_ref ADD CONSTRAINT discussion_component_ref_fk0 FOREIGN KEY (id_discussion_parent) REFERENCES discussion_component_ref(id);
ALTER TABLE discussion_component_ref ADD CONSTRAINT discussion_component_ref_fk1 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid);
ALTER TABLE discussion_component_ref ADD CONSTRAINT discussion_component_ref_fk2 FOREIGN KEY (uuid_author) REFERENCES user_ref(uuid);

ALTER TABLE discussion_company_ref ADD CONSTRAINT discussion_company_ref_fk0 FOREIGN KEY (id_discussion_parent) REFERENCES discussion_company_ref(id);
ALTER TABLE discussion_company_ref ADD CONSTRAINT discussion_company_ref_fk1 FOREIGN KEY (uuid_company) REFERENCES company_ref(uuid);
ALTER TABLE discussion_company_ref ADD CONSTRAINT discussion_company_ref_fk2 FOREIGN KEY (uuid_author) REFERENCES user_ref(uuid);;

ALTER TABLE param_to_component ADD CONSTRAINT param_to_component_fk0 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid);
ALTER TABLE param_to_component ADD CONSTRAINT param_to_component_fk1 FOREIGN KEY (id_param) REFERENCES param_ref(id);

ALTER TABLE component_fav ADD CONSTRAINT component_fav_fk0 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid);
ALTER TABLE component_fav ADD CONSTRAINT component_fav_fk1 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);

ALTER TABLE company_fav ADD CONSTRAINT company_fav_fk0 FOREIGN KEY (uuid_company) REFERENCES company_ref(uuid);
ALTER TABLE company_fav ADD CONSTRAINT company_fav_fk1 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);

ALTER TABLE standard_fav ADD CONSTRAINT standard_fav_fk0 FOREIGN KEY (uuid_standard) REFERENCES standard_ref(uuid);
ALTER TABLE standard_fav ADD CONSTRAINT standard_fav_fk1 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);

ALTER TABLE user_fav ADD CONSTRAINT user_fav_fk0 FOREIGN KEY (uuid_user_favorite) REFERENCES user_ref(uuid);
ALTER TABLE user_fav ADD CONSTRAINT user_fav_fk1 FOREIGN KEY (uuid_user_follower) REFERENCES user_ref(uuid);

ALTER TABLE spec_translate_list ADD CONSTRAINT spec_translate_list_fk0 FOREIGN KEY (id_spec) REFERENCES spec_ref(id);
ALTER TABLE spec_translate_list ADD CONSTRAINT spec_translate_list_fk1 FOREIGN KEY (id_lang) REFERENCES language_ref(id);

ALTER TABLE param_translate_list ADD CONSTRAINT param_translate_list_fk0 FOREIGN KEY (id_param) REFERENCES param_ref(id);
ALTER TABLE param_translate_list ADD CONSTRAINT param_translate_list_fk1 FOREIGN KEY (id_lang) REFERENCES language_ref(id);

ALTER TABLE company_represent_ref ADD CONSTRAINT company_represent_ref_fk0 FOREIGN KEY (uuid_company) REFERENCES company_ref(uuid);
ALTER TABLE company_represent_ref ADD CONSTRAINT company_represent_ref_fk1 FOREIGN KEY (id_representation_type) REFERENCES representation_type_ref(id);
ALTER TABLE company_represent_ref ADD CONSTRAINT company_represent_ref_fk2 FOREIGN KEY (id_region) REFERENCES region_ref(id);

ALTER TABLE role_access ADD CONSTRAINT role_access_fk0 FOREIGN KEY (id_role) REFERENCES role_member_ref(id);
ALTER TABLE role_access ADD CONSTRAINT role_access_fk1 FOREIGN KEY (id_type_access) REFERENCES type_access_ref(id);

ALTER TABLE company_member_role ADD CONSTRAINT company_member_role_fk0 FOREIGN KEY (uuid_company) REFERENCES company_ref(uuid);
ALTER TABLE company_member_role ADD CONSTRAINT company_member_role_fk1 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);
ALTER TABLE company_member_role ADD CONSTRAINT company_member_role_fk2 FOREIGN KEY (id_role) REFERENCES role_member_ref(id);

ALTER TABLE component_access_to_user ADD CONSTRAINT component_access_to_user_fk0 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid);
ALTER TABLE component_access_to_user ADD CONSTRAINT component_access_to_user_fk1 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);
ALTER TABLE component_access_to_user ADD CONSTRAINT component_access_to_user_fk2 FOREIGN KEY (id_type_access) REFERENCES type_access_ref(id);

ALTER TABLE component_access_to_company ADD CONSTRAINT component_access_to_company_fk0 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid);
ALTER TABLE component_access_to_company ADD CONSTRAINT component_access_to_company_fk1 FOREIGN KEY (uuid_company) REFERENCES company_ref(uuid);
ALTER TABLE component_access_to_company ADD CONSTRAINT component_access_to_company_fk2 FOREIGN KEY (id_type_access) REFERENCES type_access_ref(id);

ALTER TABLE standard_access_to_user ADD CONSTRAINT standard_access_to_user_fk0 FOREIGN KEY (uuid_standard) REFERENCES standard_ref(uuid);
ALTER TABLE standard_access_to_user ADD CONSTRAINT standard_access_to_user_fk1 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);
ALTER TABLE standard_access_to_user ADD CONSTRAINT standard_access_to_user_fk2 FOREIGN KEY (id_type_access) REFERENCES type_access_ref(id);

ALTER TABLE standard_access_to_company ADD CONSTRAINT standard_access_to_company_fk0 FOREIGN KEY (uuid_standard) REFERENCES standard_ref(uuid);
ALTER TABLE standard_access_to_company ADD CONSTRAINT standard_access_to_company_fk1 FOREIGN KEY (uuid_company) REFERENCES company_ref(uuid);
ALTER TABLE standard_access_to_company ADD CONSTRAINT standard_access_to_company_fk2 FOREIGN KEY (id_type_access) REFERENCES type_access_ref(id);

ALTER TABLE supplier_to_component ADD CONSTRAINT supplier_to_component_fk0 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid);
ALTER TABLE supplier_to_component ADD CONSTRAINT supplier_to_component_fk1 FOREIGN KEY (uuid_company) REFERENCES company_ref(uuid);

ALTER TABLE component_modification_list ADD CONSTRAINT component_modification_list_fk0 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid);
ALTER TABLE component_modification_list ADD CONSTRAINT component_modification_list_fk1 FOREIGN KEY (uuid_modification_parent) REFERENCES component_modification_list(uuid);
ALTER TABLE component_modification_list ADD CONSTRAINT component_modification_list_fk2 FOREIGN KEY (id_actual_status) REFERENCES actual_status_ref(id);

ALTER TABLE file_to_modification ADD CONSTRAINT file_to_modification_fk0 FOREIGN KEY (uuid_modification) REFERENCES component_modification_list(uuid);
ALTER TABLE file_to_modification ADD CONSTRAINT file_to_modification_fk1 FOREIGN KEY (uuid_file) REFERENCES file_ref(uuid);

ALTER TABLE set_file_to_programm ADD CONSTRAINT set_file_to_programm_fk0 FOREIGN KEY (uuid_modification) REFERENCES component_modification_list(uuid);
ALTER TABLE set_file_to_programm ADD CONSTRAINT set_file_to_programm_fk1 FOREIGN KEY (uuid_file) REFERENCES file_ref(uuid);
ALTER TABLE set_file_to_programm ADD CONSTRAINT set_file_to_programm_fk2 FOREIGN KEY (id_programm) REFERENCES programm_ref(id);

ALTER TABLE param_to_modification ADD CONSTRAINT param_to_modification_fk0 FOREIGN KEY (uuid_modification) REFERENCES component_modification_list(uuid);
ALTER TABLE param_to_modification ADD CONSTRAINT param_to_modification_fk1 FOREIGN KEY (id_param) REFERENCES param_ref(id);

ALTER TABLE standard_ref ADD CONSTRAINT standard_ref_fk0 FOREIGN KEY (uuid_standard_parent) REFERENCES standard_ref(uuid);
ALTER TABLE standard_ref ADD CONSTRAINT standard_ref_fk1 FOREIGN KEY (uuid_image_file) REFERENCES file_ref(uuid);
ALTER TABLE standard_ref ADD CONSTRAINT standard_ref_fk2 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);
ALTER TABLE standard_ref ADD CONSTRAINT standard_ref_fk3 FOREIGN KEY (uuid_company) REFERENCES company_ref(uuid);
ALTER TABLE standard_ref ADD CONSTRAINT standard_ref_fk4 FOREIGN KEY (id_standard_status) REFERENCES standard_status_ref(id);
ALTER TABLE standard_ref ADD CONSTRAINT standard_ref_fk5 FOREIGN KEY (id_type_access) REFERENCES type_access_ref(id);
ALTER TABLE standard_ref ADD CONSTRAINT standard_ref_fk6 FOREIGN KEY (id_region) REFERENCES region_ref(id);
