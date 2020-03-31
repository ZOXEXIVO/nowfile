use crate::utils::{PathUtils};
use actix_multipart::Multipart;
use actix_web::web::Data;
use actix_web::{Error, HttpResponse, Result, HttpRequest};

use futures::{StreamExt, TryStreamExt};
use std::io::Write;
use crate::models::{ApplicationState, FileMetadata};

use slog::*;

pub async fn upload_action(
    request: HttpRequest,
    state: Data<ApplicationState<'_>>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    let mut payload_data = payload.try_next().await.unwrap().unwrap();

    let mut result_file_content = Vec::new();

    while let Some(chunk) = payload_data.next().await {
        result_file_content.write_all(&chunk.unwrap()).unwrap();
    }

    let file_metadata = FileMetadata::new(
        String::from(""),
        PathUtils::get_unique_file_path()
    );

    let connection_info = request.connection_info();
    let remote_addr = connection_info.remote().unwrap_or("-");

    let client = state.storage_client_pool.pull();

    let upload_result = client.upload(
        &file_metadata.path,
        &file_metadata.content_type,
        result_file_content,
    ).await;
   
    match upload_result {
        Ok(_) => {
            let file_id = file_metadata.into_id(&state.options.token_key);

            info!(state.logger, "Upload success {0}", remote_addr);
            
            Ok(HttpResponse::Ok().body(file_id))
        },
        Err(err) => {
            error!(state.logger, "Upload error {0}, {1}", err, remote_addr);
            
            Ok(HttpResponse::BadRequest().finish())
        }
    }
}
