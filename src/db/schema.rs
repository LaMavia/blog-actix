table! {
    posts (id) {
        id -> Int4,
        author -> Int4,
        date -> Varchar,
        title -> Varchar,
        body -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        login -> Varchar,
        password -> Varchar,
    }
}

joinable!(posts -> users (author));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
