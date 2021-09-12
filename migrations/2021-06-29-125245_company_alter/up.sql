-- Your SQL goes here
ALTER TABLE company_ref ADD CONSTRAINT company_ref_fk0 FOREIGN KEY (user_uuid) REFERENCES user_ref(uuid) ON DELETE CASCADE;
ALTER TABLE company_ref ADD CONSTRAINT company_ref_fk1 FOREIGN KEY (image_file_uuid) REFERENCES file_ref(uuid) ON DELETE CASCADE;
ALTER TABLE company_ref ADD CONSTRAINT company_ref_fk2 FOREIGN KEY (region_id) REFERENCES region_ref(id) ON DELETE CASCADE;
ALTER TABLE company_ref ADD CONSTRAINT company_ref_fk3 FOREIGN KEY (company_type_id) REFERENCES company_type_ref(id) ON DELETE CASCADE;
ALTER TABLE company_ref ADD CONSTRAINT company_ref_fk4 FOREIGN KEY (type_access_id) REFERENCES type_access_ref(id) ON DELETE CASCADE;

ALTER TABLE company_represent_ref ADD CONSTRAINT company_represent_ref_fk0 FOREIGN KEY (company_uuid) REFERENCES company_ref(uuid) ON DELETE CASCADE;
ALTER TABLE company_represent_ref ADD CONSTRAINT company_represent_ref_fk1 FOREIGN KEY (representation_type_id) REFERENCES representation_type_ref(id) ON DELETE CASCADE;
ALTER TABLE company_represent_ref ADD CONSTRAINT company_represent_ref_fk2 FOREIGN KEY (region_id) REFERENCES region_ref(id) ON DELETE CASCADE;

ALTER TABLE company_member_role ADD CONSTRAINT company_member_role_fk0 FOREIGN KEY (company_uuid) REFERENCES company_ref(uuid) ON DELETE CASCADE;
ALTER TABLE company_member_role ADD CONSTRAINT company_member_role_fk1 FOREIGN KEY (user_uuid) REFERENCES user_ref(uuid) ON DELETE CASCADE;
ALTER TABLE company_member_role ADD CONSTRAINT company_member_role_fk2 FOREIGN KEY (role_id) REFERENCES role_member_ref(id) ON DELETE CASCADE;

ALTER TABLE spec_to_company ADD CONSTRAINT spec_to_company_fk0 FOREIGN KEY (spec_id) REFERENCES spec_ref(id) ON DELETE CASCADE;
ALTER TABLE spec_to_company ADD CONSTRAINT spec_to_company_fk1 FOREIGN KEY (company_uuid) REFERENCES company_ref(uuid) ON DELETE CASCADE;

ALTER TABLE company_history_list ADD CONSTRAINT company_history_list_fk0 FOREIGN KEY (company_uuid) REFERENCES company_ref(uuid) ON DELETE CASCADE;
ALTER TABLE company_history_list ADD CONSTRAINT company_history_list_fk1 FOREIGN KEY (type_of_change_id) REFERENCES type_of_change_ref(id) ON DELETE CASCADE;

ALTER TABLE discussion_company_ref ADD CONSTRAINT discussion_company_ref_fk0 FOREIGN KEY (parent_discussion_id) REFERENCES discussion_company_ref(id) ON DELETE CASCADE;
ALTER TABLE discussion_company_ref ADD CONSTRAINT discussion_company_ref_fk1 FOREIGN KEY (company_uuid) REFERENCES company_ref(uuid) ON DELETE CASCADE;
ALTER TABLE discussion_company_ref ADD CONSTRAINT discussion_company_ref_fk2 FOREIGN KEY (author_uuid) REFERENCES user_ref(uuid) ON DELETE CASCADE;;

ALTER TABLE role_access ADD CONSTRAINT role_access_fk0 FOREIGN KEY (role_id) REFERENCES role_member_ref(id) ON DELETE CASCADE;
ALTER TABLE role_access ADD CONSTRAINT role_access_fk1 FOREIGN KEY (type_access_id) REFERENCES type_access_ref(id) ON DELETE CASCADE;

ALTER TABLE company_access_to_component ADD CONSTRAINT company_access_to_component_fk0 FOREIGN KEY (component_uuid) REFERENCES component_ref(uuid) ON DELETE CASCADE;
ALTER TABLE company_access_to_component ADD CONSTRAINT company_access_to_component_fk1 FOREIGN KEY (company_uuid) REFERENCES company_ref(uuid) ON DELETE CASCADE;
ALTER TABLE company_access_to_component ADD CONSTRAINT company_access_to_component_fk2 FOREIGN KEY (type_access_id) REFERENCES type_access_ref(id) ON DELETE CASCADE;

ALTER TABLE company_access_to_standard ADD CONSTRAINT company_access_to_standard_fk0 FOREIGN KEY (standard_uuid) REFERENCES standard_ref(uuid) ON DELETE CASCADE;
ALTER TABLE company_access_to_standard ADD CONSTRAINT company_access_to_standard_fk1 FOREIGN KEY (company_uuid) REFERENCES company_ref(uuid) ON DELETE CASCADE;
ALTER TABLE company_access_to_standard ADD CONSTRAINT company_access_to_standard_fk2 FOREIGN KEY (type_access_id) REFERENCES type_access_ref(id) ON DELETE CASCADE;

ALTER TABLE company_certificate_ref ADD CONSTRAINT company_certificate_ref_fk0 FOREIGN KEY (file_uuid) REFERENCES file_ref(uuid) ON DELETE CASCADE;
ALTER TABLE company_certificate_ref ADD CONSTRAINT company_certificate_ref_fk1 FOREIGN KEY (company_uuid) REFERENCES company_ref(uuid) ON DELETE CASCADE;

ALTER TABLE company_type_translate_list ADD CONSTRAINT company_type_translate_list_fk0 FOREIGN KEY (company_type_id) REFERENCES company_type_ref(id) ON DELETE CASCADE;
ALTER TABLE company_type_translate_list ADD CONSTRAINT company_type_translate_list_fk1 FOREIGN KEY (lang_id) REFERENCES language_ref(id) ON DELETE CASCADE;

ALTER TABLE representation_type_translate_list ADD CONSTRAINT representation_type_translate_list_fk0 FOREIGN KEY (representation_type_id) REFERENCES representation_type_ref(id) ON DELETE CASCADE;
ALTER TABLE representation_type_translate_list ADD CONSTRAINT representation_type_translate_list_fk1 FOREIGN KEY (lang_id) REFERENCES language_ref(id) ON DELETE CASCADE;

ALTER TABLE role_member_translate_list ADD CONSTRAINT role_member_translate_list_fk0 FOREIGN KEY (role_member_id) REFERENCES role_member_ref(id) ON DELETE CASCADE;
ALTER TABLE role_member_translate_list ADD CONSTRAINT role_member_translate_list_fk1 FOREIGN KEY (lang_id) REFERENCES language_ref(id) ON DELETE CASCADE;
