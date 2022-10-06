mod models;
mod repos;

use actix_web::{guard, middleware::Logger, web, web::Data, App, HttpResponse, HttpServer};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use dotenv::dotenv;
use log::info;
use models::{MutationRoot, QueryRoot};
use mongodb::{options::ClientOptions, Client};
use std::env;

use crate::repos::mongodb_user_repo::MongoDBUserRepo;

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
    dotenv().ok();
    pretty_env_logger::init();

    let mut client_options = ClientOptions::parse(&env::var("MONGODB_URI").unwrap())
        .await
        .unwrap();
    client_options.app_name = Some("unboundnotes".to_string());
    let client = Client::with_options(client_options).unwrap();
    let db = client.database(&env::var("MONGODB_DB").unwrap());

    info!("GraphiQL IDE: http://localhost:8000");

    let user_repo = MongoDBUserRepo::new(&db).await;

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(Data::new(Schema::new(
                QueryRoot {
                    user_repo: user_repo.clone(),
                },
                MutationRoot {
                    user_repo: user_repo.clone(),
                },
                EmptySubscription,
            )))
            .app_data(Data::new(user_repo.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(gql_playgound))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
