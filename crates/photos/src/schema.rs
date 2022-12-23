table! {
    attach_items (id) {
        id -> Int4,
        item_id -> Int4,
        item_types -> Int2,
        attach_types -> Int2,
    }
}

table! {
    communities_memberships (id) {
        id -> Int4,
        user_id -> Int4,
        community_id -> Int4,
        level -> Int2,
    }
}

table! {
    community_photo_list_collections (id) {
        id -> Int4,
        community_id -> Int4,
        photo_list_id -> Int4,
    }
}

table! {
    community_photo_list_positions (id) {
        id -> Int4,
        community_id -> Int4,
        list_id -> Int4,
        position -> Int2,
        types -> Int2,
    }
}

table! {
    community_photo_notifications (id) {
        id -> Int4,
        community_id -> Int4,
        comment -> Bool,
        comment_reply -> Bool,
        comment_mention -> Bool,
        repost -> Bool,
        reactions -> Bool,
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
        community_id -> Int4,
        user_id -> Int4,
        name -> Varchar,
        types -> Int2,
        link -> Varchar,
        s_avatar -> Nullable<Varchar>,
        see_el -> Int2,
        see_comment -> Int2,
        create_list -> Int2,
        create_el -> Int2,
        create_comment -> Int2,
        copy_el -> Int2,
        lists -> Int4,
        photos -> Int4,
        comments -> Int4,
    }
}

table! {
    featured_user_communities (id) {
        id -> Int4,
        owner -> Int4,
        list_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        community_id -> Nullable<Int4>,
        mute -> Bool,
        sleep -> Nullable<Timestamp>,
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
    item_articles (id) {
        id -> Int4,
        title -> Varchar,
        user_id -> Int4,
        community_id -> Nullable<Int4>,
        list_id -> Int4,
        item_id -> Int4,
        types -> Int2,
        image -> Nullable<Varchar>,
    }
}

table! {
    item_audios (id) {
        id -> Int4,
        title -> Varchar,
        user_id -> Int4,
        community_id -> Nullable<Int4>,
        list_id -> Int4,
        item_id -> Int4,
        artist_id -> Nullable<Int4>,
        types -> Int2,
        file -> Varchar,
        image -> Nullable<Varchar>,
    }
}

table! {
    item_comments (id) {
        id -> Int4,
        user_id -> Int4,
        community_id -> Nullable<Int4>,
        content -> Nullable<Varchar>,
        attach -> Nullable<Varchar>,
        comment_id -> Int4,
        comment_types -> Int2,
        item_id -> Int4,
        item_types -> Int2,
        types -> Int2,
        created -> Timestamp,
    }
}

table! {
    item_communitys (id) {
        id -> Int4,
        community_id -> Int4,
        name -> Varchar,
        types -> Int2,
        link -> Varchar,
        s_avatar -> Nullable<Varchar>,
    }
}

table! {
    item_docs (id) {
        id -> Int4,
        title -> Varchar,
        user_id -> Int4,
        community_id -> Nullable<Int4>,
        list_id -> Int4,
        item_id -> Int4,
        types -> Int2,
        file -> Varchar,
    }
}

table! {
    item_forums (id) {
        id -> Int4,
        title -> Varchar,
        user_id -> Int4,
        community_id -> Nullable<Int4>,
        list_id -> Int4,
        item_id -> Int4,
        types -> Int2,
    }
}

table! {
    item_goods (id) {
        id -> Int4,
        title -> Varchar,
        user_id -> Int4,
        community_id -> Nullable<Int4>,
        list_id -> Int4,
        item_id -> Int4,
        price -> Nullable<Int4>,
        types -> Int2,
        image -> Nullable<Varchar>,
    }
}

table! {
    item_lists (id) {
        id -> Int4,
        name -> Varchar,
        user_id -> Int4,
        community_id -> Nullable<Int4>,
        list_id -> Int4,
        list_types -> Int2,
        types -> Int2,
        image -> Nullable<Varchar>,
        count -> Int4,
        see_el -> Int2,
        copy_el -> Int2,
    }
}

table! {
    item_posts (id) {
        id -> Int4,
        content -> Nullable<Varchar>,
        list_id -> Int4,
        community_id -> Nullable<Int4>,
        user_id -> Int4,
        item_id -> Int4,
        types -> Int2,
        attach -> Nullable<Varchar>,
        created -> Timestamp,
        is_signature -> Bool,
    }
}

table! {
    item_sites (id) {
        id -> Int4,
        title -> Varchar,
        user_id -> Int4,
        community_id -> Nullable<Int4>,
        item_id -> Int4,
        types -> Int2,
    }
}

table! {
    item_surveys (id) {
        id -> Int4,
        title -> Varchar,
        user_id -> Int4,
        community_id -> Nullable<Int4>,
        list_id -> Int4,
        item_id -> Int4,
        types -> Int2,
        image -> Nullable<Varchar>,
        is_anonymous -> Bool,
        is_multiple -> Bool,
        is_no_edited -> Bool,
        time_end -> Nullable<Timestamp>,
        vote -> Int4,
    }
}

table! {
    item_users (id) {
        id -> Int4,
        user_id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        types -> Int2,
        link -> Varchar,
        s_avatar -> Nullable<Varchar>,
        see_all -> Int2,
    }
}

table! {
    item_videos (id) {
        id -> Int4,
        title -> Varchar,
        user_id -> Int4,
        community_id -> Nullable<Int4>,
        list_id -> Int4,
        item_id -> Int4,
        types -> Int2,
        image -> Nullable<Varchar>,
        file -> Varchar,
        view -> Int4,
    }
}

table! {
    item_wikis (id) {
        id -> Int4,
        title -> Varchar,
        user_id -> Int4,
        community_id -> Nullable<Int4>,
        list_id -> Int4,
        item_id -> Int4,
        types -> Int2,
        image -> Nullable<Varchar>,
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
    news_user_communities (id) {
        id -> Int4,
        owner -> Int4,
        list_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        community_id -> Nullable<Int4>,
        mute -> Bool,
        sleep -> Nullable<Timestamp>,
    }
}

table! {
    notify_user_communities (id) {
        id -> Int4,
        owner -> Int4,
        list_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        community_id -> Nullable<Int4>,
        mute -> Bool,
        sleep -> Nullable<Timestamp>,
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
        types -> Int2,
        secret_key -> Varchar,
        service_key -> Varchar,
        is_active -> Bool,
    }
}

table! {
    perms_lists (id) {
        id -> Int4,
        user_id -> Int4,
        community_id -> Nullable<Int4>,
        list_id -> Int4,
        list_types -> Int2,
        types -> Int2,
        see_el -> Int2,
        copy_el -> Int2,
    }
}

table! {
    photo_comment_counter_reactions (id) {
        id -> Int4,
        photo_comment_id -> Int4,
        reaction_id -> Int4,
        count -> Int4,
    }
}

table! {
    photo_comment_reactions (id) {
        id -> Int4,
        user_id -> Int4,
        photo_comment_id -> Int4,
        reaction_id -> Int4,
    }
}

table! {
    photo_comments (id) {
        id -> Int4,
        photo_id -> Int4,
        user_id -> Int4,
        community_id -> Nullable<Int4>,
        parent_id -> Nullable<Int4>,
        content -> Nullable<Varchar>,
        attach -> Nullable<Varchar>,
        types -> Int2,
        created -> Timestamp,
        repost -> Int4,
        reactions -> Int4,
        replies -> Int4,
    }
}

table! {
    photo_counter_reactions (id) {
        id -> Int4,
        photo_id -> Int4,
        reaction_id -> Int4,
        count -> Int4,
    }
}

table! {
    photo_list_perms (id) {
        id -> Int4,
        user_id -> Int4,
        photo_list_id -> Int4,
        types -> Int2,
    }
}

table! {
    photo_lists (id) {
        id -> Int4,
        name -> Varchar,
        community_id -> Nullable<Int4>,
        user_id -> Int4,
        types -> Int2,
        description -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        created -> Timestamp,
        count -> Int4,
        repost -> Int4,
        copy -> Int4,
        see_el -> Int2,
        see_comment -> Int2,
        create_el -> Int2,
        create_comment -> Int2,
        copy_el -> Int2,
        reactions -> Nullable<Varchar>,
    }
}

table! {
    photo_reactions (id) {
        id -> Int4,
        user_id -> Int4,
        photo_id -> Int4,
        reaction_id -> Int4,
    }
}

table! {
    photos (id) {
        id -> Int4,
        community_id -> Nullable<Int4>,
        user_id -> Int4,
        photo_list_id -> Int4,
        types -> Int2,
        server_id -> Int2,
        file -> Varchar,
        description -> Nullable<Varchar>,
        comments_on -> Bool,
        created -> Timestamp,
        comment -> Int4,
        view -> Int4,
        repost -> Int4,
        copy -> Int4,
        position -> Int2,
        reactions -> Int4,
    }
}

table! {
    user_photo_list_collections (id) {
        id -> Int4,
        user_id -> Int4,
        photo_list_id -> Int4,
    }
}

table! {
    user_photo_list_positions (id) {
        id -> Int4,
        user_id -> Int4,
        list_id -> Int4,
        position -> Int2,
        types -> Int2,
    }
}

table! {
    user_photo_notifications (id) {
        id -> Int4,
        user_id -> Int4,
        comment -> Bool,
        comment_reply -> Bool,
        comment_mention -> Bool,
        repost -> Bool,
        reactions -> Bool,
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
        see_el -> Int2,
        see_comment -> Int2,
        create_el -> Int2,
        create_comment -> Int2,
        copy_el -> Int2,
        lists -> Int4,
        photos -> Int4,
        comments -> Int4,
    }
}

joinable!(moderated_logs -> users (user_id));
joinable!(photo_comments -> communitys (community_id));
joinable!(photo_comments -> photos (photo_id));
joinable!(photo_comments -> users (user_id));
joinable!(photo_lists -> communitys (community_id));
joinable!(photo_lists -> users (user_id));
joinable!(photos -> communitys (community_id));
joinable!(photos -> photo_lists (photo_list_id));
joinable!(photos -> users (user_id));

allow_tables_to_appear_in_same_query!(
    attach_items,
    communities_memberships,
    community_photo_list_collections,
    community_photo_list_positions,
    community_photo_notifications,
    community_visible_perms,
    communitys,
    featured_user_communities,
    follows,
    friends,
    item_articles,
    item_audios,
    item_comments,
    item_communitys,
    item_docs,
    item_forums,
    item_goods,
    item_lists,
    item_posts,
    item_sites,
    item_surveys,
    item_users,
    item_videos,
    item_wikis,
    list_user_communities_keys,
    moderated_logs,
    moderated_penalties,
    moderated_reports,
    moderateds,
    news_user_communities,
    notify_user_communities,
    owner_services,
    owner_services_items,
    owners,
    perms_lists,
    photo_comment_counter_reactions,
    photo_comment_reactions,
    photo_comments,
    photo_counter_reactions,
    photo_list_perms,
    photo_lists,
    photo_reactions,
    photos,
    user_photo_list_collections,
    user_photo_list_positions,
    user_photo_notifications,
    user_visible_perms,
    users,
);
