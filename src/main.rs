use actix_web::{web, App, Error, HttpResponse, HttpServer};
use juniper_actix::{graphiql_handler, graphql_handler, playground_handler};

mod schema;
mod daos;

use schema::{Context, Schema};
use daos::{Daos};

async fn graphiql_route() -> Result<HttpResponse, Error> {
    graphiql_handler("/graphgl", None).await
}
async fn playground_route() -> Result<HttpResponse, Error> {
    playground_handler("/graphgl", None).await
}
async fn graphql_route(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<Schema>,
    daos: web::Data<Daos>,
) -> Result<HttpResponse, Error> {
    let context = Context::new(daos);
    graphql_handler(&schema, &context, req, payload).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let daos = daos::new();

    let server = HttpServer::new(move || {
        App::new()
            .data(schema::new())
            .data(daos.clone())
            .service(
                web::resource("/graphgl")
                    .route(web::post().to(graphql_route))
                    .route(web::get().to(graphql_route)),
            )
            .service(web::resource("/playground").route(web::get().to(playground_route)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql_route)))
    });
    server.bind("127.0.0.1:8080").unwrap().run().await
}
