-- Your SQL goes here
INSERT INTO company_ref (uuid, orgname, shortname, inn, phone,
  email, description, address, site_url, time_zone, uuid_user,
  uuid_image_file, id_region, id_type_org, is_supplier, is_email_verified,
  is_enabled, is_delete, created_at, updated_at) VALUES
  ('2cd385e1-8f7e-4908-8235-dfe42938b46d', 'romashka', 'rom-ka', '12345678910', '+79991234567', 'email@email.ru', 'description for this company', 'Moscow', 'https://cadbase.ru', 3, '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 'bc1c2151-86d0-4656-9c9d-d016dd584297', 1, 1, 't', 't', 't', 'f', now(), now()),
  ('e97ea679-4560-4a9b-ad8b-80d2d191235e', 'testorgname', 'testshortname', 'testinn', '+1234567890', 'testemail@testemail.ru', 'testdescription', 'testaddress', 'testsiteUrl', 2, '68b8281a-d19c-4d4b-88eb-6fd4a2afde1b', 'bc1c2151-86d0-4656-9c9d-d016dd584297', 13, 5, 'f', 'f', 't', 'f', now(), now());

INSERT INTO type_company_ref (name, shortname) VALUES
  ('Индивидуальный предприниматель', 'ИП'),
  ('Акционерные общества', 'АО'),
  ('Публичные акционерные общества', 'ПАО'),
  ('Непубличные акционерные общества', 'НАО'),
  ('Общества с ограниченной ответственностью', 'ООО'),
  ('Хозяйственные партнерства', 'Х.парт-ва'),
  ('Производственные кооперативы (артели)', 'Артель'),
  ('Сельскохозяйственные производственные кооперативы', 'Сельхоз'),
  ('Кооперативные хозяйства (коопхозы)', 'Коопхоз'),
  ('Прочие юридические лица, являющиеся коммерческими организациями', 'Прочие');

INSERT INTO company_represent_ref (uuid, uuid_company, id_region, id_representation_type, name, address, phone) VALUES
  ('22a08149-b63a-4f65-a1cd-4a28f85567ec', 'e97ea679-4560-4a9b-ad8b-80d2d191235e', 1, 1, 'Местный офис', 'г. Москва', '+79991234567'),
  ('297e44b3-d36c-4ab5-be16-e2a955aabf13', 'e97ea679-4560-4a9b-ad8b-80d2d191235e', 2, 1, 'additional office', 'Batkov District, Minsk', '+375548418789'),
  ('96df6359-a31e-40ad-aa06-9355abc2cc58', 'e97ea679-4560-4a9b-ad8b-80d2d191235e', 3, 1, 'additional office', 'None str, Kiev', '+380874487556');

INSERT INTO company_member_role (uuid_company, uuid_user, id_role, is_enabled, created_at, updated_at) VALUES
  ('2cd385e1-8f7e-4908-8235-dfe42938b46d', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 1, 't', now(), now()),
  ('e97ea679-4560-4a9b-ad8b-80d2d191235e', '68b8281a-d19c-4d4b-88eb-6fd4a2afde1b', 1, 't', now(), now());

INSERT INTO spec_to_company (id_spec, uuid_company) VALUES
  (2, '2cd385e1-8f7e-4908-8235-dfe42938b46d'),
  (3, 'e97ea679-4560-4a9b-ad8b-80d2d191235e');

INSERT INTO company_history_list (uuid_company, id_type_of_change, old_data, changed_at) VALUES
  ('2cd385e1-8f7e-4908-8235-dfe42938b46d', 1, 'Комментарий к изменению', now());

INSERT INTO discussion_company_ref (id_discussion_parent, uuid_company,
uuid_author, message_content, is_delete, created_at, updated_at) VALUES
  (1, '2cd385e1-8f7e-4908-8235-dfe42938b46d', '31ecc6f8-0c09-4a59-a2d5-34b5b833e59b', 'this message', 'f', now(), now());

-- TABLE: representation_type_ref:  id (serial), representation_type (VARCHAR(100))
INSERT INTO representation_type_ref (representation_type) VALUES
  ('Представительство');

INSERT INTO role_member_ref (name) VALUES
  ('Стажер'),
  ('Инженер');

INSERT INTO role_access (id_role, id_type_access) VALUES
  (1, 2),
  (1, 1);

INSERT INTO company_access_to_component (uuid_component, uuid_company, id_type_access,
is_enabled, is_delete, created_at, updated_at) VALUES
  ('a5953fd9-7393-4f1e-a899-06b5e159dbf1', '2cd385e1-8f7e-4908-8235-dfe42938b46d', 1, 't', 'f', now(), now());
  
INSERT INTO company_access_to_standard (uuid_standard, uuid_company, id_type_access,
  is_enabled, is_delete, created_at, updated_at) VALUES
  ('303ec2aa-2066-42e3-93fb-de4fb9344bcb', '2cd385e1-8f7e-4908-8235-dfe42938b46d', 1, 't', 'f', now(), now());
