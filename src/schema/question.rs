use juniper::{ GraphQLObject, graphql_object, FieldResult };
use super::Context;

#[derive(GraphQLObject)]
#[graphql(description = "Question asked on a platform")]
struct Question {
    id: i32,
    rating: i32,
    title: String,
    content: String,
    created_at: String,
}