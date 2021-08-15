-- Your SQL goes here
ALTER TABLE company_ref ADD CONSTRAINT company_ref_fk0 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid) ON DELETE CASCADE;
ALTER TABLE company_ref ADD CONSTRAINT company_ref_fk1 FOREIGN KEY (uuid_image_file) REFERENCES file_ref(uuid);
ALTER TABLE company_ref ADD CONSTRAINT company_ref_fk2 FOREIGN KEY (id_region) REFERENCES region_ref(id);
ALTER TABLE company_ref ADD CONSTRAINT company_ref_fk3 FOREIGN KEY (id_type_org) REFERENCES type_company_ref(id);

ALTER TABLE company_represent_ref ADD CONSTRAINT company_represent_ref_fk0 FOREIGN KEY (uuid_company) REFERENCES company_ref(uuid);
ALTER TABLE company_represent_ref ADD CONSTRAINT company_represent_ref_fk1 FOREIGN KEY (id_representation_type) REFERENCES representation_type_ref(id);
ALTER TABLE company_represent_ref ADD CONSTRAINT company_represent_ref_fk2 FOREIGN KEY (id_region) REFERENCES region_ref(id);

ALTER TABLE company_member_role ADD CONSTRAINT company_member_role_fk0 FOREIGN KEY (uuid_company) REFERENCES company_ref(uuid);
ALTER TABLE company_member_role ADD CONSTRAINT company_member_role_fk1 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);
ALTER TABLE company_member_role ADD CONSTRAINT company_member_role_fk2 FOREIGN KEY (id_role) REFERENCES role_member_ref(id);

ALTER TABLE spec_to_company ADD CONSTRAINT spec_to_company_fk0 FOREIGN KEY (id_spec) REFERENCES spec_ref(id) ON DELETE CASCADE;
ALTER TABLE spec_to_company ADD CONSTRAINT spec_to_company_fk1 FOREIGN KEY (uuid_company) REFERENCES company_ref(uuid) ON DELETE CASCADE;

ALTER TABLE company_history_list ADD CONSTRAINT company_history_list_fk0 FOREIGN KEY (uuid_company) REFERENCES company_ref(uuid);
ALTER TABLE company_history_list ADD CONSTRAINT company_history_list_fk1 FOREIGN KEY (id_type_of_change) REFERENCES type_of_change_ref(id);

ALTER TABLE discussion_company_ref ADD CONSTRAINT discussion_company_ref_fk0 FOREIGN KEY (id_discussion_parent) REFERENCES discussion_company_ref(id);
ALTER TABLE discussion_company_ref ADD CONSTRAINT discussion_company_ref_fk1 FOREIGN KEY (uuid_company) REFERENCES company_ref(uuid);
ALTER TABLE discussion_company_ref ADD CONSTRAINT discussion_company_ref_fk2 FOREIGN KEY (uuid_author) REFERENCES user_ref(uuid);;

ALTER TABLE role_access ADD CONSTRAINT role_access_fk0 FOREIGN KEY (id_role) REFERENCES role_member_ref(id);
ALTER TABLE role_access ADD CONSTRAINT role_access_fk1 FOREIGN KEY (id_type_access) REFERENCES type_access_ref(id);

ALTER TABLE company_access_to_component ADD CONSTRAINT company_access_to_component_fk0 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid) ON DELETE CASCADE;
ALTER TABLE company_access_to_component ADD CONSTRAINT company_access_to_component_fk1 FOREIGN KEY (uuid_company) REFERENCES company_ref(uuid) ON DELETE CASCADE;
ALTER TABLE company_access_to_component ADD CONSTRAINT company_access_to_component_fk2 FOREIGN KEY (id_type_access) REFERENCES type_access_ref(id) ON DELETE CASCADE;

ALTER TABLE company_access_to_standard ADD CONSTRAINT company_access_to_standard_fk0 FOREIGN KEY (uuid_standard) REFERENCES standard_ref(uuid) ON DELETE CASCADE;
ALTER TABLE company_access_to_standard ADD CONSTRAINT company_access_to_standard_fk1 FOREIGN KEY (uuid_company) REFERENCES company_ref(uuid) ON DELETE CASCADE;
ALTER TABLE company_access_to_standard ADD CONSTRAINT company_access_to_standard_fk2 FOREIGN KEY (id_type_access) REFERENCES type_access_ref(id) ON DELETE CASCADE;

ALTER TABLE company_certificate_ref ADD CONSTRAINT company_certificate_ref_fk0 FOREIGN KEY (uuid_file) REFERENCES file_ref(uuid);
ALTER TABLE company_certificate_ref ADD CONSTRAINT company_certificate_ref_fk1 FOREIGN KEY (uuid_company) REFERENCES company_ref(uuid);

ALTER TABLE type_company_translate_list ADD CONSTRAINT type_company_translate_list_fk0 FOREIGN KEY (id_type_company) REFERENCES type_company_ref(id);
ALTER TABLE type_company_translate_list ADD CONSTRAINT type_company_translate_list_fk1 FOREIGN KEY (id_lang) REFERENCES language_ref(id);

ALTER TABLE representation_type_translate_list ADD CONSTRAINT representation_type_translate_list_fk0 FOREIGN KEY (id_representation_type) REFERENCES representation_type_ref(id);
ALTER TABLE representation_type_translate_list ADD CONSTRAINT representation_type_translate_list_fk1 FOREIGN KEY (id_lang) REFERENCES language_ref(id);

ALTER TABLE role_member_translate_list ADD CONSTRAINT role_member_translate_list_fk0 FOREIGN KEY (id_role_member) REFERENCES role_member_ref(id);
ALTER TABLE role_member_translate_list ADD CONSTRAINT role_member_translate_list_fk1 FOREIGN KEY (id_lang) REFERENCES language_ref(id);
