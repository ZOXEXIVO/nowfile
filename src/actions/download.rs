use actix_web::web::Data;
use crate::AppState;
use actix_web::{HttpResponse, web, Result};
use serde::Deserialize;
use crate::file::FileMetadata;

#[derive(Deserialize)]
pub struct DownloadFileQuery {
   file_id: String
}

pub async fn download_action(state: Data<AppState<'_>>, query_params: web::Path<DownloadFileQuery>) -> Result<HttpResponse> {
    let file_metadata = FileMetadata::from_hash(&query_params.file_id);
    
    let client = state.storage_client.pull();

    let file_content = client.download(file_metadata.path).await;

    Ok(HttpResponse::Ok().content_type(file_metadata.content_type).body(file_content))
}

