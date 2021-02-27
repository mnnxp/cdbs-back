table! {
    actual_status_ref (id) {
        id -> Int4,
        actualstatus -> Varchar,
    }
}

table! {
    component_access_to_user (id) {
        id -> Int4,
        id_component -> Int4,
        id_user -> Int4,
        id_type_access -> Int4,
        is_actual -> Int4,
        is_delete -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    component_fav_ref (id) {
        id -> Int4,
        id_component -> Int4,
        id_user -> Int4,
        created_at -> Timestamp,
        is_active -> Int4,
    }
}

table! {
    component_keyword_ref (id) {
        id -> Int4,
        keyword -> Varchar,
    }
}

table! {
    component_modification_list (id) {
        id -> Int4,
        id_component -> Int4,
        modification_name -> Varchar,
        created_at -> Timestamp,
        id_name_cad -> Int4,
        comment -> Varchar,
        id_modification_parent -> Int4,
        commentchange -> Varchar,
        id_actual_status -> Int4,
        is_delete -> Int4,
    }
}

table! {
    component_ref (id) {
        id -> Int4,
        name -> Varchar,
        id_user -> Int4,
        comment -> Varchar,
        id_component_parent -> Int4,
        id_actual_status -> Int4,
        id_component_type -> Int4,
        is_delete -> Int4,
        id_type_access -> Int4,
        commentchange -> Varchar,
        is_standard -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    component_to_keyword (id) {
        id -> Int4,
        id_component -> Int4,
        id_component_keyword -> Int4,
    }
}

table! {
    component_to_user (id) {
        id -> Int4,
        id_component -> Int4,
        id_user -> Int4,
        comment -> Varchar,
    }
}

table! {
    component_type_ref (id) {
        id -> Int4,
        component_type -> Varchar,
    }
}

table! {
    discussion_ref (id) {
        id -> Int4,
        created_at -> Timestamp,
        id_component -> Int4,
        id_user_from -> Int4,
        id_user_to -> Int4,
        comment -> Varchar,
        id_discussion_parent -> Int4,
    }
}

table! {
    extension_ref (id) {
        id -> Int4,
        extension -> Varchar,
        id_name_cad -> Int4,
    }
}

table! {
    file_ref (id) {
        id -> Int4,
        id_file -> Int4,
        id_user_create -> Int4,
        created_at -> Timestamp,
        filename -> Varchar,
        id_ext -> Int4,
        filesize -> Float8,
        path -> Varchar,
    }
}

table! {
    file_to_component (id) {
        id -> Int4,
        id_file -> Int4,
        id_component -> Int4,
    }
}

table! {
    file_to_modification (id) {
        id -> Int4,
        id_modification -> Int4,
        id_file -> Int4,
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
    name_cad_ref (id) {
        id -> Int4,
        name_cad -> Varchar,
    }
}

table! {
    param_ref (id) {
        id -> Int4,
        paramname -> Varchar,
    }
}

table! {
    param_to_component (id) {
        id -> Int4,
        id_component -> Int4,
        id_param -> Int4,
        value -> Varchar,
    }
}

table! {
    param_to_modification (id) {
        id -> Int4,
        id_modification -> Int4,
        id_param -> Int4,
        value -> Varchar,
    }
}

table! {
    param_translate_list (id) {
        id -> Int4,
        id_param -> Int4,
        id_lang -> Int4,
        param -> Varchar,
    }
}

table! {
    region_ref (id) {
        id -> Int4,
        region -> Varchar,
    }
}

table! {
    representation_type_ref (id) {
        id -> Int4,
        representation_type -> Varchar,
    }
}

table! {
    spec_ref (id) {
        id -> Int4,
        spec -> Varchar,
        id_spec_parent -> Int4,
    }
}

table! {
    spec_to_component (id) {
        id -> Int4,
        id_spec -> Int4,
        id_component -> Int4,
    }
}

table! {
    spec_to_user (id) {
        id -> Int4,
        id_spec -> Int4,
        id_user -> Int4,
    }
}

table! {
    spec_translate_list (id) {
        id -> Int4,
        id_spec -> Int4,
        id_lang -> Int4,
        spec -> Varchar,
    }
}

table! {
    type_access_ref (id) {
        id -> Int4,
        type_access -> Varchar,
    }
}

table! {
    type_of_change_ref (id) {
        id -> Int4,
        type_of_change -> Varchar,
    }
}

table! {
    type_user_ref (id) {
        id -> Int4,
        typeorg -> Varchar,
        typeorgshort -> Varchar,
    }
}

table! {
    user_history_list (id) {
        id -> Int4,
        id_user -> Int4,
        datechange -> Timestamp,
        id_type_of_change -> Int4,
        commentchange -> Varchar,
    }
}

table! {
    user_ref (id) {
        id -> Int4,
        uuid -> Uuid,
        email -> Varchar,
        email_verified -> Int4,
        psw_hash -> Bytea,
        psw_salt -> Varchar,
        id_type_user -> Int4,
        firstname -> Varchar,
        lastname -> Varchar,
        secondname -> Varchar,
        nickname -> Varchar,
        orgname -> Varchar,
        shortname -> Varchar,
        inn -> Varchar,
        phone -> Varchar,
        id_name_cad -> Int4,
        comment -> Varchar,
        address -> Varchar,
        time_zone -> Varchar,
        position -> Varchar,
        site_url -> Varchar,
        id_file_info_icon -> Int4,
        id_region -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    user_represet_ref (id) {
        id -> Int4,
        id_user -> Int4,
        id_region -> Int4,
        id_representation_type -> Int4,
        name -> Varchar,
        address -> Varchar,
        phone -> Varchar,
    }
}

table! {
    user_tokens_ref (id) {
        id -> Int4,
        id_user -> Int4,
        token -> Varchar,
        date_start -> Timestamp,
        date_end -> Timestamp,
    }
}

joinable!(component_access_to_user -> component_ref (id_component));
joinable!(component_access_to_user -> type_access_ref (id_type_access));
joinable!(component_access_to_user -> user_ref (id_user));
joinable!(component_fav_ref -> component_ref (id_component));
joinable!(component_fav_ref -> user_ref (id_user));
joinable!(component_modification_list -> actual_status_ref (id_actual_status));
joinable!(component_modification_list -> component_ref (id_component));
joinable!(component_modification_list -> name_cad_ref (id_name_cad));
joinable!(component_ref -> actual_status_ref (id_actual_status));
joinable!(component_ref -> component_type_ref (id_component_type));
joinable!(component_ref -> type_access_ref (id_type_access));
joinable!(component_ref -> user_ref (id_user));
joinable!(component_to_keyword -> component_keyword_ref (id_component_keyword));
joinable!(component_to_keyword -> component_ref (id_component));
joinable!(component_to_user -> component_ref (id_component));
joinable!(component_to_user -> user_ref (id_user));
joinable!(discussion_ref -> component_ref (id_component));
joinable!(extension_ref -> name_cad_ref (id_name_cad));
joinable!(file_ref -> extension_ref (id_ext));
joinable!(file_to_component -> component_ref (id_component));
joinable!(file_to_component -> file_ref (id_file));
joinable!(file_to_modification -> component_modification_list (id_modification));
joinable!(file_to_modification -> file_ref (id_file));
joinable!(param_to_component -> component_ref (id_component));
joinable!(param_to_component -> param_ref (id_param));
joinable!(param_to_modification -> component_modification_list (id_modification));
joinable!(param_to_modification -> param_ref (id_param));
joinable!(param_translate_list -> language_ref (id_lang));
joinable!(param_translate_list -> param_ref (id_param));
joinable!(spec_to_component -> component_ref (id_component));
joinable!(spec_to_component -> spec_ref (id_spec));
joinable!(spec_to_user -> spec_ref (id_spec));
joinable!(spec_to_user -> user_ref (id_user));
joinable!(spec_translate_list -> language_ref (id_lang));
joinable!(spec_translate_list -> spec_ref (id_spec));
joinable!(user_history_list -> type_of_change_ref (id_type_of_change));
joinable!(user_history_list -> user_ref (id_user));
joinable!(user_ref -> name_cad_ref (id_name_cad));
joinable!(user_ref -> region_ref (id_region));
joinable!(user_ref -> type_user_ref (id_type_user));
joinable!(user_represet_ref -> region_ref (id_region));
joinable!(user_represet_ref -> representation_type_ref (id_representation_type));
joinable!(user_represet_ref -> user_ref (id_user));
joinable!(user_tokens_ref -> user_ref (id_user));

allow_tables_to_appear_in_same_query!(
    actual_status_ref,
    component_access_to_user,
    component_fav_ref,
    component_keyword_ref,
    component_modification_list,
    component_ref,
    component_to_keyword,
    component_to_user,
    component_type_ref,
    discussion_ref,
    extension_ref,
    file_ref,
    file_to_component,
    file_to_modification,
    language_ref,
    name_cad_ref,
    param_ref,
    param_to_component,
    param_to_modification,
    param_translate_list,
    region_ref,
    representation_type_ref,
    spec_ref,
    spec_to_component,
    spec_to_user,
    spec_translate_list,
    type_access_ref,
    type_of_change_ref,
    type_user_ref,
    user_history_list,
    user_ref,
    user_represet_ref,
    user_tokens_ref,
);
