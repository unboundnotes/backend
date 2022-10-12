mod models;
mod repos;
mod resolvers;
mod utils;

use std::sync::Arc;

use actix_web::{guard, middleware::Logger, web, web::Data, App, HttpResponse, HttpServer};
use async_graphql::{
    extensions::{Analyzer, ApolloTracing, Logger as GQLLogger},
    http::GraphiQLSource,
    EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use dotenv::dotenv;
use log::info;
use mongodb::{options::ClientOptions, Client};
use repos::traits::UserRepo;
use resolvers::{MutationsRoot, QueryRoot};
use utils::jwt::verify_token;

use crate::{repos::mongodb_user_repo::MongoDBUserRepo, utils::config::Config};

async fn index(
    schema: web::Data<Schema<QueryRoot, MutationsRoot, EmptySubscription>>,
    db: web::Data<dyn UserRepo>,
    config: web::Data<Config>,
    req: GraphQLRequest,
    http_req: actix_web::HttpRequest,
) -> GraphQLResponse {
    let mut loggedin_user = None;
    if let Some(header_data) = http_req
        .headers()
        .get(actix_web::http::header::AUTHORIZATION)
    {
        if let Ok(header) = header_data.to_str() {
            if let Some(token) = header.split("Bearer ").last() {
                if let Ok(uuid) = verify_token(&config.jwt_secret, token) {
                    loggedin_user = db.get_user_by_uuid(&uuid).await.unwrap();
                }
            }
        }
    }

    schema
        .execute(req.into_inner().data(loggedin_user))
        .await
        .into()
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
    let config = Config {};

    let mut client_options = ClientOptions::parse(&config.mongo_uri).await.unwrap();
    client_options.app_name = Some("unboundnotes".to_string());
    let client = Client::with_options(client_options).unwrap();
    let db = client.database(&config.mongo_db);

    info!("GraphiQL IDE: http://localhost:8000");

    let user_repo = MongoDBUserRepo::new(&db);
    let userrepo_arc: Arc<dyn UserRepo> = Arc::new(user_repo);

    let config1 = config.clone();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(Data::new(
                Schema::build(
                    QueryRoot::default(),
                    MutationsRoot::default(),
                    EmptySubscription,
                )
                .extension(ApolloTracing)
                .extension(GQLLogger)
                .extension(Analyzer)
                // Instead of cloning the whole repo, use an Arc
                .data(Arc::clone(&userrepo_arc))
                .data(config.clone())
                .finish(),
            ))
            .app_data(Data::from(Arc::clone(&userrepo_arc)))
            .app_data(Data::new(config.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(gql_playgound))
    })
    .bind(&config1.bind_addr)?
    .run()
    .await
}
