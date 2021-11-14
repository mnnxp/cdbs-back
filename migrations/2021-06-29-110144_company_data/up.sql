-- Your SQL goes here
INSERT INTO company_ref (uuid, orgname, shortname, inn, phone,
  email, description, address, site_url, time_zone, user_uuid,
  image_file_uuid, region_id, company_type_id, type_access_id, is_supplier, is_email_verified,
  is_enabled, is_delete, created_at, updated_at) VALUES
  ('2cd385e1-8f7e-4908-8235-dfe42938b46d', 'romashka', 'rom-ka', '12345678910', '+79991234567', 'email@email.ru', 'description for this company', 'Moscow', 'https://cadbase.ru', 'Europe/Moscow', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 'bc1c2151-86d0-4656-9c9d-d016dd584297', 1, 1, 3, 't', 't', 't', 'f', now(), now()),
  ('e97ea679-4560-4a9b-ad8b-80d2d191235e', 'testorgname', 'testshortname', 'testinn', '+1234567890', 'testemail@testemail.ru', 'testdescription', 'testaddress', 'testsiteUrl', 'Europe/Moscow', '68b8281a-d19c-4d4b-88eb-6fd4a2afde1b', 'bc1c2151-86d0-4656-9c9d-d016dd584297', 13, 5, 1, 'f', 'f', 't', 'f', now(), now());

INSERT INTO company_type_ref (id) VALUES
  (1),
  (2),
  (3),
  (4),
  (5),
  (6),
  (7),
  (8),
  (9),
  (10);

INSERT INTO company_type_translate_list (company_type_id, lang_id, name, shortname) VALUES
  (1, 1, 'Individual entrepreneur', 'IP'),
  (2, 1, 'Joint Stock Companies', 'JSC'),
  (3, 1, 'Public Joint Stock Companies', 'PJSC'),
  (4, 1, 'Non-public joint stock companies', 'NAO'),
  (5, 1, 'Limited Liability Company', 'LLC'),
  (6, 1, 'Business partnerships', 'H.part-va'),
  (7, 1, 'Production cooperatives (artels)', 'Artel'),
  (8, 1, 'Agricultural production cooperatives', 'Agricultural'),
  (9, 1, 'Cooperative farms (cooperative farms)', 'Cooperative farms'),
  (10, 1, 'Other legal entities that are commercial organizations', 'Others'),
  (1, 2, 'Индивидуальный предприниматель', 'ИП'),
  (2, 2, 'Акционерные общества', 'АО'),
  (3, 2, 'Публичные акционерные общества', 'ПАО'),
  (4, 2, 'Непубличные акционерные общества', 'НАО'),
  (5, 2, 'Общества с ограниченной ответственностью', 'ООО'),
  (6, 2, 'Хозяйственные партнерства', 'Х.парт-ва'),
  (7, 2, 'Производственные кооперативы (артели)', 'Артель'),
  (8, 2, 'Сельскохозяйственные производственные кооперативы', 'Сельхоз'),
  (9, 2, 'Кооперативные хозяйства (коопхозы)', 'Коопхоз'),
  (10, 2, 'Прочие юридические лица, являющиеся коммерческими организациями', 'Прочие');

INSERT INTO company_represent_ref (uuid, company_uuid, region_id, representation_type_id, name, address, phone) VALUES
  ('22a08149-b63a-4f65-a1cd-4a28f85567ec', 'e97ea679-4560-4a9b-ad8b-80d2d191235e', 1, 1, 'Местный офис', 'г. Москва', '+79991234567'),
  ('297e44b3-d36c-4ab5-be16-e2a955aabf13', 'e97ea679-4560-4a9b-ad8b-80d2d191235e', 2, 1, 'additional office', 'Batkov District, Minsk', '+375548418789'),
  ('96df6359-a31e-40ad-aa06-9355abc2cc58', 'e97ea679-4560-4a9b-ad8b-80d2d191235e', 3, 1, 'additional office', 'None str, Kiev', '+380874487556');

INSERT INTO company_member_list (company_uuid, user_uuid, role_id, is_enabled, created_at, updated_at) VALUES
  ('2cd385e1-8f7e-4908-8235-dfe42938b46d', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 5, 't', now(), now()),
  ('2cd385e1-8f7e-4908-8235-dfe42938b46d', 'c3f5f69c-bb54-45d9-bfa7-1d28cc1afa5a', 1, 't', now(), now()),
  ('2cd385e1-8f7e-4908-8235-dfe42938b46d', '68b8281a-d19c-4d4b-88eb-6fd4a2afde1b', 3, 't', now(), now()),
  ('e97ea679-4560-4a9b-ad8b-80d2d191235e', '68b8281a-d19c-4d4b-88eb-6fd4a2afde1b', 1, 't', now(), now()),
  ('e97ea679-4560-4a9b-ad8b-80d2d191235e', 'e97ea679-4560-4a9b-ad8b-80d2d1912602', 4, 't', now(), now());

INSERT INTO spec_to_company (spec_id, company_uuid) VALUES
  (2, '2cd385e1-8f7e-4908-8235-dfe42938b46d'),
  (3, 'e97ea679-4560-4a9b-ad8b-80d2d191235e');

INSERT INTO company_history_list (company_uuid, type_of_change_id, old_data, changed_at) VALUES
  ('2cd385e1-8f7e-4908-8235-dfe42938b46d', 1, 'Комментарий к изменению', now());

INSERT INTO discussion_company_ref (parent_discussion_id, company_uuid,
author_uuid, message_content, is_delete, created_at, updated_at) VALUES
  (1, '2cd385e1-8f7e-4908-8235-dfe42938b46d', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 'this message', 'f', now(), now());

INSERT INTO representation_type_ref (id) VALUES
  (1),
  (2);

INSERT INTO representation_type_translate_list (representation_type_id, lang_id, representation_type) VALUES
  (1, 1, 'Office'),
  (2, 1, 'Branch');

INSERT INTO role_member_list (id, company_uuid) VALUES
  (1,'2cd385e1-8f7e-4908-8235-dfe42938b46d'),
  (2,'2cd385e1-8f7e-4908-8235-dfe42938b46d'),
  (3,'e97ea679-4560-4a9b-ad8b-80d2d191235e'),
  (4,'e97ea679-4560-4a9b-ad8b-80d2d191235e'),
  (5,'e97ea679-4560-4a9b-ad8b-80d2d191235e');

INSERT INTO role_member_translate_list (role_member_id, lang_id, name) VALUES
  (1, 1, 'Trainee'),
  (2, 1, 'Designer'),
  (3, 1, 'Engineer'),
  (4, 1, 'Constructor'),
  (5, 1, 'Architect');

INSERT INTO role_access (role_id, type_access_id) VALUES
  (1, 3),
  (2, 2),
  (3, 1),
  (4, 1),
  (5, 1);

INSERT INTO company_access_to_component (component_uuid, company_uuid, type_access_id,
is_enabled, created_at, updated_at) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', '2cd385e1-8f7e-4908-8235-dfe42938b46d', 1, 't', now(), now());

INSERT INTO company_access_to_standard (standard_uuid, company_uuid, type_access_id,
  is_enabled, created_at, updated_at) VALUES
  ('303ec2aa-2066-42e3-93fb-de4fb9344bcb', '2cd385e1-8f7e-4908-8235-dfe42938b46d', 1, 't', now(), now());

INSERT INTO company_certificate_ref (file_uuid, company_uuid, description) VALUES
  ('9a227a5d-c54e-496a-a1ef-c5d49d8bd0a2', '2cd385e1-8f7e-4908-8235-dfe42938b46d', 'company certificate');
