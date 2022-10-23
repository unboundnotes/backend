mod models;
mod repos;
mod resolvers;
mod schema;
mod utils;

use std::sync::Arc;

use crate::{
    repos::{
        postgres_users_repo::PostgresqlUsersRepo,
        postgres_workspace_repo::PostgresqlWorkspaceRepo,
        s3_images_repo::S3ImagesRepo,
        traits::{ImagesRepo, WorkspaceRepo},
    },
    utils::{
        config::{BaseConfig, Config},
        postgresql_data_source::PostgresqlDataSource,
    },
};
use actix_cors::Cors;
use actix_web::{guard, http, middleware::Logger, web, web::Data, App, HttpResponse, HttpServer};
use appconfig_derive::NopDataSource;
use async_graphql::{
    extensions::{Analyzer, ApolloTracing, Logger as GQLLogger},
    http::GraphiQLSource,
    EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use dotenvy::dotenv;
use log::info;
use repos::traits::UserRepo;
use resolvers::{MutationsRoot, QueryRoot};

async fn index(
    schema: web::Data<Schema<QueryRoot, MutationsRoot, EmptySubscription>>,
    // db: web::Data<dyn UserRepo>,
    // config: web::Data<BaseConfig>,
    req: GraphQLRequest,
    // http_req: actix_web::HttpRequest,
) -> GraphQLResponse {
    // let mut loggedin_user = None;
    // if let Some(header_data) = http_req
    //     .headers()
    //     .get(actix_web::http::header::AUTHORIZATION)
    // {
    //     if let Ok(header) = header_data.to_str() {
    //         if let Some(token) = header.split("Bearer ").last() {
    //             if let Ok(uuid) = verify_token(&config.jwt_secret, token) {
    //                 loggedin_user = db.get_user_by_uuid(&uuid).await.unwrap();
    //             }
    //         }
    //     }
    // }

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
    let base_config = BaseConfig::build(&mut NopDataSource {}, None)
        .await
        .unwrap();
    let mut psql_ds = PostgresqlDataSource::new(&base_config.database_url)
        .await
        .unwrap();
    let config = Config::build(&mut psql_ds, None, base_config)
        .await
        .unwrap();
    let config = Arc::new(config);

    let manager = ConnectionManager::<PgConnection>::new(&config.base.database_url);
    let pool = Pool::new(manager).unwrap();

    info!("GraphiQL IDE: http://localhost:8000");

    let config_clone = Arc::clone(&config);

    HttpServer::new(move || {
        let logger = Logger::default();
        let pool = pool.clone();
        let user_repo = PostgresqlUsersRepo::new(pool.clone());
        let userrepo_arc: Arc<dyn UserRepo> = Arc::new(user_repo);
        let workspace_repo = PostgresqlWorkspaceRepo::new(pool.clone());
        let workspacerepo_arc: Arc<dyn WorkspaceRepo> = Arc::new(workspace_repo);
        let s3_images_repo = S3ImagesRepo::new(
            &config_clone.s3_bucket.clone(),
            &config_clone.s3_endpoint.clone(),
        )
        .unwrap();
        let s3_images_repo_arc: Arc<dyn ImagesRepo> = Arc::new(s3_images_repo);

        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
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
                .data(Arc::clone(&userrepo_arc))
                .data(Arc::clone(&config_clone))
                .data(Arc::clone(&workspacerepo_arc))
                .data(Arc::clone(&s3_images_repo_arc))
                .finish(),
            ))
            .app_data(Data::from(Arc::clone(&userrepo_arc)))
            .app_data(Data::from(Arc::clone(&config_clone)))
            .app_data(Data::from(Arc::clone(&workspacerepo_arc)))
            .app_data(Data::from(Arc::clone(&s3_images_repo_arc)))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(gql_playgound))
    })
    .bind(&config.base.bind_addr.clone())?
    .run()
    .await
}
