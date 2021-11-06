-- Your SQL goes here
INSERT INTO user_ref (uuid, email, psw_hash, psw_salt,
  firstname, lastname, secondname, username, phone, description, address,
  position, time_zone, image_file_uuid, region_id, program_id, type_access_id,
  is_email_verified, is_enabled, is_delete, created_at, updated_at) VALUES
  ('31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 'email@email.ru', E'\\xc3747b782c0b5c13cb1257a951d5120cd6a958f3516ba5d40c1db1c1eae99b15', E'\\xc3747b782c0b5c13cb1257a951d5120cd6a958f3516ba5d40c1db1c1eae99b15', 'Johm', 'Ivanov', 'Rucovich', 'usernameeee', '+79991234567', 'description for this user', 'Moscow', 'manufacturer', 'Europe/Moscow', 'bc1c2151-86d0-4656-9c9d-d016dd584297', 1, 1, 1, 'f', 't', 'f', now(), now()),
  ('68b8281a-d19c-4d4b-88eb-6fd4a2afde1b', 'test@test.com', E'\\x085f06287c840b5c23578912d5e0cc2e4baf53865e9170a05ed084c1292ab7f4', E'\\xc3747b782c0b5c13cb1257a951d5120cd6a958f3516ba5d40c1db1c1eae99b15', 'Vans', 'Bpero', 'Nado', 'albane', 'none', 'none', 'noneadress', 'engineer', 'Europe/Moscow', 'bc1c2151-86d0-4656-9c9d-d016dd584297', 4, 1, 1, 'f', 't', 'f', now(), now()),
  ('e97ea679-4560-4a9b-ad8b-80d2d1912602', 'testemail@testemail.ru', E'\\xc0a6617d8971cac49489078028b6ea4bd6f7929f6e542a1de5cc4c2a0f6fdfce', E'\\xc3747b782c0b5c13cb1257a951d5120cd6a958f3516ba5d40c1db1c1eae99b15', 'testfirstname', 'testlastname', 'testsecondname', 'testingname', '+1234567890', 'testdescription', 'testaddress', 'testposition', 'Europe/Moscow', 'bc1c2151-86d0-4656-9c9d-d016dd584297', 5, 1, 1, 'f', 't', 'f', now(), now()),
  ('c3f5f69c-bb54-45d9-bfa7-1d28cc1afa5a', 'bname@somain.com', E'\\xfdeba1b9304208da90c90ddb6fe0ee49787b085636b26f156d6240869bdb3665', E'\\xc3747b782c0b5c13cb1257a951d5120cd6a958f3516ba5d40c1db1c1eae99b15', 'Xans', 'Gpero', 'Bado', 'testusertext', 'none', 'none', 'noneadress', 'noneposition', 'Europe/Moscow', 'bc1c2151-86d0-4656-9c9d-d016dd584297', 4, 1, 1, 'f', 't', 'f', now(), now());

INSERT INTO user_token_ref (user_uuid, token, created_at, expiration_at) VALUES
  ('31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 'eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJpc3MiOiIwLjAuMC4wIiwic3ViIjoiMzFlY2M2ZjgtMGMwOS00YTU5LWEyZDUtMzRiNWI4MzNlNTliIiwiaWF0IjoxNjI3MjE0Njc3LCJleHAiOjE2MjczMDEwNzcsInVzZXJuYW1lIjoidXNlcm5hbWVlZWUiLCJpZF9wcm9ncmFtIjoxfQ.qZXPJTEEJRujUXRcLlrVMp5x1SW2M-2vAEqlGVtzpchIvehgWDtq7MdJLY21WOw5KApOr80MAzGzwTJTAt5XCb__sNv1ufZRqKcK6B_aBYQvq-Ph8cLF7qdQc1y6FI-vPfp5U_wtNNmtdEmp6q04gYpK6n4MP0UeIts9ILDuQgu0NGRYJqkXEyGiztIwd_Rw12mSZ3yanfT34ZQHB-HFjxPHeJ3QtxWXhuE1TcDFCmF_ca-ikrz-ygC7TiYTyOVuRQEkp3GIT7nPlUVpqtmjmAnLNL-U8ioABQj5SEsVCP5EDRPW2RSMJzmTdcArYZ4NlaWGxLWI33L3v2YNn-Kl71sGuwytYjfLQhK9F2SxydYE-C6Z1EUfDMdH2UsqcKwXQLYYPC9MTJR76JqU1SvhDMqAzj0eIIonHJi0YO5U_ZUfzsG04ocmARW8Q2cEws1tPUYxD67AlN_d1OO2Nu4yQQdAEBTH8Ef3Yq7_KANwX4aUrgLEWgOXj3qD2CpWMyarq_JFiobVbVqK549kWgqY2WALw_jyQQQ8IbszeNGpeBjePFI89PdoOXvavGrh-_y1JkW3qGiScO2D1DzzbyZA-Bw41RBJGR9DpYuy6tF4wlIyZvwKhv758Kh5MZLR0PAWC5uiegXokiYuihqTeRAS1IBqadxppIb1WfrEQJIhDW4', now(), now()+interval '1' day);

INSERT INTO user_history_list (user_uuid, type_of_change_id, old_data, changed_at) VALUES
  ('31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 1, 'Комментарий к изменению', now());

INSERT INTO user_access_to_component (component_uuid, user_uuid, type_access_id,
  is_enabled, created_at, updated_at) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 1, 't', now(), now());

INSERT INTO user_access_to_standard (standard_uuid, user_uuid, type_access_id,
  is_enabled, created_at, updated_at) VALUES
  ('303ec2aa-2066-42e3-93fb-de4fb9344bcb', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 1, 't', now(), now());

INSERT INTO user_fav (user_favorite_uuid, user_follower_uuid) VALUES
  ('68b8281a-d19c-4d4b-88eb-6fd4a2afde1b', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b');

INSERT INTO company_fav (company_uuid, user_uuid) VALUES
  ('2cd385e1-8f7e-4908-8235-dfe42938b46d', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b');

INSERT INTO component_fav (component_uuid, user_uuid) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b');

INSERT INTO standard_fav (standard_uuid, user_uuid) VALUES
  ('303ec2aa-2066-42e3-93fb-de4fb9344bcb', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b');

INSERT INTO notification_to_user (notification_id, user_uuid, is_read) VALUES
  (1, '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 'f');

INSERT INTO user_certificate_ref (file_uuid, user_uuid, description) VALUES
  ('ae496786-33f9-4727-a0fb-9c6702a3f30a', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 'user certificate');
