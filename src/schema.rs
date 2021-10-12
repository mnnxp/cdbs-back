table! {
    actual_status_ref (id) {
        id -> Int4,
    }
}

table! {
    actual_status_translate_list (actual_status_id, lang_id) {
        actual_status_id -> Int4,
        lang_id -> Int4,
        name -> Varchar,
    }
}

table! {
    company_access_to_component (component_uuid, company_uuid) {
        component_uuid -> Uuid,
        company_uuid -> Uuid,
        type_access_id -> Int4,
        is_enabled -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    company_access_to_standard (standard_uuid, company_uuid) {
        standard_uuid -> Uuid,
        company_uuid -> Uuid,
        type_access_id -> Int4,
        is_enabled -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    company_certificate_ref (file_uuid, company_uuid) {
        file_uuid -> Uuid,
        company_uuid -> Uuid,
        description -> Varchar,
    }
}

table! {
    company_fav (company_uuid, user_uuid) {
        company_uuid -> Uuid,
        user_uuid -> Uuid,
        is_enabled -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    company_history_list (id) {
        id -> Int4,
        company_uuid -> Uuid,
        type_of_change_id -> Int4,
        old_data -> Varchar,
        changed_at -> Timestamp,
    }
}

table! {
    company_member_list (company_uuid, user_uuid) {
        company_uuid -> Uuid,
        user_uuid -> Uuid,
        role_id -> Int4,
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
        user_uuid -> Uuid,
        image_file_uuid -> Uuid,
        region_id -> Int4,
        company_type_id -> Int4,
        type_access_id -> Int4,
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
        company_uuid -> Uuid,
        region_id -> Int4,
        representation_type_id -> Int4,
        name -> Varchar,
        address -> Varchar,
        phone -> Varchar,
    }
}

table! {
    company_type_ref (id) {
        id -> Int4,
    }
}

table! {
    company_type_translate_list (company_type_id, lang_id) {
        company_type_id -> Int4,
        lang_id -> Int4,
        name -> Varchar,
        shortname -> Varchar,
    }
}

table! {
    component_fav (component_uuid, user_uuid) {
        component_uuid -> Uuid,
        user_uuid -> Uuid,
        is_enabled -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    component_history_list (id) {
        id -> Int4,
        component_uuid -> Uuid,
        type_of_change_id -> Int4,
        old_data -> Varchar,
        changed_at -> Timestamp,
    }
}

table! {
    component_modification_list (uuid) {
        uuid -> Uuid,
        component_uuid -> Uuid,
        parent_modification_uuid -> Uuid,
        modification_name -> Varchar,
        description -> Varchar,
        actual_status_id -> Int4,
        is_delete -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    component_ref (uuid) {
        uuid -> Uuid,
        parent_component_uuid -> Uuid,
        name -> Varchar,
        description -> Varchar,
        user_uuid -> Uuid,
        type_access_id -> Int4,
        component_type_id -> Int4,
        actual_status_id -> Int4,
        is_base -> Bool,
        is_delete -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    component_type_ref (id) {
        id -> Int4,
    }
}

table! {
    component_type_translate_list (component_type_id, lang_id) {
        component_type_id -> Int4,
        lang_id -> Int4,
        component_type -> Varchar,
    }
}

table! {
    condition_to_license (condition_id, license_id) {
        condition_id -> Int4,
        license_id -> Int4,
    }
}

table! {
    degree_importance_ref (id) {
        id -> Int4,
    }
}

table! {
    degree_importance_translate_list (degree_importance_id, lang_id) {
        degree_importance_id -> Int4,
        lang_id -> Int4,
        degree -> Varchar,
    }
}

table! {
    discussion_company_ref (id) {
        id -> Int4,
        parent_discussion_id -> Int4,
        company_uuid -> Uuid,
        author_uuid -> Uuid,
        message_content -> Varchar,
        is_delete -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    discussion_component_ref (id) {
        id -> Int4,
        parent_discussion_id -> Int4,
        component_uuid -> Uuid,
        author_uuid -> Uuid,
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
        program_id -> Int4,
    }
}

table! {
    file_ref (uuid) {
        uuid -> Uuid,
        parent_file_uuid -> Uuid,
        hash -> Bytea,
        user_uuid -> Uuid,
        filename -> Varchar,
        content_type -> Varchar,
        id_ext -> Int4,
        filesize -> Int8,
        path_file -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    file_to_component (file_uuid, component_uuid) {
        file_uuid -> Uuid,
        component_uuid -> Uuid,
    }
}

table! {
    file_to_modification (file_uuid, modification_uuid) {
        file_uuid -> Uuid,
        modification_uuid -> Uuid,
    }
}

table! {
    file_to_standard (file_uuid, standard_uuid) {
        file_uuid -> Uuid,
        standard_uuid -> Uuid,
    }
}

table! {
    fileset_for_program (uuid) {
        uuid -> Uuid,
        modification_uuid -> Uuid,
        program_id -> Int4,
    }
}

table! {
    keyword_ref (id) {
        id -> Int4,
        keyword -> Varchar,
    }
}

table! {
    keyword_to_component (component_uuid, keyword_id) {
        component_uuid -> Uuid,
        keyword_id -> Int4,
    }
}

table! {
    keyword_to_standard (standard_uuid, keyword_id) {
        standard_uuid -> Uuid,
        keyword_id -> Int4,
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
    license_condition_translate_list (condition_license_id, lang_id) {
        condition_license_id -> Int4,
        lang_id -> Int4,
        condition -> Varchar,
    }
}

table! {
    license_limitation_ref (id) {
        id -> Int4,
    }
}

table! {
    license_limitation_translate_list (limitation_license_id, lang_id) {
        limitation_license_id -> Int4,
        lang_id -> Int4,
        limitation -> Varchar,
    }
}

table! {
    license_permission_ref (id) {
        id -> Int4,
    }
}

table! {
    license_permission_translate_list (permission_license_id, lang_id) {
        permission_license_id -> Int4,
        lang_id -> Int4,
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
    license_to_component (component_uuid, license_id) {
        component_uuid -> Uuid,
        license_id -> Int4,
    }
}

table! {
    limitation_to_license (limitation_id, license_id) {
        limitation_id -> Int4,
        license_id -> Int4,
    }
}

table! {
    modification_file_from_fileset (fileset_uuid, file_uuid) {
        fileset_uuid -> Uuid,
        file_uuid -> Uuid,
    }
}

table! {
    notification_ref (id) {
        id -> Int4,
        notification -> Varchar,
        degree_importance_id -> Int4,
        created_at -> Timestamp,
        is_read -> Bool,
    }
}

table! {
    notification_to_user (notification_id, user_uuid) {
        notification_id -> Int4,
        user_uuid -> Uuid,
    }
}

table! {
    param_ref (id) {
        id -> Int4,
    }
}

table! {
    param_to_component (component_uuid, param_id) {
        component_uuid -> Uuid,
        param_id -> Int4,
        value -> Varchar,
    }
}

table! {
    param_to_modification (modification_uuid, param_id) {
        modification_uuid -> Uuid,
        param_id -> Int4,
        value -> Varchar,
    }
}

table! {
    param_translate_list (param_id, lang_id) {
        param_id -> Int4,
        lang_id -> Int4,
        paramname -> Varchar,
    }
}

table! {
    permission_to_license (permission_id, license_id) {
        permission_id -> Int4,
        license_id -> Int4,
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
    region_translate_list (region_id, lang_id) {
        region_id -> Int4,
        lang_id -> Int4,
        region -> Varchar,
    }
}

table! {
    representation_type_ref (id) {
        id -> Int4,
    }
}

table! {
    representation_type_translate_list (representation_type_id, lang_id) {
        representation_type_id -> Int4,
        lang_id -> Int4,
        representation_type -> Varchar,
    }
}

table! {
    role_access (role_id, type_access_id) {
        role_id -> Int4,
        type_access_id -> Int4,
    }
}

table! {
    role_member_list (id) {
        id -> Int4,
        company_uuid -> Uuid,
    }
}

table! {
    role_member_translate_list (role_member_id, lang_id) {
        role_member_id -> Int4,
        lang_id -> Int4,
        name -> Varchar,
    }
}

table! {
    spec_ref (id) {
        id -> Int4,
        parent_spec_id -> Int4,
    }
}

table! {
    spec_to_company (spec_id, company_uuid) {
        spec_id -> Int4,
        company_uuid -> Uuid,
    }
}

table! {
    spec_to_component (spec_id, component_uuid) {
        spec_id -> Int4,
        component_uuid -> Uuid,
    }
}

table! {
    spec_to_standard (spec_id, standard_uuid) {
        spec_id -> Int4,
        standard_uuid -> Uuid,
    }
}

table! {
    spec_translate_list (spec_id, lang_id) {
        spec_id -> Int4,
        lang_id -> Int4,
        spec -> Varchar,
    }
}

table! {
    standard_fav (standard_uuid, user_uuid) {
        standard_uuid -> Uuid,
        user_uuid -> Uuid,
        is_enabled -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    standard_history_list (id) {
        id -> Int4,
        standard_uuid -> Uuid,
        type_of_change_id -> Int4,
        old_data -> Varchar,
        changed_at -> Timestamp,
    }
}

table! {
    standard_ref (uuid) {
        uuid -> Uuid,
        parent_standard_uuid -> Uuid,
        classifier -> Varchar,
        name -> Varchar,
        description -> Varchar,
        specified_tolerance -> Varchar,
        technical_committee -> Varchar,
        publication_at -> Timestamp,
        image_file_uuid -> Uuid,
        user_uuid -> Uuid,
        company_uuid -> Uuid,
        type_access_id -> Int4,
        standard_status_id -> Int4,
        region_id -> Int4,
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
    standard_status_translate_list (standard_status_id, lang_id) {
        standard_status_id -> Int4,
        lang_id -> Int4,
        name -> Varchar,
    }
}

table! {
    standard_to_component (standard_uuid, component_uuid) {
        standard_uuid -> Uuid,
        component_uuid -> Uuid,
    }
}

table! {
    storage_access_ref (id) {
        id -> Int4,
        application_key_id -> Varchar,
        application_key -> Varchar,
        expiration_at -> Timestamp,
        bucket -> Varchar,
        region -> Varchar,
        endpoint -> Varchar,
    }
}

table! {
    supplier_to_component (component_uuid, company_uuid) {
        component_uuid -> Uuid,
        company_uuid -> Uuid,
        description -> Varchar,
    }
}

table! {
    type_access_ref (id) {
        id -> Int4,
    }
}

table! {
    type_access_translate_list (type_access_id, lang_id) {
        type_access_id -> Int4,
        lang_id -> Int4,
        name -> Varchar,
    }
}

table! {
    type_of_change_ref (id) {
        id -> Int4,
    }
}

table! {
    type_of_change_translate_list (type_of_change_id, lang_id) {
        type_of_change_id -> Int4,
        lang_id -> Int4,
        type_of_change -> Varchar,
    }
}

table! {
    user_access_to_component (component_uuid, user_uuid) {
        component_uuid -> Uuid,
        user_uuid -> Uuid,
        type_access_id -> Int4,
        is_enabled -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_access_to_standard (standard_uuid, user_uuid) {
        standard_uuid -> Uuid,
        user_uuid -> Uuid,
        type_access_id -> Int4,
        is_enabled -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_certificate_ref (file_uuid, user_uuid) {
        file_uuid -> Uuid,
        user_uuid -> Uuid,
        description -> Varchar,
    }
}

table! {
    user_fav (user_favorite_uuid, user_follower_uuid) {
        user_favorite_uuid -> Uuid,
        user_follower_uuid -> Uuid,
        is_enabled -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    user_history_list (id) {
        id -> Int4,
        user_uuid -> Uuid,
        type_of_change_id -> Int4,
        old_data -> Varchar,
        changed_at -> Timestamp,
    }
}

table! {
    user_ref (uuid) {
        uuid -> Uuid,
        email -> Varchar,
        psw_hash -> Bytea,
        psw_salt -> Bytea,
        firstname -> Varchar,
        lastname -> Varchar,
        secondname -> Varchar,
        username -> Varchar,
        phone -> Varchar,
        description -> Varchar,
        address -> Varchar,
        position -> Varchar,
        time_zone -> Varchar,
        image_file_uuid -> Uuid,
        region_id -> Int4,
        program_id -> Int4,
        type_access_id -> Int4,
        is_email_verified -> Bool,
        is_enabled -> Bool,
        is_delete -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_token_ref (user_uuid, token) {
        user_uuid -> Uuid,
        token -> Varchar,
        created_at -> Timestamp,
        expiration_at -> Timestamp,
    }
}

joinable!(actual_status_translate_list -> actual_status_ref (actual_status_id));
joinable!(actual_status_translate_list -> language_ref (lang_id));
joinable!(company_access_to_component -> company_ref (company_uuid));
joinable!(company_access_to_component -> component_ref (component_uuid));
joinable!(company_access_to_component -> type_access_ref (type_access_id));
joinable!(company_access_to_standard -> company_ref (company_uuid));
joinable!(company_access_to_standard -> standard_ref (standard_uuid));
joinable!(company_access_to_standard -> type_access_ref (type_access_id));
joinable!(company_certificate_ref -> company_ref (company_uuid));
joinable!(company_certificate_ref -> file_ref (file_uuid));
joinable!(company_fav -> company_ref (company_uuid));
joinable!(company_fav -> user_ref (user_uuid));
joinable!(company_history_list -> company_ref (company_uuid));
joinable!(company_history_list -> type_of_change_ref (type_of_change_id));
joinable!(company_member_list -> company_ref (company_uuid));
joinable!(company_member_list -> role_member_list (role_id));
joinable!(company_member_list -> user_ref (user_uuid));
joinable!(company_ref -> company_type_ref (company_type_id));
joinable!(company_ref -> file_ref (image_file_uuid));
joinable!(company_ref -> region_ref (region_id));
joinable!(company_ref -> type_access_ref (type_access_id));
joinable!(company_ref -> user_ref (user_uuid));
joinable!(company_represent_ref -> company_ref (company_uuid));
joinable!(company_represent_ref -> region_ref (region_id));
joinable!(company_represent_ref -> representation_type_ref (representation_type_id));
joinable!(company_type_translate_list -> company_type_ref (company_type_id));
joinable!(company_type_translate_list -> language_ref (lang_id));
joinable!(component_fav -> component_ref (component_uuid));
joinable!(component_fav -> user_ref (user_uuid));
joinable!(component_history_list -> component_ref (component_uuid));
joinable!(component_modification_list -> actual_status_ref (actual_status_id));
joinable!(component_modification_list -> component_ref (component_uuid));
joinable!(component_ref -> actual_status_ref (actual_status_id));
joinable!(component_ref -> component_type_ref (component_type_id));
joinable!(component_ref -> type_access_ref (type_access_id));
joinable!(component_ref -> user_ref (user_uuid));
joinable!(component_type_translate_list -> component_type_ref (component_type_id));
joinable!(component_type_translate_list -> language_ref (lang_id));
joinable!(condition_to_license -> license_condition_ref (condition_id));
joinable!(condition_to_license -> license_ref (license_id));
joinable!(degree_importance_translate_list -> degree_importance_ref (degree_importance_id));
joinable!(degree_importance_translate_list -> language_ref (lang_id));
joinable!(discussion_company_ref -> company_ref (company_uuid));
joinable!(discussion_company_ref -> user_ref (author_uuid));
joinable!(discussion_component_ref -> component_ref (component_uuid));
joinable!(discussion_component_ref -> user_ref (author_uuid));
joinable!(extension_ref -> program_ref (program_id));
joinable!(file_ref -> extension_ref (id_ext));
joinable!(file_to_component -> component_ref (component_uuid));
joinable!(file_to_component -> file_ref (file_uuid));
joinable!(file_to_modification -> component_modification_list (modification_uuid));
joinable!(file_to_modification -> file_ref (file_uuid));
joinable!(file_to_standard -> file_ref (file_uuid));
joinable!(file_to_standard -> standard_ref (standard_uuid));
joinable!(fileset_for_program -> component_modification_list (modification_uuid));
joinable!(fileset_for_program -> program_ref (program_id));
joinable!(keyword_to_component -> component_ref (component_uuid));
joinable!(keyword_to_component -> keyword_ref (keyword_id));
joinable!(keyword_to_standard -> keyword_ref (keyword_id));
joinable!(keyword_to_standard -> standard_ref (standard_uuid));
joinable!(license_condition_translate_list -> language_ref (lang_id));
joinable!(license_condition_translate_list -> license_condition_ref (condition_license_id));
joinable!(license_limitation_translate_list -> language_ref (lang_id));
joinable!(license_limitation_translate_list -> license_limitation_ref (limitation_license_id));
joinable!(license_permission_translate_list -> language_ref (lang_id));
joinable!(license_permission_translate_list -> license_permission_ref (permission_license_id));
joinable!(license_to_component -> component_ref (component_uuid));
joinable!(license_to_component -> license_ref (license_id));
joinable!(limitation_to_license -> license_limitation_ref (limitation_id));
joinable!(limitation_to_license -> license_ref (license_id));
joinable!(modification_file_from_fileset -> file_ref (file_uuid));
joinable!(modification_file_from_fileset -> fileset_for_program (fileset_uuid));
joinable!(notification_ref -> degree_importance_ref (degree_importance_id));
joinable!(notification_to_user -> notification_ref (notification_id));
joinable!(notification_to_user -> user_ref (user_uuid));
joinable!(param_to_component -> component_ref (component_uuid));
joinable!(param_to_component -> param_ref (param_id));
joinable!(param_to_modification -> component_modification_list (modification_uuid));
joinable!(param_to_modification -> param_ref (param_id));
joinable!(param_translate_list -> language_ref (lang_id));
joinable!(param_translate_list -> param_ref (param_id));
joinable!(permission_to_license -> license_permission_ref (permission_id));
joinable!(permission_to_license -> license_ref (license_id));
joinable!(region_translate_list -> language_ref (lang_id));
joinable!(region_translate_list -> region_ref (region_id));
joinable!(representation_type_translate_list -> language_ref (lang_id));
joinable!(representation_type_translate_list -> representation_type_ref (representation_type_id));
joinable!(role_access -> role_member_list (role_id));
joinable!(role_access -> type_access_ref (type_access_id));
joinable!(role_member_list -> company_ref (company_uuid));
joinable!(role_member_translate_list -> language_ref (lang_id));
joinable!(role_member_translate_list -> role_member_list (role_member_id));
joinable!(spec_to_company -> company_ref (company_uuid));
joinable!(spec_to_company -> spec_ref (spec_id));
joinable!(spec_to_component -> component_ref (component_uuid));
joinable!(spec_to_component -> spec_ref (spec_id));
joinable!(spec_to_standard -> spec_ref (spec_id));
joinable!(spec_to_standard -> standard_ref (standard_uuid));
joinable!(spec_translate_list -> language_ref (lang_id));
joinable!(spec_translate_list -> spec_ref (spec_id));
joinable!(standard_fav -> standard_ref (standard_uuid));
joinable!(standard_fav -> user_ref (user_uuid));
joinable!(standard_history_list -> standard_ref (standard_uuid));
joinable!(standard_ref -> company_ref (company_uuid));
joinable!(standard_ref -> file_ref (image_file_uuid));
joinable!(standard_ref -> region_ref (region_id));
joinable!(standard_ref -> standard_status_ref (standard_status_id));
joinable!(standard_ref -> type_access_ref (type_access_id));
joinable!(standard_ref -> user_ref (user_uuid));
joinable!(standard_status_translate_list -> language_ref (lang_id));
joinable!(standard_status_translate_list -> standard_status_ref (standard_status_id));
joinable!(standard_to_component -> component_ref (component_uuid));
joinable!(standard_to_component -> standard_ref (standard_uuid));
joinable!(supplier_to_component -> company_ref (company_uuid));
joinable!(supplier_to_component -> component_ref (component_uuid));
joinable!(type_access_translate_list -> language_ref (lang_id));
joinable!(type_access_translate_list -> type_access_ref (type_access_id));
joinable!(type_of_change_translate_list -> language_ref (lang_id));
joinable!(type_of_change_translate_list -> type_of_change_ref (type_of_change_id));
joinable!(user_access_to_component -> component_ref (component_uuid));
joinable!(user_access_to_component -> type_access_ref (type_access_id));
joinable!(user_access_to_component -> user_ref (user_uuid));
joinable!(user_access_to_standard -> standard_ref (standard_uuid));
joinable!(user_access_to_standard -> type_access_ref (type_access_id));
joinable!(user_access_to_standard -> user_ref (user_uuid));
joinable!(user_certificate_ref -> file_ref (file_uuid));
joinable!(user_certificate_ref -> user_ref (user_uuid));
joinable!(user_history_list -> type_of_change_ref (type_of_change_id));
joinable!(user_history_list -> user_ref (user_uuid));
joinable!(user_ref -> program_ref (program_id));
joinable!(user_ref -> region_ref (region_id));
joinable!(user_ref -> type_access_ref (type_access_id));
joinable!(user_token_ref -> user_ref (user_uuid));

allow_tables_to_appear_in_same_query!(
    actual_status_ref,
    actual_status_translate_list,
    company_access_to_component,
    company_access_to_standard,
    company_certificate_ref,
    company_fav,
    company_history_list,
    company_member_list,
    company_ref,
    company_represent_ref,
    company_type_ref,
    company_type_translate_list,
    component_fav,
    component_history_list,
    component_modification_list,
    component_ref,
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
    file_to_standard,
    fileset_for_program,
    keyword_ref,
    keyword_to_component,
    keyword_to_standard,
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
    modification_file_from_fileset,
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
    role_member_list,
    role_member_translate_list,
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
    storage_access_ref,
    supplier_to_component,
    type_access_ref,
    type_access_translate_list,
    type_of_change_ref,
    type_of_change_translate_list,
    user_access_to_component,
    user_access_to_standard,
    user_certificate_ref,
    user_fav,
    user_history_list,
    user_ref,
    user_token_ref,
);
