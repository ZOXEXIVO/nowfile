use actix_web::web::Data;
use actix_web::{HttpResponse, web, Result};
use serde::Deserialize;

use crate::models::{ApplicationState, FileMetadata};

#[derive(Deserialize)]
pub struct DownloadFileQuery {
   file_hash: String
}

pub async fn download_action(state: Data<ApplicationState<'_>>, query_params: web::Path<DownloadFileQuery>) -> Result<HttpResponse> {
    match FileMetadata::from_hash(&query_params.file_hash) {
        Ok(metadata) => {
            let client = state.storage_client_pool.pull();

            let file_content = client.download(metadata.path).await;

            Ok(HttpResponse::Ok().content_type(metadata.content_type).body(file_content))
        }, 
        Err(e) => {
            Ok(HttpResponse::BadRequest().body(e))
        }
    }
}

