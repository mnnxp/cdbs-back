-- Your SQL goes here
INSERT INTO user_ref (uuid, email, psw_hash, psw_salt,
  firstname, lastname, secondname, username, phone, description, address,
  position, time_zone, uuid_image_file, id_region, id_program,
  is_email_verified, is_enabled, is_delete, created_at, updated_at) VALUES
  ('31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 'email@email.ru', E'\\xc3747b782c0b5c13cb1257a951d5120cd6a958f3516ba5d40c1db1c1eae99b15', '8%8lDv&TB!295%cNWDmghT5lNDSxTUxUgRY6xNw^hACP!DDDK8IKNLP)0Hr(C7m55BQDr&L%V0F^~3O&J~QPQDfJ$&uDjwUwPShyK0B4yDhXcBe^cPoV@%^gax^%z)92', 'Johm', 'Ivanov', 'Rucovich', 'usernameeee', '+79991234567', 'description for this user', 'Moscow', 'manufacturer', 3, 'bc1c2151-86d0-4656-9c9d-d016dd584297', 1, 1, 'f', 't', 'f', now(), now()),
  ('68b8281a-d19c-4d4b-88eb-6fd4a2afde1b', 'bname@somain.com', E'\\x085f06287c840b5c23578912d5e0cc2e4baf53865e9170a05ed084c1292ab7f4', 'pKFpenRqOFyutR#OAkxb%!bi%mV5q(GPKgHmwQ*bWrcuJHC3k8raBNzUnw7r%^oFKzBf%McZlVBI#O@U1@JApg@rVHEuzlybCWx&BXjrI(41x)8kR9rjURVG9lqr0EIM', 'Vans', 'Bpero', 'Nado', 'albane', 'none', 'none', 'noneadress', 'engineer', 2, 'bc1c2151-86d0-4656-9c9d-d016dd584297', 4, 1, 'f', 't', 'f', now(), now()),
  ('e97ea679-4560-4a9b-ad8b-80d2d1912602', 'testemail@testemail.ru', E'\\xc0a6617d8971cac49489078028b6ea4bd6f7929f6e542a1de5cc4c2a0f6fdfce', 'a13%A1r9kCmDHieCl(^Yt$~traAIlnTM(0#vHvjE&tZ@9Cm2OJKCKENu6&a2pTrd*Z%qQyiYXX@fG2j7XeBLx4FYY9tkSK*B^yV)0s$sQ!y)qBL#!RDxcxZLPEKD5@lg', 'testfirstname', 'testlastname', 'testsecondname', 'testingname', '+1234567890', 'testdescription', 'testaddress', 'testposition', 2, 'bc1c2151-86d0-4656-9c9d-d016dd584297', 5, 1, 'f', 't', 'f', now(), now()),
  ('c3f5f69c-bb54-45d9-bfa7-1d28cc1afa5a', 'bname@somain.com', E'\\xfdeba1b9304208da90c90ddb6fe0ee49787b085636b26f156d6240869bdb3665', 'ThmlF#HCNEX6%##AFGH(%0Tdo0w$5kh(WA9%@KwHe3mrVlhMIj~NxeiiJyh~Ty1t(J3F#lGDWFFJRZnV1&WFMA%Rl5~8qvfW)5WjL&qm%jSuFx1Uslth^a$64YkJjN)q', 'Vans', 'Bpero', 'Nado', 'testusertext', 'none', 'none', 'noneadress', 'noneposition', 6, 'bc1c2151-86d0-4656-9c9d-d016dd584297', 4, 1, 'f', 't', 'f', now(), now());

INSERT INTO user_tokens_ref (uuid_user, token, start_at, end_at) VALUES
  ('31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 'eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJpc3MiOiIwLjAuMC4wIiwic3ViIjoiMzFlY2M2ZjgtMGMwOS00YTU5LWEyZDUtMzRiNWI4MzNlNTliIiwiaWF0IjoxNjI3MjE0Njc3LCJleHAiOjE2MjczMDEwNzcsInVzZXJuYW1lIjoidXNlcm5hbWVlZWUiLCJpZF9wcm9ncmFtIjoxfQ.qZXPJTEEJRujUXRcLlrVMp5x1SW2M-2vAEqlGVtzpchIvehgWDtq7MdJLY21WOw5KApOr80MAzGzwTJTAt5XCb__sNv1ufZRqKcK6B_aBYQvq-Ph8cLF7qdQc1y6FI-vPfp5U_wtNNmtdEmp6q04gYpK6n4MP0UeIts9ILDuQgu0NGRYJqkXEyGiztIwd_Rw12mSZ3yanfT34ZQHB-HFjxPHeJ3QtxWXhuE1TcDFCmF_ca-ikrz-ygC7TiYTyOVuRQEkp3GIT7nPlUVpqtmjmAnLNL-U8ioABQj5SEsVCP5EDRPW2RSMJzmTdcArYZ4NlaWGxLWI33L3v2YNn-Kl71sGuwytYjfLQhK9F2SxydYE-C6Z1EUfDMdH2UsqcKwXQLYYPC9MTJR76JqU1SvhDMqAzj0eIIonHJi0YO5U_ZUfzsG04ocmARW8Q2cEws1tPUYxD67AlN_d1OO2Nu4yQQdAEBTH8Ef3Yq7_KANwX4aUrgLEWgOXj3qD2CpWMyarq_JFiobVbVqK549kWgqY2WALw_jyQQQ8IbszeNGpeBjePFI89PdoOXvavGrh-_y1JkW3qGiScO2D1DzzbyZA-Bw41RBJGR9DpYuy6tF4wlIyZvwKhv758Kh5MZLR0PAWC5uiegXokiYuihqTeRAS1IBqadxppIb1WfrEQJIhDW4', now(), now()+interval '1' day);

INSERT INTO user_history_list (uuid_user, id_type_of_change, old_data, changed_at) VALUES
  ('31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 1, 'Комментарий к изменению', now());

INSERT INTO component_access_to_user (uuid_component, uuid_user, id_type_access,
  is_enabled, is_delete, created_at, updated_at) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 1, 't', 'f', now(), now());

INSERT INTO standard_access_to_user (uuid_standard, uuid_user, id_type_access,
  is_enabled, is_delete, created_at, updated_at) VALUES
  ('303ec2aa-2066-42e3-93fb-de4fb9344bcb', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 1, 't', 'f', now(), now());

INSERT INTO user_fav (uuid_user_favorite, uuid_user_follower) VALUES
  ('68b8281a-d19c-4d4b-88eb-6fd4a2afde1b', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b');

INSERT INTO company_fav (uuid_company, uuid_user) VALUES
  ('2cd385e1-8f7e-4908-8235-dfe42938b46d', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b');

INSERT INTO component_fav (uuid_component, uuid_user) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b');

INSERT INTO standard_fav (uuid_standard, uuid_user) VALUES
  ('303ec2aa-2066-42e3-93fb-de4fb9344bcb', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b');

INSERT INTO notification_to_user (id_notification, uuid_user) VALUES
  (1, '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b');
