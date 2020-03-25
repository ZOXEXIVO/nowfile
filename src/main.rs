use crate::actions::{download_action, upload_action};
use crate::storage::S3Client;
use actix_web::{web, App, HttpServer};
use object_pool::Pool;

use std::env;
use crate::state::ApplicationState;
use std::str::FromStr;

mod state;
mod actions;
mod file;
mod storage;
mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let opts = RunOptions::from_env();

        let app_state = ApplicationState {
            storage_client: Pool::new(opts.pool_size, || {
                S3Client::new(&opts.endpoint, 
                              &opts.bucket_name, 
                              &opts.access_key,
                              &opts.secret_key)
            }),
        };

        App::new().data(app_state)
            .service(web::resource("/{file_id}").route(web::get().to(download_action))
            ).service(web::resource("/").route(web::post().to(upload_action))
        )
    }).bind("0.0.0.0:17200")?
        .run()
        .await
}

struct RunOptions {
    pub endpoint: String,
    pub bucket_name: String,
    pub access_key: String,
    pub secret_key: String,
    pub pool_size: usize
}

impl RunOptions {
    pub fn from_env() -> Self{
        RunOptions {
            endpoint: env::var("ENDPOINT").unwrap_or("http://localhost:4572".to_string()),

            bucket_name: env::var("BUCKET_NAME").unwrap_or("test-bucket".to_string()),

            access_key: env::var("ACCESS_KEY").unwrap_or("123".to_string()),
            secret_key: env::var("SECRET_KEY").unwrap_or("321".to_string()),

            pool_size:  usize::from_str(&env::var("POOL_SIZE").unwrap_or("100".to_string())).unwrap()
        }
    }
}