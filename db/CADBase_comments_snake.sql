COMMENT ON TABLE client_ref IS 'профиль';
COMMENT ON COLUMN client_ref.id IS 'id профиля';
COMMENT ON COLUMN client_ref.email IS 'email профиля, на один адрес может быть несколько профилей (закос под reddit)';
COMMENT ON COLUMN client_ref.email_verified IS 'подтверждение email';
COMMENT ON COLUMN client_ref.psw IS 'пароль профиля';
COMMENT ON COLUMN client_ref.id_type_org  IS 'тип профиля (физ. лицо, юр. лицо, ип)';
COMMENT ON COLUMN client_ref.firstname IS 'Имя';
COMMENT ON COLUMN client_ref.lastname IS 'Фамилия';
COMMENT ON COLUMN client_ref.secondname IS 'Отчество';
COMMENT ON COLUMN client_ref.nickname IS 'ник профиля (может использоваться для авторизации)';
COMMENT ON COLUMN client_ref.orgname IS 'наименование организации (для юр.лиц)';
COMMENT ON COLUMN client_ref.shortorgname IS 'сокращённое наименование организации (для юр.лиц)';
COMMENT ON COLUMN client_ref.inn IS 'инн профиля';
COMMENT ON COLUMN client_ref.phone IS 'номер телефона';
COMMENT ON COLUMN client_ref.id_name_cad IS 'САПР «по умолчанию» (для быстрой загрузки данных)';
COMMENT ON COLUMN client_ref.comment IS 'информация для связи, подпись';
COMMENT ON COLUMN client_ref.address IS 'почтовый адрес';
COMMENT ON COLUMN client_ref.time_zone IS 'часовой пояс профиля';
COMMENT ON COLUMN client_ref.position IS 'роль/должность';
COMMENT ON COLUMN client_ref.site_url IS 'URL адрес сайта профиля';
COMMENT ON COLUMN client_ref.id_file_info_icon IS 'картинка пользователя';
COMMENT ON COLUMN client_ref.id_region IS 'регион пользователя';

COMMENT ON TABLE client_tokens_ref IS 'токен сессии клиента';
COMMENT ON COLUMN client_tokens_ref.id IS 'id токена';
COMMENT ON COLUMN client_tokens_ref.id_client IS 'идентификатор пользователя';
COMMENT ON COLUMN client_tokens_ref.token IS 'токен пользователя';
COMMENT ON COLUMN client_tokens_ref.date_start IS 'дата создания токена';
COMMENT ON COLUMN client_tokens_ref.date_end IS 'дата окончания действия токена';

COMMENT ON TABLE client_represet_ref IS 'локальное представительство профиля';
COMMENT ON COLUMN client_represet_ref.id IS 'id представительства';
COMMENT ON COLUMN client_represet_ref.id_client IS 'id профиля (чьё представительства)';
COMMENT ON COLUMN client_represet_ref.id_region IS 'регион представительства';
COMMENT ON COLUMN client_represet_ref.id_representation_type IS 'тип представительства';
COMMENT ON COLUMN client_represet_ref.name IS 'наименование представительства';
COMMENT ON COLUMN client_represet_ref.address IS 'почтовый адрес представительства';
COMMENT ON COLUMN client_represet_ref.phone IS 'телефон представительства';

COMMENT ON TABLE representation_type_ref IS 'тип представительства профиля';
COMMENT ON COLUMN representation_type_ref.id IS 'id типа представительства';
COMMENT ON COLUMN representation_type_ref._representation_type IS 'наименование типа представительств';

COMMENT ON TABLE type_org_ref IS 'тип профиля';
COMMENT ON COLUMN type_org_ref.id IS 'id типа профиля';
COMMENT ON COLUMN type_org_ref.typeorg IS 'полное наименование (прим. "юридическое лицо")';
COMMENT ON COLUMN type_org_ref.typeorgshort IS 'сокращенное наименование (прим. "юр. лицо")';

COMMENT ON TABLE spec_ref IS 'категории (каталога)';
COMMENT ON COLUMN spec_ref.id IS 'id категории каталога';
COMMENT ON COLUMN spec_ref.spec IS 'наименование категории';
COMMENT ON COLUMN spec_ref.id_spec_parent IS 'id родительского каталога';

COMMENT ON TABLE name_cad_ref IS 'перечень CAD (и других программ)';
COMMENT ON COLUMN name_cad_ref.id IS 'id наименования софта';
COMMENT ON COLUMN name_cad_ref._name_cad IS 'наименование CAD (название программы)';

COMMENT ON TABLE spec2_client IS 'связь каталогов с профилем';
COMMENT ON COLUMN spec2_client.id IS 'id связи';
COMMENT ON COLUMN spec2_client.id_spec IS 'связанный с профилем каталог (категория)';
COMMENT ON COLUMN spec2_client.id_client IS 'связанный с каталогом (категорией) профиль';

COMMENT ON TABLE client_history_list IS 'запись изменений данных профиля';
COMMENT ON COLUMN client_history_list.id IS 'id события';
COMMENT ON COLUMN client_history_list.id_client IS 'идентификатор профиля к которому относится изменение';
COMMENT ON COLUMN client_history_list.datechange IS 'дата изменения ';
COMMENT ON COLUMN client_history_list.id_type_of_change IS 'id изменения (тип изменения)';
COMMENT ON COLUMN client_history_list.commentchange IS 'комментарий с вносимыми изменениями';

COMMENT ON TABLE type_of_change_ref IS 'перечень типов изменений';
COMMENT ON COLUMN type_of_change_ref.id IS 'id типа изменения';
COMMENT ON COLUMN type_of_change_ref._type_of_change IS 'наименование изменения';

COMMENT ON TABLE file_ref IS 'информация о файле (изображении)';
COMMENT ON COLUMN file_ref.id IS 'id файла';
COMMENT ON COLUMN file_ref.id_file IS 'идентификатор объекта/файла';
COMMENT ON COLUMN file_ref.id_client_create IS 'идентификатор профиля загрузившего файл';
COMMENT ON COLUMN file_ref.created_at IS 'дата создания/загрузки';
COMMENT ON COLUMN file_ref.filename IS 'наименование файла';
COMMENT ON COLUMN file_ref.id_ext IS 'расширение файла (используется для определения CAD)';
COMMENT ON COLUMN file_ref.filesize IS 'размер файла';
COMMENT ON COLUMN file_ref.path IS 'путь к файлу';

COMMENT ON TABLE extension_ref IS 'таблица соответствия CAD и расширений файлов';
COMMENT ON COLUMN extension_ref.id IS 'id соответствия';
COMMENT ON COLUMN extension_ref.extension IS 'расширение файла, одно расширение может быть у нескольких программ';
COMMENT ON COLUMN extension_ref.id_name_cad IS 'соответствующая программа (CAD)';

COMMENT ON TABLE component_ref IS 'компонент';
COMMENT ON COLUMN component_ref.id IS 'id компонента';
COMMENT ON COLUMN component_ref.name IS 'наименование компонента';
COMMENT ON COLUMN component_ref.id_client IS 'идентификатор профиля загрузившего компонент';
COMMENT ON COLUMN component_ref.comment IS 'краткое описание компонента';
COMMENT ON COLUMN component_ref.id_name_cad IS 'соответствующая программа (CAD)';
COMMENT ON COLUMN component_ref.id_component_parent IS 'родительский компонент';
COMMENT ON COLUMN component_ref.id_component_status IS 'номер статуса, к примеру: «актуальный», «архивный», «снято с производства»';
COMMENT ON COLUMN component_ref.id_component_type IS 'тип компонента (базовый/кастомный)';
COMMENT ON COLUMN component_ref.is_delete IS 'флаг удаления компонента';
COMMENT ON COLUMN component_ref.id_type_access IS 'тип доступности компонента';
COMMENT ON COLUMN component_ref.commentchange IS 'комментарий с вносимыми изменениями';
COMMENT ON COLUMN component_ref.is_standard IS 'компонент соответствует стандарту';
COMMENT ON COLUMN component_ref.created_at IS 'дата создания/загрузки';

COMMENT ON TABLE component_access2_client IS 'доступ к компоненту отдельного пользователя';
COMMENT ON COLUMN component_access2_client.id IS 'id доступа';
COMMENT ON COLUMN component_access2_client.id_component IS 'идентификатор компонента';
COMMENT ON COLUMN component_access2_client.id_client IS 'идентификатор профиля с доступом';
COMMENT ON COLUMN component_access2_client.id_type_access IS 'тип доступа профиля к компоненту';
COMMENT ON COLUMN component_access2_client.is_actual IS 'флаг актуальности доступа';
COMMENT ON COLUMN component_access2_client.is_delete IS 'флаг удаления доступа';
COMMENT ON COLUMN component_access2_client.created_at IS 'дата создания доступа';

COMMENT ON TABLE actual_status_ref IS 'статус компонента';
COMMENT ON COLUMN actual_status_ref.id IS 'id статуса';
COMMENT ON COLUMN actual_status_ref.componentstatus IS 'к примеру: «актуальный», «архивный», «снято с производства»';

COMMENT ON TABLE spec2_component IS 'каталог компонента';
COMMENT ON COLUMN spec2_component.id IS 'id связи компонента с каталогом';
COMMENT ON COLUMN spec2_component.id_spec IS 'идентификатор позиции в каталоге';
COMMENT ON COLUMN spec2_component.id_component IS 'идентификатор компонента';

COMMENT ON TABLE file2_component IS 'объект/файл компонента';
COMMENT ON COLUMN file2_component.id IS 'id файла компонента';
COMMENT ON COLUMN file2_component._c_o_l_l IS 'идентификатор объекта/файла';
COMMENT ON COLUMN file2_component._c_o_l_l IS 'идентификатор компонента';

COMMENT ON TABLE component_type_ref IS 'тип компонента (базовый, кастомный)';
COMMENT ON COLUMN component_type_ref.id IS 'id типа';
COMMENT ON COLUMN component_type_ref._component_type IS 'наименование типа';

COMMENT ON TABLE component_keyword_ref IS 'ключевые слова компонента (тегирование)';
COMMENT ON COLUMN component_keyword_ref.id IS 'id тега';
COMMENT ON COLUMN component_keyword_ref.keyword IS 'ключевое слово';

COMMENT ON TABLE component2_keyword IS 'ключевые слова связанные с компонентом (тегирование)';
COMMENT ON COLUMN component2_keyword.id IS 'id связи тега и компонента';
COMMENT ON COLUMN component2_keyword.id_component IS 'идентификатор компонента';
COMMENT ON COLUMN component2_keyword.id_component_keyword IS 'идентификатор ключевого слова (тега)';

COMMENT ON TABLE type_access_ref IS 'типы доступа';
COMMENT ON COLUMN type_access_ref.id IS 'id типа доступа';
COMMENT ON COLUMN type_access_ref._type_access IS 'наименование доступа';

COMMENT ON TABLE discussion_ref IS 'обсуждение компонента';
COMMENT ON COLUMN discussion_ref.id IS 'id комментария';
COMMENT ON COLUMN discussion_ref.created_at IS 'дата создания/редактирования';
COMMENT ON COLUMN discussion_ref.id_component IS 'идентификатор компонента';
COMMENT ON COLUMN discussion_ref.id_client_from IS 'идентификатор профиля отправителя';
COMMENT ON COLUMN discussion_ref.id_client_to IS 'идентификатор профиля адресата';
COMMENT ON COLUMN discussion_ref.comment IS 'сообщение/комментарий';
COMMENT ON COLUMN discussion_ref.id_discussion_parent IS 'id родительского комментария';

COMMENT ON TABLE param_ref IS 'параметры для компонента';
COMMENT ON COLUMN param_ref.id IS 'id параметра (характеристики)';
COMMENT ON COLUMN param_ref.paramname IS 'наименование параметра';

COMMENT ON TABLE language_ref IS 'перевод параметра (язык)';
COMMENT ON COLUMN language_ref.id IS 'id языка';
COMMENT ON COLUMN language_ref.lang IS 'полное наименование языка';
COMMENT ON COLUMN language_ref.langshort IS 'краткое наименование языка';

COMMENT ON TABLE param_translate_list IS 'перевод параметра (перевод)';
COMMENT ON COLUMN param_translate_list.id IS 'id перевода';
COMMENT ON COLUMN param_translate_list.id_param IS 'идентификатор параметра';
COMMENT ON COLUMN param_translate_list.id_lang IS 'идентификатор языка перевода';
COMMENT ON COLUMN param_translate_list.param IS 'перевод параметра';

COMMENT ON TABLE spec_translate_list IS 'перевод раздела каталога';
COMMENT ON COLUMN spec_translate_list.id IS 'id перевода';
COMMENT ON COLUMN spec_translate_list.id_spec IS 'идентификатор раздела';
COMMENT ON COLUMN spec_translate_list.id_lang IS 'идентификатор языка перевода';
COMMENT ON COLUMN spec_translate_list.spec IS 'перевод раздела';

COMMENT ON TABLE param2_component IS 'параметр компонента';
COMMENT ON COLUMN param2_component.id IS 'id параметра компонента';
COMMENT ON COLUMN param2_component.id_component IS 'идентификатор компонента';
COMMENT ON COLUMN param2_component.id_param IS 'идентификатор параметра';
COMMENT ON COLUMN param2_component.value IS 'параметр компонента';

COMMENT ON TABLE component_fav_ref IS 'отслеживание компонента';
COMMENT ON COLUMN component_fav_ref.id IS 'id подписки (начала отслеживания)';
COMMENT ON COLUMN component_fav_ref.id_component IS 'идентификатор компонента для отслеживания';
COMMENT ON COLUMN component_fav_ref.id_client IS 'идентификатор профиля';
COMMENT ON COLUMN component_fav_ref.created_at IS 'дата создания/редактирования';
COMMENT ON COLUMN component_fav_ref.is_active IS 'флаг актуальности отслеживания';

COMMENT ON TABLE region_ref IS 'регион'
COMMENT ON COLUMN region_ref.id IS 'id наименования региона'
COMMENT ON COLUMN region_ref.region IS 'наименование региона'

COMMENT ON TABLE component2_client IS 'список поставщиков компонента'
COMMENT ON COLUMN component2_client.id IS 'id записи профиля в поставщики компонента'
COMMENT ON COLUMN component2_client.id_component IS 'идентификатор компонента'
COMMENT ON COLUMN component2_client.id_client IS 'идентификатор профиля поставщика'
COMMENT ON COLUMN component2_client.comment IS 'комментарий к поставщику'

COMMENT ON TABLE component_modification_list IS 'список модификаций компонента'
COMMENT ON COLUMN component_modification_list.id IS 'id компонента'
COMMENT ON COLUMN component_modification_list.id_component IS 'идентификатор компонента'
COMMENT ON COLUMN component_modification_list._modification_name IS 'наименование модификации'
COMMENT ON COLUMN component_modification_list.created_at IS 'дата создания/загрузки'
COMMENT ON COLUMN component_modification_list.id_name_cad IS 'соответствующая программа (CAD)'
COMMENT ON COLUMN component_modification_list.comment IS 'комментарий к модификации'
COMMENT ON COLUMN component_modification_list.id_modification_parent IS 'родительская модификация'
COMMENT ON COLUMN component_modification_list.commentchange IS 'комментарий с вносимыми изменениями'
COMMENT ON COLUMN component_modification_list.id_actual_status IS 'номер статуса, к примеру: «актуальный», «архивный», «снято с производства»'
COMMENT ON COLUMN component_modification_list.is_delete IS 'флаг удаления компонента'

COMMENT ON TABLE file2_modification IS 'объект/файл модификации'
COMMENT ON COLUMN file2_modification.id IS 'id файла модификации'
COMMENT ON COLUMN file2_modification.id_modification IS 'идентификатор модификации'
COMMENT ON COLUMN file2_modification.id_file IS 'идентификатор объекта/файла'

COMMENT ON TABLE param2_modification IS 'параметр модификации'
COMMENT ON COLUMN param2_modification.id IS 'id параметра модификации'
COMMENT ON COLUMN param2_modification.id_modification IS 'идентификатор модификации'
COMMENT ON COLUMN param2_modification.id_param IS 'идентификатор параметра'
COMMENT ON COLUMN param2_modification.value IS 'параметр компонента'
