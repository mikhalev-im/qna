use juniper::{graphql_object, FieldResult};
use chrono::{DateTime, Utc};

use crate::schema::Context;
use crate::schema::answer::{Answer};
use crate::daos::question::{QuestionEntity};

pub struct Question {
    id: i32,
    rating: i32,
    title: String,
    content: String,
    created_at: String,
    answers: Vec<Answer>,
}

#[graphql_object(context = Context)]
impl Question {
    fn id(&self) -> i32 {
        self.id
    }

    fn rating(&self) -> i32 {
        self.rating
    }

    fn title(&self) -> &str {
        self.title.as_str()
    }

    fn content(&self) -> &str {
        self.content.as_str()
    }

    fn created_at(&self) -> &str {
        self.created_at.as_str()
    }

    async fn answers(&self, ctx: &Context) -> Vec<Answer> {
        let answers = ctx.daos.answers.load_by_question_id(self.id).await;

        let mut result = Vec::new();
        for answer in answers {
            result.push(Answer::from_entity(answer));
        }

        result
    }
}

impl Question {
    pub fn from_entity(entity: QuestionEntity) -> Question {
        Question {
            id: entity.id,
            rating: entity.rating,
            title: entity.title,
            content: entity.content,
            created_at: DateTime::<Utc>::from(entity.created_at).to_string(),
            answers: Vec::new(),
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