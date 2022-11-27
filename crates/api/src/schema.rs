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
    smile_categories (id) {
        id -> Int4,
        name -> Varchar,
        position -> Int2,
        description -> Nullable<Varchar>,
    }
}

table! {
    smiles (id) {
        id -> Int4,
        name -> Varchar,
        position -> Int2,
        category_id -> Int4,
        image -> Varchar,
    }
}

table! {
    sticker_categories (id) {
        id -> Int4,
        name -> Varchar,
        position -> Int2,
        user_id -> Nullable<Int4>,
        description -> Nullable<Varchar>,
        avatar -> Nullable<Varchar>,
    }
}

table! {
    stickers (id) {
        id -> Int4,
        name -> Varchar,
        position -> Int2,
        category_id -> Int4,
        image -> Varchar,
    }
}

table! {
    user_populate_smiles (id) {
        id -> Int4,
        user_id -> Int4,
        smile_id -> Int4,
        count -> Int4,
        image -> Varchar,
    }
}

table! {
    user_populate_stickers (id) {
        id -> Int4,
        user_id -> Int4,
        sticker_id -> Int4,
        count -> Int4,
        image -> Varchar,
    }
}

joinable!(smiles -> smile_categories (category_id));
joinable!(stickers -> sticker_categories (category_id));

allow_tables_to_appear_in_same_query!(
    reactions,
    smile_categories,
    smiles,
    sticker_categories,
    stickers,
    user_populate_smiles,
    user_populate_stickers,
);
