// @generated automatically by Diesel CLI.

diesel::table! {
    answers (id) {
        id -> Int4,
        question_id -> Int4,
        #[max_length = 128]
        body -> Varchar,
    }
}

diesel::table! {
    histories (id) {
        id -> Int4,
        system_id -> Int4,
        user_id -> Int4,
        #[max_length = 8]
        answered_questions -> Varchar,
        results -> Json,
        started_at -> Timestamp,
        finished_at -> Timestamp,
    }
}

diesel::table! {
    questions (id) {
        id -> Int4,
        system_id -> Int4,
        #[max_length = 64]
        body -> Varchar,
        with_chooses -> Bool,
    }
}

diesel::table! {
    systems (id) {
        id -> Int4,
        user_id -> Int4,
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

diesel::joinable!(answers -> questions (question_id));
diesel::joinable!(histories -> systems (system_id));
diesel::joinable!(histories -> users (user_id));
diesel::joinable!(questions -> systems (system_id));
diesel::joinable!(systems -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    answers,
    histories,
    questions,
    systems,
    users,
);
