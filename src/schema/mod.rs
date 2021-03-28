use juniper::{graphql_object, FieldResult, EmptySubscription};
use super::daos::{Daos};

mod answer;
mod question;

use answer::{Answer};

struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn apiVersion() -> &str {
        "1.0"
    }

    fn answer(context: &Context, id: String) -> FieldResult<Answer> {
        answer::resolve_answer(context, id)
    }
}

struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    fn createAnswer(context: &Context) -> FieldResult<Answer> {
        Ok(Answer{
            id: 2,
            question_id: 1,
            content: String::from("new content"),
            created_at: String::from("2020-01-01"),
        })
    }
}

pub struct Context {
    daos: Daos,
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