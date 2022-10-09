use chrono::{DateTime, Utc};
use diesel::{prelude::*, QueryDsl, Queryable};
use jsonb_diesel::{
    db::Db,
    schema::{form_question, question},
};
use uuid::Uuid;

#[derive(Queryable, Debug, PartialEq)]
pub struct FormQuestion {
    pub id: Uuid,
    pub form_id: Uuid,
    pub question_id: Uuid,
    pub order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Debug, PartialEq)]
pub struct Question {
    pub id: Uuid,
    pub text: String,
    pub input_type: String,
    pub answer_options: serde_json::Value,
    pub advanced_options: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

fn main() {
    let conn = &mut Db::new().connect().expect("Failed to get connection.");

    let result = form_question::dsl::form_question
        .inner_join(question::table)
        .load::<(FormQuestion, Question)>(conn);

    match result {
        Ok(ok) => println!("{ok:#?}"),
        Err(err) => println!("{err:#?}"),
    }
}
