table! {
    products (id) {
        id -> Int4,
        name -> Text,
        category -> Text,
    }
}

table! {
    session (id) {
        id -> Int4,
        refresh_token -> Text,
        refresh_token_expire_at -> Timestamp,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        confirmed -> Int4,
        role -> Int4,
    }
}

joinable!(session -> users (user_id));

allow_tables_to_appear_in_same_query!(
    products,
    session,
    users,
);
