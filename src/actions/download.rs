use actix_web::web::Data;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use serde::Deserialize;

use slog::*;

use crate::models::{ApplicationState, FileMetadata};

#[derive(Deserialize)]
pub struct DownloadFileQuery {
    file_id: String,
}

pub async fn download_action(
    request: HttpRequest,
    route_params: web::Path<DownloadFileQuery>,
    state: Data<ApplicationState>,
) -> Result<HttpResponse> {
    let connection_info = request.connection_info();
    let remote_addr = connection_info.remote_addr().unwrap_or("-");

    match FileMetadata::from_id(&route_params.file_id) {
        Ok(metadata) => {
            let client = state.storage_client_pool.pull(|| state.create_client());

            let file_content_result = client.download(&metadata.path).await;

            match file_content_result {
                Ok(file_content) => {
                    info!(
                        state.logger,
                        "download success, {0} {1}", metadata.path, remote_addr
                    );

                    Ok(HttpResponse::Ok()
                        .content_type(&metadata.content_type)
                        .body(file_content))
                }
                Err(err) => {
                    error!(
                        state.logger,
                        "download failed, {0}, {1}", &route_params.file_id, err
                    );

                    Ok(HttpResponse::InternalServerError().finish())
                }
            }
        }
        Err(err) => {
            error!(
                state.logger,
                "download failed, {0}, {1}", &route_params.file_id, err
            );

            Ok(HttpResponse::NotFound().finish())
        }
    }
}
