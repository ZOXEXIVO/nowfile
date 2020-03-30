use crate::utils::{PathUtils};
use actix_multipart::Multipart;
use actix_web::web::Data;
use actix_web::{Error, HttpResponse, Result};

use futures::{StreamExt, TryStreamExt};
use std::io::Write;
use crate::models::{ApplicationState, FileMetadata};
use mime_guess::MimeGuess;

pub async fn upload_action(
    state: Data<ApplicationState<'_>>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    let mut payload_data = payload.try_next().await.unwrap().unwrap();

    let mut result_file_content = Vec::new();

    while let Some(chunk) = payload_data.next().await {
        result_file_content.write(&chunk.unwrap()).unwrap();
    }

    let file_metadata = FileMetadata::new(
        String::from(""),
        PathUtils::get_unique_file_path()
    );
    
    let client = state.storage_client_pool.pull();

    client.upload(
        &file_metadata.path,
        &file_metadata.content_type,
        result_file_content,
    ).await;

    let file_id = file_metadata.into_id(&state.options.token_key);

    Ok(HttpResponse::Ok().body(file_id).into())
}
