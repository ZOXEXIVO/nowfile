use crate::actions::{download_action, upload_action};
use crate::storage::S3Client;
use actix_web::{web, App, HttpServer};
use object_pool::Pool;

use std::env;
use std::str::FromStr;
use crate::models::{ApplicationState};
use crate::logging::Logger;

use slog::*;

mod models;
mod digest;
mod actions;
mod storage;
mod utils;
mod logging;
 
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {  
        let logger = Logger::init("nowfile");
        let application_state = create_app_state(RunOptions::from_env(), logger);
        
        App::new().data(application_state)
            .service(web::resource("/{file_id}").route(web::get().to(download_action))
            ).service(web::resource("/").route(web::post().to(upload_action))
        )
    }).bind("0.0.0.0:17200")?
        .run()
        .await
}

fn create_app_state<'s>(options: RunOptions, logger: slog::Logger) -> ApplicationState<'s> {
    info!(logger, "nowfile started {0}, {1}", &options.endpoint, &options.bucket_name);
    
    ApplicationState {
        storage_client_pool: Pool::new(options.pool_size, || {
            S3Client::new(&options.endpoint,
                          &options.bucket_name,
                          &options.access_key,
                          &options.secret_key)            
        }),
        logger,
        options
    }
}

pub struct RunOptions {
    pub endpoint: String,
    pub bucket_name: String,
    pub access_key: String,
    pub secret_key: String,
    pub token_key: String,
    pub pool_size: usize
}

impl RunOptions {
    pub fn from_env() -> Self{
        RunOptions {
            endpoint: env::var("ENDPOINT").unwrap_or("http://localhost:4572".to_string()),

            bucket_name: env::var("BUCKET_NAME").unwrap_or("test-bucket".to_string()),

            access_key: env::var("ACCESS_KEY").unwrap_or("123".to_string()),
            secret_key: env::var("SECRET_KEY").unwrap_or("321".to_string()),

            token_key: env::var("TOKEN_KEY").unwrap_or("123456789".to_string()),

            pool_size:  usize::from_str(&env::var("POOL_SIZE").unwrap_or("50".to_string())).unwrap()
        }
    }
}