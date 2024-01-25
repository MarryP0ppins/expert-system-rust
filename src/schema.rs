// @generated automatically by Diesel CLI.

diesel::table! {
    histories (id) {
        id -> Int4,
        system -> Int4,
        user -> Int4,
        #[max_length = 8]
        answered_questions -> Varchar,
        results -> Json,
    }
}

diesel::table! {
    questions (id) {
        id -> Int4,
        system -> Int4,
        #[max_length = 64]
        body -> Varchar,
        with_chooses -> Bool,
    }
}

diesel::table! {
    systems (id) {
        id -> Int4,
        user -> Int4,
        about -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 128]
        name -> Varchar,
        private -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 32]
        email -> Varchar,
        #[max_length = 16]
        username -> Varchar,
        created_at -> Timestamp,
        #[max_length = 16]
        first_name -> Varchar,
        #[max_length = 16]
        last_name -> Varchar,
        is_superuser -> Bool,
        #[max_length = 16]
        password -> Varchar,
    }
}

diesel::joinable!(histories -> systems (system));
diesel::joinable!(histories -> users (user));
diesel::joinable!(questions -> systems (system));
diesel::joinable!(systems -> users (user));

diesel::allow_tables_to_appear_in_same_query!(
    histories,
    questions,
    systems,
    users,
);
