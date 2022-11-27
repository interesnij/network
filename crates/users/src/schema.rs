table! {
    featured_friends (id) {
        id -> Int4,
        user_id -> Int4,
        target_id -> Int4,
        hidden -> Bool,
    }
}

table! {
    follows (id) {
        id -> Int4,
        user_id -> Int4,
        target_id -> Int4,
        view -> Bool,
        visited -> Int4,
    }
}

table! {
    friends (id) {
        id -> Int4,
        user_id -> Int4,
        target_id -> Int4,
        visited -> Int4,
    }
}

table! {
    ip_users (id) {
        id -> Int4,
        user_id -> Int4,
        ip -> Varchar,
    }
}

table! {
    list_user_communities_keys (id) {
        id -> Int4,
        types -> Int2,
        name -> Varchar,
        owner -> Int4,
    }
}

table! {
    moderated_logs (id) {
        id -> Int4,
        user_id -> Int4,
        object_id -> Int4,
        action -> Int2,
        description -> Nullable<Varchar>,
        types -> Int2,
        created -> Timestamp,
        time_to_suspend -> Nullable<Timestamp>,
    }
}

table! {
    moderated_penalties (id) {
        id -> Int4,
        user_id -> Int4,
        moderated_id -> Int4,
        expiration -> Nullable<Timestamp>,
        types -> Int2,
        object_id -> Int4,
        status -> Int2,
        created -> Timestamp,
    }
}

table! {
    moderated_reports (id) {
        id -> Int4,
        user_id -> Int4,
        moderated_id -> Int4,
        description -> Nullable<Varchar>,
        types -> Int2,
        created -> Timestamp,
    }
}

table! {
    moderateds (id) {
        id -> Int4,
        description -> Nullable<Varchar>,
        verified -> Bool,
        status -> Int2,
        types -> Int2,
        object_id -> Int4,
        created -> Timestamp,
        count -> Int4,
    }
}

table! {
    owner_services (id) {
        id -> Int4,
        types -> Int2,
        name -> Varchar,
    }
}

table! {
    owner_services_items (id) {
        id -> Int4,
        owner_id -> Int4,
        service_id -> Int4,
    }
}

table! {
    owners (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        types -> Int2,
        secret_key -> Varchar,
        service_key -> Varchar,
        is_active -> Bool,
    }
}

table! {
    phone_codes (id) {
        id -> Int4,
        phone -> Varchar,
        code -> Int4,
        types -> Int2,
        accept -> Bool,
        created -> Timestamp,
    }
}

table! {
    user_blocks (id) {
        id -> Int4,
        user_id -> Int4,
        target_id -> Int4,
    }
}

table! {
    user_brother_sisters (id) {
        id -> Int4,
        user_id -> Int4,
        target_id -> Int4,
    }
}

table! {
    user_children_ones (id) {
        id -> Int4,
        user_id -> Int4,
        target_id -> Int4,
    }
}

table! {
    user_colleagues_ones (id) {
        id -> Int4,
        user_id -> Int4,
        target_id -> Int4,
    }
}

table! {
    user_dad_ones (id) {
        id -> Int4,
        user_id -> Int4,
        target_id -> Int4,
    }
}

table! {
    user_delete_anketas (id) {
        id -> Int4,
        user_id -> Int4,
        answer -> Int2,
        other -> Nullable<Varchar>,
        created -> Timestamp,
    }
}

table! {
    user_grandsons_ones (id) {
        id -> Int4,
        user_id -> Int4,
        target_id -> Int4,
    }
}

table! {
    user_infos (id) {
        id -> Int4,
        user_id -> Int4,
        avatar_id -> Nullable<Int4>,
        language -> Varchar,
        email -> Nullable<Varchar>,
        birthday -> Nullable<Date>,
        b_avatar -> Nullable<Varchar>,
        status -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
        level -> Int2,
        cover -> Nullable<Varchar>,
        created -> Timestamp,
        friends -> Int4,
        follows -> Int4,
    }
}

table! {
    user_locations (id) {
        id -> Int4,
        user_id -> Int4,
        city_ru -> Nullable<Varchar>,
        city_en -> Nullable<Varchar>,
        region_ru -> Nullable<Varchar>,
        region_en -> Nullable<Varchar>,
        country_ru -> Nullable<Varchar>,
        country_en -> Nullable<Varchar>,
    }
}

table! {
    user_love_statuss (id) {
        id -> Int4,
        user_id -> Int4,
        male_status -> Int2,
        female_status -> Int2,
    }
}

table! {
    user_mom_ones (id) {
        id -> Int4,
        user_id -> Int4,
        target_id -> Int4,
    }
}

table! {
    user_notifications (id) {
        id -> Int4,
        user_id -> Int4,
        connection_request -> Bool,
        connection_confirmed -> Bool,
        user_invite -> Bool,
    }
}

table! {
    user_partner_ones (id) {
        id -> Int4,
        user_id -> Int4,
        target_id -> Int4,
    }
}

table! {
    user_privates (id) {
        id -> Int4,
        user_id -> Int4,
        see_all -> Int2,
        see_info -> Int2,
        see_friend -> Int2,
    }
}

table! {
    user_visible_perms (id) {
        id -> Int4,
        user_id -> Int4,
        target_id -> Int4,
        types -> Int2,
    }
}

table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        phone -> Varchar,
        types -> Int2,
        is_man -> Bool,
        password -> Varchar,
        link -> Varchar,
        s_avatar -> Nullable<Varchar>,
        last_activity -> Timestamp,
    }
}

joinable!(moderated_logs -> users (user_id));
joinable!(user_notifications -> users (user_id));

allow_tables_to_appear_in_same_query!(
    featured_friends,
    follows,
    friends,
    ip_users,
    list_user_communities_keys,
    moderated_logs,
    moderated_penalties,
    moderated_reports,
    moderateds,
    owner_services,
    owner_services_items,
    owners,
    phone_codes,
    user_blocks,
    user_brother_sisters,
    user_children_ones,
    user_colleagues_ones,
    user_dad_ones,
    user_delete_anketas,
    user_grandsons_ones,
    user_infos,
    user_locations,
    user_love_statuss,
    user_mom_ones,
    user_notifications,
    user_partner_ones,
    user_privates,
    user_visible_perms,
    users,
);
