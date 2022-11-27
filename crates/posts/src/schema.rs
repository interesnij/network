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
    community_post_list_collections (id) {
        id -> Int4,
        community_id -> Int4,
        post_list_id -> Int4,
    }
}

table! {
    community_post_list_positions (id) {
        id -> Int4,
        community_id -> Int4,
        list_id -> Int4,
        position -> Int2,
        types -> Int2,
    }
}

table! {
    community_post_notifications (id) {
        id -> Int4,
        community_id -> Int4,
        comment -> Bool,
        comment_reply -> Bool,
        mention -> Bool,
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
        see_member -> Int2,
        see_el -> Int2,
        see_comment -> Int2,
        create_list -> Int2,
        create_el -> Int2,
        create_comment -> Int2,
        copy_el -> Int2,
        lists -> Int4,
        posts -> Int4,
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
        see_member -> Int2,
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
    item_photos (id) {
        id -> Int4,
        user_id -> Int4,
        community_id -> Nullable<Int4>,
        list_id -> Int4,
        item_id -> Int4,
        preview -> Varchar,
        file -> Varchar,
        types -> Int2,
    }
}

table! {
    item_reposts (id) {
        id -> Int4,
        item_id -> Int4,
        item_types -> Int2,
        post_id -> Nullable<Int4>,
        message_id -> Nullable<Int4>,
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
        see_friend -> Int2,
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
    post_comment_counter_reactions (id) {
        id -> Int4,
        post_comment_id -> Int4,
        reaction_id -> Int4,
        count -> Int4,
    }
}

table! {
    post_comment_reactions (id) {
        id -> Int4,
        user_id -> Int4,
        post_comment_id -> Int4,
        reaction_id -> Int4,
    }
}

table! {
    post_comments (id) {
        id -> Int4,
        post_id -> Int4,
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
    post_counter_reactions (id) {
        id -> Int4,
        post_id -> Int4,
        reaction_id -> Int4,
        count -> Int4,
    }
}

table! {
    post_list_perms (id) {
        id -> Int4,
        user_id -> Int4,
        post_list_id -> Int4,
        types -> Int2,
    }
}

table! {
    post_lists (id) {
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
    post_reactions (id) {
        id -> Int4,
        user_id -> Int4,
        post_id -> Int4,
        reaction_id -> Int4,
    }
}

table! {
    posts (id) {
        id -> Int4,
        content -> Nullable<Varchar>,
        community_id -> Nullable<Int4>,
        user_id -> Int4,
        post_list_id -> Int4,
        types -> Int2,
        attach -> Nullable<Varchar>,
        comments_on -> Bool,
        created -> Timestamp,
        comment -> Int4,
        view -> Int4,
        repost -> Int4,
        copy -> Int4,
        position -> Int2,
        is_signature -> Bool,
        parent_id -> Nullable<Int4>,
        reactions -> Int4,
    }
}

table! {
    user_post_list_collections (id) {
        id -> Int4,
        user_id -> Int4,
        post_list_id -> Int4,
    }
}

table! {
    user_post_list_positions (id) {
        id -> Int4,
        user_id -> Int4,
        list_id -> Int4,
        position -> Int2,
        types -> Int2,
    }
}

table! {
    user_post_notifications (id) {
        id -> Int4,
        user_id -> Int4,
        comment -> Bool,
        comment_reply -> Bool,
        mention -> Bool,
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
        link -> Varchar,
        s_avatar -> Nullable<Varchar>,
        last_activity -> Timestamp,
        see_all -> Int2,
        see_friend -> Int2,
        see_el -> Int2,
        see_comment -> Int2,
        create_el -> Int2,
        create_comment -> Int2,
        copy_el -> Int2,
        lists -> Int4,
        posts -> Int4,
        comments -> Int4,
    }
}

joinable!(item_reposts -> posts (post_id));
joinable!(moderated_logs -> users (user_id));
joinable!(post_comments -> communitys (community_id));
joinable!(post_comments -> posts (post_id));
joinable!(post_comments -> users (user_id));
joinable!(post_lists -> communitys (community_id));
joinable!(post_lists -> users (user_id));
joinable!(posts -> communitys (community_id));
joinable!(posts -> post_lists (post_list_id));
joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    attach_items,
    communities_memberships,
    community_post_list_collections,
    community_post_list_positions,
    community_post_notifications,
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
    item_photos,
    item_reposts,
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
    owners,
    perms_lists,
    post_comment_counter_reactions,
    post_comment_reactions,
    post_comments,
    post_counter_reactions,
    post_list_perms,
    post_lists,
    post_reactions,
    posts,
    user_post_list_collections,
    user_post_list_positions,
    user_post_notifications,
    user_visible_perms,
    users,
);
