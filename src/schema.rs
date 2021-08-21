table! {
    actual_status_ref (id) {
        id -> Int4,
    }
}

table! {
    actual_status_translate_list (id_actual_status, id_lang) {
        id_actual_status -> Int4,
        id_lang -> Int4,
        name -> Varchar,
    }
}

table! {
    company_access_to_component (uuid_component, uuid_company) {
        uuid_component -> Uuid,
        uuid_company -> Uuid,
        id_type_access -> Int4,
        is_enabled -> Bool,
        is_delete -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    company_access_to_standard (uuid_standard, uuid_company) {
        uuid_standard -> Uuid,
        uuid_company -> Uuid,
        id_type_access -> Int4,
        is_enabled -> Bool,
        is_delete -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    company_certificate_ref (uuid_file, uuid_company) {
        uuid_file -> Uuid,
        uuid_company -> Uuid,
        description -> Varchar,
    }
}

table! {
    company_fav (uuid_company, uuid_user) {
        uuid_company -> Uuid,
        uuid_user -> Uuid,
        is_enabled -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    company_history_list (id) {
        id -> Int4,
        uuid_company -> Uuid,
        id_type_of_change -> Int4,
        old_data -> Varchar,
        changed_at -> Timestamp,
    }
}

table! {
    company_member_role (uuid_company, uuid_user, id_role) {
        uuid_company -> Uuid,
        uuid_user -> Uuid,
        id_role -> Int4,
        is_enabled -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    company_ref (uuid) {
        uuid -> Uuid,
        orgname -> Varchar,
        shortname -> Varchar,
        inn -> Varchar,
        phone -> Varchar,
        email -> Varchar,
        description -> Varchar,
        address -> Varchar,
        site_url -> Varchar,
        time_zone -> Varchar,
        uuid_user -> Uuid,
        uuid_image_file -> Uuid,
        id_region -> Int4,
        id_type_org -> Int4,
        is_supplier -> Bool,
        is_email_verified -> Bool,
        is_enabled -> Bool,
        is_delete -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    company_represent_ref (uuid) {
        uuid -> Uuid,
        uuid_company -> Uuid,
        id_region -> Int4,
        id_representation_type -> Int4,
        name -> Varchar,
        address -> Varchar,
        phone -> Varchar,
    }
}

table! {
    component_fav (uuid_component, uuid_user) {
        uuid_component -> Uuid,
        uuid_user -> Uuid,
        is_enabled -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    component_history_list (id) {
        id -> Int4,
        uuid_component -> Uuid,
        id_type_of_change -> Int4,
        old_data -> Varchar,
        changed_at -> Timestamp,
    }
}

table! {
    component_modification_list (uuid) {
        uuid -> Uuid,
        uuid_component -> Uuid,
        uuid_modification_parent -> Uuid,
        modification_name -> Varchar,
        description -> Varchar,
        id_actual_status -> Int4,
        is_delete -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    component_ref (uuid) {
        uuid -> Uuid,
        uuid_component_parent -> Uuid,
        name -> Varchar,
        description -> Varchar,
        uuid_user -> Uuid,
        id_type_access -> Int4,
        id_component_type -> Int4,
        id_actual_status -> Int4,
        is_standard -> Bool,
        is_delete -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    component_to_keyword (uuid_component, id_keyword) {
        uuid_component -> Uuid,
        id_keyword -> Int4,
    }
}

table! {
    component_type_ref (id) {
        id -> Int4,
    }
}

table! {
    component_type_translate_list (id_component_type, id_lang) {
        id_component_type -> Int4,
        id_lang -> Int4,
        component_type -> Varchar,
    }
}

table! {
    condition_to_license (id_condition, id_license) {
        id_condition -> Int4,
        id_license -> Int4,
    }
}

table! {
    degree_importance_ref (id) {
        id -> Int4,
    }
}

table! {
    degree_importance_translate_list (id_degree_importance, id_lang) {
        id_degree_importance -> Int4,
        id_lang -> Int4,
        degree -> Varchar,
    }
}

table! {
    discussion_company_ref (id) {
        id -> Int4,
        id_discussion_parent -> Int4,
        uuid_company -> Uuid,
        uuid_author -> Uuid,
        message_content -> Varchar,
        is_delete -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    discussion_component_ref (id) {
        id -> Int4,
        id_discussion_parent -> Int4,
        uuid_component -> Uuid,
        uuid_author -> Uuid,
        message_content -> Varchar,
        is_delete -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    extension_ref (id) {
        id -> Int4,
        extension -> Varchar,
        id_program -> Int4,
    }
}

table! {
    file_ref (uuid) {
        uuid -> Uuid,
        uuid_file_parent -> Uuid,
        hash -> Bytea,
        uuid_user -> Uuid,
        filename -> Varchar,
        content_type -> Varchar,
        id_ext -> Int4,
        filesize -> Int4,
        path_file -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    file_to_component (uuid_file, uuid_component) {
        uuid_file -> Uuid,
        uuid_component -> Uuid,
    }
}

table! {
    file_to_modification (uuid_file, uuid_modification) {
        uuid_file -> Uuid,
        uuid_modification -> Uuid,
    }
}

table! {
    file_to_set_modification (id_set, uuid_file) {
        id_set -> Int4,
        uuid_file -> Uuid,
    }
}

table! {
    file_to_standard (uuid_file, uuid_standard) {
        uuid_file -> Uuid,
        uuid_standard -> Uuid,
    }
}

table! {
    keyword_ref (id) {
        id -> Int4,
        keyword -> Varchar,
    }
}

table! {
    language_ref (id) {
        id -> Int4,
        lang -> Varchar,
        langshort -> Varchar,
    }
}

table! {
    license_condition_ref (id) {
        id -> Int4,
    }
}

table! {
    license_condition_translate_list (id_license_condition, id_lang) {
        id_license_condition -> Int4,
        id_lang -> Int4,
        condition -> Varchar,
    }
}

table! {
    license_limitation_ref (id) {
        id -> Int4,
    }
}

table! {
    license_limitation_translate_list (id_license_limitation, id_lang) {
        id_license_limitation -> Int4,
        id_lang -> Int4,
        limitation -> Varchar,
    }
}

table! {
    license_permission_ref (id) {
        id -> Int4,
    }
}

table! {
    license_permission_translate_list (id_license_permission, id_lang) {
        id_license_permission -> Int4,
        id_lang -> Int4,
        permission -> Varchar,
    }
}

table! {
    license_ref (id) {
        id -> Int4,
        name -> Varchar,
        keyword -> Varchar,
        publication_at -> Timestamp,
    }
}

table! {
    license_to_component (uuid_component, id_license) {
        uuid_component -> Uuid,
        id_license -> Int4,
    }
}

table! {
    limitation_to_license (id_limitation, id_license) {
        id_limitation -> Int4,
        id_license -> Int4,
    }
}

table! {
    notification_ref (id) {
        id -> Int4,
        notification -> Varchar,
        id_degree_importance -> Int4,
        generated_at -> Timestamp,
        is_read -> Bool,
    }
}

table! {
    notification_to_user (id_notification, uuid_user) {
        id -> Int4,
        id_notification -> Int4,
        uuid_user -> Uuid,
    }
}

table! {
    param_ref (id) {
        id -> Int4,
    }
}

table! {
    param_to_component (uuid_component, id_param) {
        uuid_component -> Uuid,
        id_param -> Int4,
        value -> Varchar,
    }
}

table! {
    param_to_modification (uuid_modification, id_param) {
        uuid_modification -> Uuid,
        id_param -> Int4,
        value -> Varchar,
    }
}

table! {
    param_translate_list (id_param, id_lang) {
        id_param -> Int4,
        id_lang -> Int4,
        paramname -> Varchar,
    }
}

table! {
    permission_to_license (id_permission, id_license) {
        id_permission -> Int4,
        id_license -> Int4,
    }
}

table! {
    program_ref (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    region_ref (id) {
        id -> Int4,
    }
}

table! {
    region_translate_list (id_region, id_lang) {
        id_region -> Int4,
        id_lang -> Int4,
        region -> Varchar,
    }
}

table! {
    representation_type_ref (id) {
        id -> Int4,
    }
}

table! {
    representation_type_translate_list (id_representation_type, id_lang) {
        id_representation_type -> Int4,
        id_lang -> Int4,
        representation_type -> Varchar,
    }
}

table! {
    role_access (id_role, id_type_access) {
        id_role -> Int4,
        id_type_access -> Int4,
    }
}

table! {
    role_member_ref (id) {
        id -> Int4,
    }
}

table! {
    role_member_translate_list (id_role_member, id_lang) {
        id_role_member -> Int4,
        id_lang -> Int4,
        name -> Varchar,
    }
}

table! {
    set_files_for_program (id) {
        id -> Int4,
        uuid_modification -> Uuid,
        id_program -> Int4,
    }
}

table! {
    spec_ref (id) {
        id -> Int4,
        id_spec_parent -> Int4,
    }
}

table! {
    spec_to_company (id_spec, uuid_company) {
        id_spec -> Int4,
        uuid_company -> Uuid,
    }
}

table! {
    spec_to_component (id_spec, uuid_component) {
        id_spec -> Int4,
        uuid_component -> Uuid,
    }
}

table! {
    spec_to_standard (id_spec, uuid_standard) {
        id_spec -> Int4,
        uuid_standard -> Uuid,
    }
}

table! {
    spec_translate_list (id_spec, id_lang) {
        id_spec -> Int4,
        id_lang -> Int4,
        spec -> Varchar,
    }
}

table! {
    standard_fav (uuid_standard, uuid_user) {
        uuid_standard -> Uuid,
        uuid_user -> Uuid,
        is_enabled -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    standard_history_list (id) {
        id -> Int4,
        uuid_standard -> Uuid,
        id_type_of_change -> Int4,
        old_data -> Varchar,
        changed_at -> Timestamp,
    }
}

table! {
    standard_ref (uuid) {
        uuid -> Uuid,
        uuid_standard_parent -> Uuid,
        classifier -> Varchar,
        name -> Varchar,
        description -> Varchar,
        specified_tolerance -> Varchar,
        technical_committee -> Varchar,
        publication_at -> Timestamp,
        uuid_image_file -> Uuid,
        uuid_user -> Uuid,
        uuid_company -> Uuid,
        id_type_access -> Int4,
        id_standard_status -> Int4,
        id_region -> Int4,
        is_delete -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    standard_status_ref (id) {
        id -> Int4,
    }
}

table! {
    standard_status_translate_list (id_standard_status, id_lang) {
        id_standard_status -> Int4,
        id_lang -> Int4,
        name -> Varchar,
    }
}

table! {
    standard_to_component (uuid_standard, uuid_component) {
        uuid_standard -> Uuid,
        uuid_component -> Uuid,
    }
}

table! {
    standard_to_keyword (uuid_standard, id_keyword) {
        uuid_standard -> Uuid,
        id_keyword -> Int4,
    }
}

table! {
    supplier_to_component (uuid_component, uuid_company) {
        uuid_component -> Uuid,
        uuid_company -> Uuid,
        description -> Varchar,
    }
}

table! {
    type_access_ref (id) {
        id -> Int4,
    }
}

table! {
    type_access_translate_list (id_type_access, id_lang) {
        id_type_access -> Int4,
        id_lang -> Int4,
        name -> Varchar,
    }
}

table! {
    type_company_ref (id) {
        id -> Int4,
    }
}

table! {
    type_company_translate_list (id_type_company, id_lang) {
        id_type_company -> Int4,
        id_lang -> Int4,
        name -> Varchar,
        shortname -> Varchar,
    }
}

table! {
    type_of_change_ref (id) {
        id -> Int4,
    }
}

table! {
    type_of_change_translate_list (id_type_of_change, id_lang) {
        id_type_of_change -> Int4,
        id_lang -> Int4,
        type_of_change -> Varchar,
    }
}

table! {
    user_access_to_component (uuid_component, uuid_user) {
        uuid_component -> Uuid,
        uuid_user -> Uuid,
        id_type_access -> Int4,
        is_enabled -> Bool,
        is_delete -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_access_to_standard (uuid_standard, uuid_user) {
        uuid_standard -> Uuid,
        uuid_user -> Uuid,
        id_type_access -> Int4,
        is_enabled -> Bool,
        is_delete -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_certificate_ref (uuid_file, uuid_user) {
        uuid_file -> Uuid,
        uuid_user -> Uuid,
        description -> Varchar,
    }
}

table! {
    user_fav (uuid_user_favorite, uuid_user_follower) {
        uuid_user_favorite -> Uuid,
        uuid_user_follower -> Uuid,
        is_enabled -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    user_history_list (id) {
        id -> Int4,
        uuid_user -> Uuid,
        id_type_of_change -> Int4,
        old_data -> Varchar,
        changed_at -> Timestamp,
    }
}

table! {
    user_ref (uuid) {
        uuid -> Uuid,
        email -> Varchar,
        psw_hash -> Bytea,
        psw_salt -> Varchar,
        firstname -> Varchar,
        lastname -> Varchar,
        secondname -> Varchar,
        username -> Varchar,
        phone -> Varchar,
        description -> Varchar,
        address -> Varchar,
        position -> Varchar,
        time_zone -> Varchar,
        uuid_image_file -> Uuid,
        id_region -> Int4,
        id_program -> Int4,
        is_email_verified -> Bool,
        is_enabled -> Bool,
        is_delete -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_tokens_ref (uuid_user, token) {
        uuid_user -> Uuid,
        token -> Varchar,
        start_at -> Timestamp,
        end_at -> Timestamp,
    }
}

joinable!(actual_status_translate_list -> actual_status_ref (id_actual_status));
joinable!(actual_status_translate_list -> language_ref (id_lang));
joinable!(company_access_to_component -> company_ref (uuid_company));
joinable!(company_access_to_component -> component_ref (uuid_component));
joinable!(company_access_to_component -> type_access_ref (id_type_access));
joinable!(company_access_to_standard -> company_ref (uuid_company));
joinable!(company_access_to_standard -> standard_ref (uuid_standard));
joinable!(company_access_to_standard -> type_access_ref (id_type_access));
joinable!(company_certificate_ref -> company_ref (uuid_company));
joinable!(company_certificate_ref -> file_ref (uuid_file));
joinable!(company_fav -> company_ref (uuid_company));
joinable!(company_fav -> user_ref (uuid_user));
joinable!(company_history_list -> company_ref (uuid_company));
joinable!(company_history_list -> type_of_change_ref (id_type_of_change));
joinable!(company_member_role -> company_ref (uuid_company));
joinable!(company_member_role -> role_member_ref (id_role));
joinable!(company_member_role -> user_ref (uuid_user));
joinable!(company_ref -> file_ref (uuid_image_file));
joinable!(company_ref -> region_ref (id_region));
joinable!(company_ref -> type_company_ref (id_type_org));
joinable!(company_ref -> user_ref (uuid_user));
joinable!(company_represent_ref -> company_ref (uuid_company));
joinable!(company_represent_ref -> region_ref (id_region));
joinable!(company_represent_ref -> representation_type_ref (id_representation_type));
joinable!(component_fav -> component_ref (uuid_component));
joinable!(component_fav -> user_ref (uuid_user));
joinable!(component_history_list -> component_ref (uuid_component));
joinable!(component_modification_list -> actual_status_ref (id_actual_status));
joinable!(component_modification_list -> component_ref (uuid_component));
joinable!(component_ref -> actual_status_ref (id_actual_status));
joinable!(component_ref -> component_type_ref (id_component_type));
joinable!(component_ref -> type_access_ref (id_type_access));
joinable!(component_ref -> user_ref (uuid_user));
joinable!(component_to_keyword -> component_ref (uuid_component));
joinable!(component_to_keyword -> keyword_ref (id_keyword));
joinable!(component_type_translate_list -> component_type_ref (id_component_type));
joinable!(component_type_translate_list -> language_ref (id_lang));
joinable!(condition_to_license -> license_condition_ref (id_condition));
joinable!(condition_to_license -> license_ref (id_license));
joinable!(degree_importance_translate_list -> degree_importance_ref (id_degree_importance));
joinable!(degree_importance_translate_list -> language_ref (id_lang));
joinable!(discussion_company_ref -> company_ref (uuid_company));
joinable!(discussion_company_ref -> user_ref (uuid_author));
joinable!(discussion_component_ref -> component_ref (uuid_component));
joinable!(discussion_component_ref -> user_ref (uuid_author));
joinable!(extension_ref -> program_ref (id_program));
joinable!(file_ref -> extension_ref (id_ext));
joinable!(file_to_component -> component_ref (uuid_component));
joinable!(file_to_component -> file_ref (uuid_file));
joinable!(file_to_modification -> component_modification_list (uuid_modification));
joinable!(file_to_modification -> file_ref (uuid_file));
joinable!(file_to_set_modification -> file_ref (uuid_file));
joinable!(file_to_set_modification -> set_files_for_program (id_set));
joinable!(file_to_standard -> file_ref (uuid_file));
joinable!(file_to_standard -> standard_ref (uuid_standard));
joinable!(license_condition_translate_list -> language_ref (id_lang));
joinable!(license_condition_translate_list -> license_condition_ref (id_license_condition));
joinable!(license_limitation_translate_list -> language_ref (id_lang));
joinable!(license_limitation_translate_list -> license_limitation_ref (id_license_limitation));
joinable!(license_permission_translate_list -> language_ref (id_lang));
joinable!(license_permission_translate_list -> license_permission_ref (id_license_permission));
joinable!(license_to_component -> component_ref (uuid_component));
joinable!(license_to_component -> license_ref (id_license));
joinable!(limitation_to_license -> license_limitation_ref (id_limitation));
joinable!(limitation_to_license -> license_ref (id_license));
joinable!(notification_ref -> degree_importance_ref (id_degree_importance));
joinable!(notification_to_user -> notification_ref (id_notification));
joinable!(notification_to_user -> user_ref (uuid_user));
joinable!(param_to_component -> component_ref (uuid_component));
joinable!(param_to_component -> param_ref (id_param));
joinable!(param_to_modification -> component_modification_list (uuid_modification));
joinable!(param_to_modification -> param_ref (id_param));
joinable!(param_translate_list -> language_ref (id_lang));
joinable!(param_translate_list -> param_ref (id_param));
joinable!(permission_to_license -> license_permission_ref (id_permission));
joinable!(permission_to_license -> license_ref (id_license));
joinable!(region_translate_list -> language_ref (id_lang));
joinable!(region_translate_list -> region_ref (id_region));
joinable!(representation_type_translate_list -> language_ref (id_lang));
joinable!(representation_type_translate_list -> representation_type_ref (id_representation_type));
joinable!(role_access -> role_member_ref (id_role));
joinable!(role_access -> type_access_ref (id_type_access));
joinable!(role_member_translate_list -> language_ref (id_lang));
joinable!(role_member_translate_list -> role_member_ref (id_role_member));
joinable!(set_files_for_program -> component_modification_list (uuid_modification));
joinable!(set_files_for_program -> program_ref (id_program));
joinable!(spec_to_company -> company_ref (uuid_company));
joinable!(spec_to_company -> spec_ref (id_spec));
joinable!(spec_to_component -> component_ref (uuid_component));
joinable!(spec_to_component -> spec_ref (id_spec));
joinable!(spec_to_standard -> spec_ref (id_spec));
joinable!(spec_to_standard -> standard_ref (uuid_standard));
joinable!(spec_translate_list -> language_ref (id_lang));
joinable!(spec_translate_list -> spec_ref (id_spec));
joinable!(standard_fav -> standard_ref (uuid_standard));
joinable!(standard_fav -> user_ref (uuid_user));
joinable!(standard_history_list -> standard_ref (uuid_standard));
joinable!(standard_ref -> company_ref (uuid_company));
joinable!(standard_ref -> file_ref (uuid_image_file));
joinable!(standard_ref -> region_ref (id_region));
joinable!(standard_ref -> standard_status_ref (id_standard_status));
joinable!(standard_ref -> type_access_ref (id_type_access));
joinable!(standard_ref -> user_ref (uuid_user));
joinable!(standard_status_translate_list -> language_ref (id_lang));
joinable!(standard_status_translate_list -> standard_status_ref (id_standard_status));
joinable!(standard_to_component -> component_ref (uuid_component));
joinable!(standard_to_component -> standard_ref (uuid_standard));
joinable!(standard_to_keyword -> keyword_ref (id_keyword));
joinable!(standard_to_keyword -> standard_ref (uuid_standard));
joinable!(supplier_to_component -> company_ref (uuid_company));
joinable!(supplier_to_component -> component_ref (uuid_component));
joinable!(type_access_translate_list -> language_ref (id_lang));
joinable!(type_access_translate_list -> type_access_ref (id_type_access));
joinable!(type_company_translate_list -> language_ref (id_lang));
joinable!(type_company_translate_list -> type_company_ref (id_type_company));
joinable!(type_of_change_translate_list -> language_ref (id_lang));
joinable!(type_of_change_translate_list -> type_of_change_ref (id_type_of_change));
joinable!(user_access_to_component -> component_ref (uuid_component));
joinable!(user_access_to_component -> type_access_ref (id_type_access));
joinable!(user_access_to_component -> user_ref (uuid_user));
joinable!(user_access_to_standard -> standard_ref (uuid_standard));
joinable!(user_access_to_standard -> type_access_ref (id_type_access));
joinable!(user_access_to_standard -> user_ref (uuid_user));
joinable!(user_certificate_ref -> file_ref (uuid_file));
joinable!(user_certificate_ref -> user_ref (uuid_user));
joinable!(user_history_list -> type_of_change_ref (id_type_of_change));
joinable!(user_history_list -> user_ref (uuid_user));
joinable!(user_ref -> program_ref (id_program));
joinable!(user_ref -> region_ref (id_region));
joinable!(user_tokens_ref -> user_ref (uuid_user));

allow_tables_to_appear_in_same_query!(
    actual_status_ref,
    actual_status_translate_list,
    company_access_to_component,
    company_access_to_standard,
    company_certificate_ref,
    company_fav,
    company_history_list,
    company_member_role,
    company_ref,
    company_represent_ref,
    component_fav,
    component_history_list,
    component_modification_list,
    component_ref,
    component_to_keyword,
    component_type_ref,
    component_type_translate_list,
    condition_to_license,
    degree_importance_ref,
    degree_importance_translate_list,
    discussion_company_ref,
    discussion_component_ref,
    extension_ref,
    file_ref,
    file_to_component,
    file_to_modification,
    file_to_set_modification,
    file_to_standard,
    keyword_ref,
    language_ref,
    license_condition_ref,
    license_condition_translate_list,
    license_limitation_ref,
    license_limitation_translate_list,
    license_permission_ref,
    license_permission_translate_list,
    license_ref,
    license_to_component,
    limitation_to_license,
    notification_ref,
    notification_to_user,
    param_ref,
    param_to_component,
    param_to_modification,
    param_translate_list,
    permission_to_license,
    program_ref,
    region_ref,
    region_translate_list,
    representation_type_ref,
    representation_type_translate_list,
    role_access,
    role_member_ref,
    role_member_translate_list,
    set_files_for_program,
    spec_ref,
    spec_to_company,
    spec_to_component,
    spec_to_standard,
    spec_translate_list,
    standard_fav,
    standard_history_list,
    standard_ref,
    standard_status_ref,
    standard_status_translate_list,
    standard_to_component,
    standard_to_keyword,
    supplier_to_component,
    type_access_ref,
    type_access_translate_list,
    type_company_ref,
    type_company_translate_list,
    type_of_change_ref,
    type_of_change_translate_list,
    user_access_to_component,
    user_access_to_standard,
    user_certificate_ref,
    user_fav,
    user_history_list,
    user_ref,
    user_tokens_ref,
);
