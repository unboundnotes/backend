mod models;

use actix_web::{guard, middleware::Logger, web, web::Data, App, HttpResponse, HttpServer};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use log::info;
use models::{MutationRoot, QueryRoot};

async fn index(
    schema: web::Data<Schema<QueryRoot, MutationRoot, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint("http://localhost:8000")
                .finish(),
        )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    pretty_env_logger::init();
    info!("GraphiQL IDE: http://localhost:8000");

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(Data::new(Schema::new(
                QueryRoot,
                MutationRoot,
                EmptySubscription,
            )))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(gql_playgound))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
