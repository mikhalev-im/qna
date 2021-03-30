use juniper::{GraphQLObject, FieldResult};
use chrono::{DateTime, Utc};

use crate::schema::Context;
use crate::daos::question::{QuestionEntity};

#[derive(GraphQLObject)]
#[graphql(description = "Question asked on a platform")]
pub struct Question {
    id: i32,
    rating: i32,
    title: String,
    content: String,
    created_at: String,
}

impl Question {
    pub fn from_entity(entity: QuestionEntity) -> Question {
        Question {
            id: entity.id,
            rating: entity.rating,
            title: entity.title,
            content: entity.content,
            created_at: DateTime::<Utc>::from(entity.created_at).to_string(),
        }
    }
}

pub async fn questions(ctx: &Context) -> FieldResult<Vec<Question>> {
    let questions = ctx.daos.questions.load().await;

    let mut result = Vec::new();
    for question in questions {
        result.push(Question::from_entity(question));
    }

    Ok(result)
}