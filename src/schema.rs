// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "operatorenum"))]
    pub struct Operatorenum;
}

diesel::table! {
    answers (id) {
        id -> Int4,
        question_id -> Int4,
        #[max_length = 128]
        body -> Varchar,
    }
}

diesel::table! {
    attributes (id) {
        id -> Int4,
        system_id -> Int4,
        #[max_length = 64]
        name -> Varchar,
    }
}

diesel::table! {
    attributesvalue_object (object_id, attribute_value_id, attribute_id) {
        id -> Int4,
        object_id -> Int4,
        attribute_value_id -> Int4,
        attribute_id -> Int4,
    }
}

diesel::table! {
    attributesvalues (id) {
        id -> Int4,
        attribute_id -> Int4,
        #[max_length = 64]
        value -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Operatorenum;

    clauses (id) {
        id -> Int4,
        rule_id -> Int4,
        #[max_length = 64]
        compared_value -> Varchar,
        logical_group -> Int4,
        operator -> Operatorenum,
        question_id -> Int4,
    }
}

diesel::table! {
    histories (id) {
        id -> Int4,
        system_id -> Int4,
        user_id -> Int4,
        #[max_length = 9]
        answered_questions -> Varchar,
        results -> Json,
        started_at -> Timestamp,
        finished_at -> Timestamp,
    }
}

diesel::table! {
    objects (id) {
        id -> Int4,
        system_id -> Int4,
        #[max_length = 128]
        name -> Varchar,
    }
}

diesel::table! {
    questions (id) {
        id -> Int4,
        system_id -> Int4,
        #[max_length = 128]
        body -> Varchar,
        with_chooses -> Bool,
    }
}

diesel::table! {
    rule_answer (answer_id, rule_id, question_id) {
        id -> Int4,
        answer_id -> Int4,
        rule_id -> Int4,
        question_id -> Int4,
    }
}

diesel::table! {
    rule_attributevalue (attribute_value_id, rule_id, attribute_id) {
        id -> Int4,
        attribute_value_id -> Int4,
        rule_id -> Int4,
        attribute_id -> Int4,
    }
}

diesel::table! {
    rules (id) {
        id -> Int4,
        system_id -> Int4,
        attribute_rule -> Bool,
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
        #[max_length = 128]
        image_uri -> Nullable<Varchar>,
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
        #[max_length = 256]
        password -> Varchar,
    }
}

diesel::joinable!(answers -> questions (question_id));
diesel::joinable!(attributes -> systems (system_id));
diesel::joinable!(attributesvalue_object -> attributes (attribute_id));
diesel::joinable!(attributesvalue_object -> attributesvalues (attribute_value_id));
diesel::joinable!(attributesvalue_object -> objects (object_id));
diesel::joinable!(attributesvalues -> attributes (attribute_id));
diesel::joinable!(clauses -> questions (question_id));
diesel::joinable!(clauses -> rules (rule_id));
diesel::joinable!(histories -> systems (system_id));
diesel::joinable!(histories -> users (user_id));
diesel::joinable!(objects -> systems (system_id));
diesel::joinable!(questions -> systems (system_id));
diesel::joinable!(rule_answer -> answers (answer_id));
diesel::joinable!(rule_answer -> questions (question_id));
diesel::joinable!(rule_answer -> rules (rule_id));
diesel::joinable!(rule_attributevalue -> attributes (attribute_id));
diesel::joinable!(rule_attributevalue -> attributesvalues (attribute_value_id));
diesel::joinable!(rule_attributevalue -> rules (rule_id));
diesel::joinable!(rules -> systems (system_id));
diesel::joinable!(systems -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    answers,
    attributes,
    attributesvalue_object,
    attributesvalues,
    clauses,
    histories,
    objects,
    questions,
    rule_answer,
    rule_attributevalue,
    rules,
    systems,
    users,
);
