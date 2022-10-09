// @generated automatically by Diesel CLI.

diesel::table! {
    form (id) {
        id -> Uuid,
        name -> Text,
        tag -> Text,
        version -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    form_question (id) {
        id -> Uuid,
        form_id -> Uuid,
        question_id -> Uuid,
        order -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    question (id) {
        id -> Uuid,
        text -> Text,
        input_type -> Text,
        answer_options -> Jsonb,
        advanced_options -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(form_question -> form (form_id));
diesel::joinable!(form_question -> question (question_id));

diesel::allow_tables_to_appear_in_same_query!(
    form,
    form_question,
    question,
);
