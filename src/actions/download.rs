use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, web, Result, HttpRequest};
use serde::Deserialize;

use slog::*;

use crate::models::{ApplicationState, FileMetadata};

#[derive(Deserialize)]
pub struct DownloadFileQuery {
   file_id: String
}

pub async fn download_action(request: HttpRequest, route_params: web::Path<DownloadFileQuery>, state: Data<ApplicationState<'_>>) -> Result<HttpResponse> {
    if route_params.file_id == "favicon.ico" {
        return Ok(HttpResponse::NotFound().finish())
    }

    match FileMetadata::from_id(&route_params.file_id, &state.options.token_key) {
        Ok(metadata) => {
            let client = state.storage_client_pool.pull();

            let file_content = client.download(&metadata.path).await;

            let connection_info = request.connection_info();
            let remote_addr = connection_info.remote();
            
            info!(state.logger, "download success, {0} {1}", metadata.path, remote_addr.unwrap_or("-"));
            
            Ok(HttpResponse::Ok().content_type(&metadata.content_type).body(file_content))
        }, 
        Err(err) => {
            warn!(state.logger, "download failed, {0}, {1}", &route_params.file_id, err);
            
            Ok(HttpResponse::BadRequest().body(err))
        }
    }
}

