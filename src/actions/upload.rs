use crate::utils::PathUtils;
use actix_multipart::Multipart;
use actix_web::web::Data;
use actix_web::{Error, HttpRequest, HttpResponse, Result};

use crate::models::{ApplicationState, FileMetadata};
use futures::{StreamExt, TryStreamExt};
use std::io::Write;

use log::{info, error};

pub async fn upload_action(
    request: HttpRequest,
    state: Data<ApplicationState>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    let mut payload_data = payload.try_next().await.unwrap().unwrap();

    let mut result_file_content = Vec::new();

    while let Some(chunk) = payload_data.next().await {
        result_file_content.write_all(&chunk.unwrap()).unwrap();
    }

    let file_metadata = FileMetadata::new(String::from(""), PathUtils::get_unique_file_path());

    let connection_info = request.connection_info();
    let remote_addr = connection_info.peer_addr().unwrap_or("-");

    let client = state.storage_client_pool.pull(|| state.create_client());

    let upload_result = client
        .upload(
            &file_metadata.path,
            &file_metadata.content_type,
            result_file_content,
        )
        .await;

    match upload_result {
        Ok(_) => {
            let file_id = file_metadata.into_id();

            info!("Upload success {0}", remote_addr);

            Ok(HttpResponse::Ok().body(file_id))
        }
        Err(err) => {
            error!("Upload error {0}, {1}", err, remote_addr);

            Ok(HttpResponse::BadRequest().finish())
        }
    }
}
