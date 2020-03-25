use crate::utils::{PathUtils};
use crate::AppState;
use actix_multipart::Multipart;
use actix_web::web::Data;
use actix_web::{Error, HttpResponse, Result};

use futures::{StreamExt, TryStreamExt};
use std::io::Write;
use crate::file::FileMetadata;

pub async fn upload_action(
    state: Data<AppState<'_>>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    let mut payload_data = payload.try_next().await.unwrap().unwrap();

    let mut result_file_content = Vec::new();

    while let Some(chunk) = payload_data.next().await {
        result_file_content.write(&chunk.unwrap()).unwrap();
    }

    let file_metadata = FileMetadata::new(
        payload_data.content_disposition().unwrap().to_string(),
        PathUtils::get_unique_file_path(),
        result_file_content.len()
    );
    
    let client = state.storage_client.pull();

    client.upload(
        &file_metadata.path,
        &file_metadata.content_type,
        result_file_content,
    ).await;

    Ok(HttpResponse::Ok().body(file_metadata.into_hash()).into())
}
