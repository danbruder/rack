table! {
    clients (id) {
        id -> Integer,
        name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    clients,
    users,
);
