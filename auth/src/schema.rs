table! {
    products (id) {
        id -> Int4,
        name -> Text,
        category -> Text,
    }
}

table! {
    users (email) {
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    products,
    users,
);
