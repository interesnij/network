table! {
    communities_memberships (id) {
        id -> Int4,
        user_id -> Int4,
        community_id -> Int4,
        level -> Int2,
        created -> Timestamp,
        visited -> Int2,
    }
}

table! {
    community_banned_users (id) {
        id -> Int4,
        community_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    community_categorys (id) {
        id -> Int4,
        name -> Varchar,
        avatar -> Nullable<Varchar>,
        position -> Int2,
    }
}

table! {
    community_follows (id) {
        id -> Int4,
        user_id -> Int4,
        community_id -> Int4,
        view -> Bool,
        visited -> Int2,
    }
}

table! {
    community_infos (id) {
        id -> Int4,
        community_id -> Int4,
        avatar_id -> Nullable<Int4>,
        b_avatar -> Nullable<Varchar>,
        status -> Nullable<Varchar>,
        level -> Int2,
        cover -> Nullable<Varchar>,
        created -> Timestamp,
        description -> Nullable<Varchar>,
    }
}

table! {
    community_invites (id) {
        id -> Int4,
        user_id -> Int4,
        community_id -> Int4,
        invite_creator -> Int4,
    }
}

table! {
    community_notifications (id) {
        id -> Int4,
        community_id -> Int4,
        connection_request -> Bool,
        connection_confirmed -> Bool,
        community_invite -> Bool,
    }
}

table! {
    community_privates (id) {
        id -> Int4,
        community_id -> Int4,
        see_member -> Int2,
        see_info -> Int2,
        see_settings -> Int2,
        see_log -> Int2,
        see_stat -> Int2,
    }
}

table! {
    community_subcategorys (id) {
        id -> Int4,
        name -> Varchar,
        category_id -> Int4,
        avatar -> Nullable<Varchar>,
        position -> Int2,
    }
}

table! {
    community_visible_perms (id) {
        id -> Int4,
        community_id -> Int4,
        target_id -> Int4,
        types -> Int2,
    }
}

table! {
    communitys (id) {
        id -> Int4,
        name -> Varchar,
        status -> Nullable<Varchar>,
        types -> Int2,
        link -> Varchar,
        s_avatar -> Nullable<Varchar>,
        category_id -> Int4,
        user_id -> Int4,
        members -> Int4,
    }
}

table! {
    featured_communities (id) {
        id -> Int4,
        user_id -> Int4,
        community_id -> Int4,
        hidden -> Bool,
    }
}

table! {
    follows (id) {
        id -> Int4,
        user_id -> Int4,
        target_id -> Int4,
    }
}

table! {
    friends (id) {
        id -> Int4,
        user_id -> Int4,
        target_id -> Int4,
    }
}

table! {
    moderated_logs (id) {
        id -> Int4,
        user_id -> Int4,
        community_id -> Int4,
        action -> Int2,
        description -> Nullable<Varchar>,
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
        community_id -> Int4,
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
        community_id -> Int4,
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
        community_id -> Nullable<Int4>,
        name -> Varchar,
        description -> Nullable<Varchar>,
        types -> Int2,
        secret_key -> Varchar,
        service_key -> Varchar,
        is_active -> Bool,
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
        user_id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        types -> Int2,
        is_man -> Bool,
        password -> Varchar,
        link -> Varchar,
        s_avatar -> Nullable<Varchar>,
        last_activity -> Timestamp,
        see_all -> Int2,
        see_community -> Int2,
        communities -> Int4,
    }
}

joinable!(moderated_logs -> users (user_id));

allow_tables_to_appear_in_same_query!(
    communities_memberships,
    community_banned_users,
    community_categorys,
    community_follows,
    community_infos,
    community_invites,
    community_notifications,
    community_privates,
    community_subcategorys,
    community_visible_perms,
    communitys,
    featured_communities,
    follows,
    friends,
    moderated_logs,
    moderated_penalties,
    moderated_reports,
    moderateds,
    owner_services,
    owner_services_items,
    owners,
    user_visible_perms,
    users,
);
