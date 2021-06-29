-- Your SQL goes here
ALTER TABLE company_ref ADD CONSTRAINT company_ref_fk0 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);
ALTER TABLE company_ref ADD CONSTRAINT company_ref_fk1 FOREIGN KEY (uuid_image_file) REFERENCES file_ref(uuid);
ALTER TABLE company_ref ADD CONSTRAINT company_ref_fk2 FOREIGN KEY (id_region) REFERENCES region_ref(id);
ALTER TABLE company_ref ADD CONSTRAINT company_ref_fk3 FOREIGN KEY (id_type_org) REFERENCES type_company_ref(id);

ALTER TABLE company_represent_ref ADD CONSTRAINT company_represent_ref_fk0 FOREIGN KEY (uuid_company) REFERENCES company_ref(uuid);
ALTER TABLE company_represent_ref ADD CONSTRAINT company_represent_ref_fk1 FOREIGN KEY (id_representation_type) REFERENCES representation_type_ref(id);
ALTER TABLE company_represent_ref ADD CONSTRAINT company_represent_ref_fk2 FOREIGN KEY (id_region) REFERENCES region_ref(id);

ALTER TABLE company_member_role ADD CONSTRAINT company_member_role_fk0 FOREIGN KEY (uuid_company) REFERENCES company_ref(uuid);
ALTER TABLE company_member_role ADD CONSTRAINT company_member_role_fk1 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);
ALTER TABLE company_member_role ADD CONSTRAINT company_member_role_fk2 FOREIGN KEY (id_role) REFERENCES role_member_ref(id);

ALTER TABLE spec_to_company ADD CONSTRAINT spec_to_company_fk0 FOREIGN KEY (id_spec) REFERENCES spec_ref(id);
ALTER TABLE spec_to_company ADD CONSTRAINT spec_to_company_fk1 FOREIGN KEY (uuid_company) REFERENCES company_ref(uuid);

ALTER TABLE company_history_list ADD CONSTRAINT company_history_list_fk0 FOREIGN KEY (uuid_company) REFERENCES company_ref(uuid);
ALTER TABLE company_history_list ADD CONSTRAINT company_history_list_fk1 FOREIGN KEY (id_type_of_change) REFERENCES type_of_change_ref(id);

ALTER TABLE discussion_company_ref ADD CONSTRAINT discussion_company_ref_fk0 FOREIGN KEY (id_discussion_parent) REFERENCES discussion_company_ref(id);
ALTER TABLE discussion_company_ref ADD CONSTRAINT discussion_company_ref_fk1 FOREIGN KEY (uuid_company) REFERENCES company_ref(uuid);
ALTER TABLE discussion_company_ref ADD CONSTRAINT discussion_company_ref_fk2 FOREIGN KEY (uuid_author) REFERENCES user_ref(uuid);;

ALTER TABLE role_access ADD CONSTRAINT role_access_fk0 FOREIGN KEY (id_role) REFERENCES role_member_ref(id);
ALTER TABLE role_access ADD CONSTRAINT role_access_fk1 FOREIGN KEY (id_type_access) REFERENCES type_access_ref(id);

ALTER TABLE component_access_to_company ADD CONSTRAINT component_access_to_company_fk0 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid);
ALTER TABLE component_access_to_company ADD CONSTRAINT component_access_to_company_fk1 FOREIGN KEY (uuid_company) REFERENCES company_ref(uuid);
ALTER TABLE component_access_to_company ADD CONSTRAINT component_access_to_company_fk2 FOREIGN KEY (id_type_access) REFERENCES type_access_ref(id);

ALTER TABLE standard_access_to_company ADD CONSTRAINT standard_access_to_company_fk0 FOREIGN KEY (uuid_standard) REFERENCES standard_ref(uuid);
ALTER TABLE standard_access_to_company ADD CONSTRAINT standard_access_to_company_fk1 FOREIGN KEY (uuid_company) REFERENCES company_ref(uuid);
ALTER TABLE standard_access_to_company ADD CONSTRAINT standard_access_to_company_fk2 FOREIGN KEY (id_type_access) REFERENCES type_access_ref(id);
