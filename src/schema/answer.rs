use juniper::{GraphQLObject, FieldResult};
use chrono::{DateTime, Utc};

use crate::schema::{Context};
use crate::daos::answer::{AnswerEntity};

#[derive(GraphQLObject)]
#[graphql(description = "Answer given to a certain question")]
pub struct Answer {
    pub id: i32,
    pub question_id: i32,
    pub content: String,
    pub created_at: String,
}

impl Answer {
    pub fn from_entity(entity: AnswerEntity) -> Answer {
        Answer {
            id: entity.id,
            question_id: entity.question_id,
            content: entity.content,
            created_at: DateTime::<Utc>::from(entity.created_at).to_string(),
        }
    }
}

pub async fn answers_by_question_id(ctx: &Context, question_id: i32) -> FieldResult<Vec<Answer>> {
    let answers = ctx.daos.answers.load_by_question_id(question_id).await;

    let mut result = Vec::new();
    for answer in answers {
        result.push(Answer::from_entity(answer));
    }

    Ok(result)
}
