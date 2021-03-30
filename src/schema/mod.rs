use juniper::{graphql_object, FieldResult, EmptySubscription};
use actix_web::{web};
use crate::daos::{Daos};

pub mod answer;
pub mod question;

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn questions(context: &Context) -> FieldResult<Vec<question::Question>> {
        question::questions(context).await
    }

    async fn answers(context: &Context, question_id: i32) -> FieldResult<Vec<answer::Answer>> {
        answer::answers_by_question_id(context, question_id).await
    }
}

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    fn createAnswer(context: &Context) -> FieldResult<answer::Answer> {
        Ok(answer::Answer{
            id: 2,
            question_id: 1,
            content: String::from("new content"),
            created_at: String::from("2020-01-01"),
        })
    }
}

pub struct Context {
    daos: web::Data<Daos>,
}

impl Context {
    pub fn new(daos: web::Data<Daos>) -> Context {
        Context {daos}
    }
}

impl juniper::Context for Context {}

pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn new() -> Schema {
    Schema::new(
        Query,
        Mutation,
        EmptySubscription::<Context>::new(),
    )
}