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
    list_user_communities_keys (id) {
        id -> Int4,
        types -> Int2,
        name -> Varchar,
        owner -> Int4,
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
        sticker_id -> Nullable<Int4>,
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
    post_list_reposts (id) {
        id -> Int4,
        post_list_id -> Int4,
        post_id -> Nullable<Int4>,
        message_id -> Nullable<Int4>,
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
    post_reposts (id) {
        id -> Int4,
        post_id -> Nullable<Int4>,
        message_id -> Nullable<Int4>,
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
        comment_enabled -> Bool,
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
    reactions (id) {
        id -> Int4,
        image -> Varchar,
        gif -> Varchar,
        name -> Varchar,
        is_active -> Bool,
        position -> Int2,
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

joinable!(post_comments -> communitys (community_id));
joinable!(post_comments -> posts (post_id));
joinable!(post_comments -> users (user_id));
joinable!(post_list_reposts -> post_lists (post_list_id));
joinable!(post_lists -> communitys (community_id));
joinable!(post_lists -> users (user_id));
joinable!(posts -> communitys (community_id));
joinable!(posts -> post_lists (post_list_id));
joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    communities_memberships,
    community_post_list_collections,
    community_post_list_positions,
    community_post_notifications,
    community_visible_perms,
    communitys,
    featured_user_communities,
    follows,
    friends,
    list_user_communities_keys,
    news_user_communities,
    notify_user_communities,
    post_comment_counter_reactions,
    post_comment_reactions,
    post_comments,
    post_counter_reactions,
    post_list_perms,
    post_list_reposts,
    post_lists,
    post_reactions,
    post_reposts,
    posts,
    reactions,
    user_post_list_collections,
    user_post_list_positions,
    user_post_notifications,
    user_visible_perms,
    users,
);
