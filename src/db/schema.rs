// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Uuid,
        #[max_length = 100]
        title -> Varchar,
        description -> Nullable<Text>,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_done -> Bool,
    }
}

diesel::table! {
    user (id) {
        id -> Uuid,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 90]
        email -> Varchar,
        #[max_length = 120]
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(todos -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(todos, user,);
