-- Your SQL goes here
ALTER TABLE user_ref ADD CONSTRAINT user_ref_fk0 FOREIGN KEY (uuid_image_file) REFERENCES file_ref(uuid);
ALTER TABLE user_ref ADD CONSTRAINT user_ref_fk1 FOREIGN KEY (id_region) REFERENCES region_ref(id);
ALTER TABLE user_ref ADD CONSTRAINT user_ref_fk2 FOREIGN KEY (id_program) REFERENCES program_ref(id);

ALTER TABLE user_token_ref ADD CONSTRAINT user_token_ref_fk0 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);

ALTER TABLE user_storage_access_ref ADD CONSTRAINT user_storage_access_ref_fk0 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);

ALTER TABLE user_history_list ADD CONSTRAINT user_history_list_fk0 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);
ALTER TABLE user_history_list ADD CONSTRAINT user_history_list_fk1 FOREIGN KEY (id_type_of_change) REFERENCES type_of_change_ref(id);

ALTER TABLE user_access_to_component ADD CONSTRAINT user_access_to_component_fk0 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid) ON DELETE CASCADE;
ALTER TABLE user_access_to_component ADD CONSTRAINT user_access_to_component_fk1 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid) ON DELETE CASCADE;
ALTER TABLE user_access_to_component ADD CONSTRAINT user_access_to_component_fk2 FOREIGN KEY (id_type_access) REFERENCES type_access_ref(id) ON DELETE CASCADE;

ALTER TABLE user_access_to_standard ADD CONSTRAINT user_access_to_standard_fk0 FOREIGN KEY (uuid_standard) REFERENCES standard_ref(uuid) ON DELETE CASCADE;
ALTER TABLE user_access_to_standard ADD CONSTRAINT user_access_to_standard_fk1 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid) ON DELETE CASCADE;
ALTER TABLE user_access_to_standard ADD CONSTRAINT user_access_to_standard_fk2 FOREIGN KEY (id_type_access) REFERENCES type_access_ref(id) ON DELETE CASCADE;

ALTER TABLE user_fav ADD CONSTRAINT user_fav_fk0 FOREIGN KEY (uuid_user_favorite) REFERENCES user_ref(uuid);
ALTER TABLE user_fav ADD CONSTRAINT user_fav_fk1 FOREIGN KEY (uuid_user_follower) REFERENCES user_ref(uuid);

ALTER TABLE company_fav ADD CONSTRAINT company_fav_fk0 FOREIGN KEY (uuid_company) REFERENCES company_ref(uuid);
ALTER TABLE company_fav ADD CONSTRAINT company_fav_fk1 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);

ALTER TABLE component_fav ADD CONSTRAINT component_fav_fk0 FOREIGN KEY (uuid_component) REFERENCES component_ref(uuid);
ALTER TABLE component_fav ADD CONSTRAINT component_fav_fk1 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);

ALTER TABLE standard_fav ADD CONSTRAINT standard_fav_fk0 FOREIGN KEY (uuid_standard) REFERENCES standard_ref(uuid);
ALTER TABLE standard_fav ADD CONSTRAINT standard_fav_fk1 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);

ALTER TABLE notification_to_user ADD CONSTRAINT notification_to_user_fk0 FOREIGN KEY (id_notification) REFERENCES notification_ref(id) ON DELETE CASCADE;
ALTER TABLE notification_to_user ADD CONSTRAINT notification_to_user_fk1 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid) ON DELETE CASCADE;

ALTER TABLE user_certificate_ref ADD CONSTRAINT user_certificate_ref_fk0 FOREIGN KEY (uuid_file) REFERENCES file_ref(uuid);
ALTER TABLE user_certificate_ref ADD CONSTRAINT user_certificate_ref_fk1 FOREIGN KEY (uuid_user) REFERENCES user_ref(uuid);
