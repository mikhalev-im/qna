use juniper::{GraphQLObject, FieldResult};
use super::{Context};

#[derive(GraphQLObject)]
#[graphql(description = "Answer given to a certain question")]
pub struct Answer {
    id: i32,
    question_id: i32,
    content: String,
    created_at: String,
}

pub fn resolve_answer(context: &Context, id: String) -> FieldResult<Answer> {
    let connection = context.daos.questions.load();
    Ok(Answer{
        id: 1,
        question_id: 1,
        content: String::from("content"),
        created_at: String::from("2020-01-01"),
    })
}
