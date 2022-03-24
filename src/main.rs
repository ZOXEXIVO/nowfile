use crate::actions::{download_action, upload_action};
use crate::storage::S3Client;
use actix_web::{web, App, HttpResponse, HttpServer};
use object_pool::Pool;

use crate::models::ApplicationState;
use env_logger::Env;
use std::env;
use std::str::FromStr;
use actix_web::web::Data;

mod actions;
mod digest;
mod models;
mod storage;
mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(move || {
        let application_state = create_app_state(RunOptions::from_env());

        App::new()
            .app_data(Data::new(application_state))
            .service(
                web::resource("/favicon.ico").route(web::get().to(|| HttpResponse::NotFound())),
            )
            .service(web::resource("/{file_id}").route(web::get().to(download_action)))
            .service(web::resource("/").route(web::post().to(upload_action)))
    })
    .bind("0.0.0.0:17200")?
    .run()
    .await
}

fn create_app_state<'s>(options: RunOptions) -> ApplicationState {
    ApplicationState {
        storage_client_pool: Pool::new(options.pool_size, || {
            S3Client::new(
                &options.endpoint,
                &options.bucket_name,
                &options.access_key,
                &options.secret_key,
            )
        }),
        options,
    }
}

// TODO: clap
pub struct RunOptions {
    pub endpoint: String,
    pub bucket_name: String,
    pub access_key: String,
    pub secret_key: String,
    pub pool_size: usize,
}

impl RunOptions {
    pub fn from_env() -> Self {
        RunOptions {
            endpoint: env::var("ENDPOINT").unwrap_or_else(|_| "http://localhost:4572".to_string()),

            bucket_name: env::var("BUCKET_NAME").unwrap_or_else(|_| "test-bucket".to_string()),

            access_key: env::var("ACCESS_KEY").unwrap_or_else(|_| "123".to_string()),
            secret_key: env::var("SECRET_KEY").unwrap_or_else(|_| "321".to_string()),

            pool_size: usize::from_str(&env::var("POOL_SIZE").unwrap_or_else(|_| "50".to_string()))
                .unwrap(),
        }
    }
}
