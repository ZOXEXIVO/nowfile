use object_pool::Pool;
use crate::storage::S3Client;
use crate::RunOptions;

pub struct ApplicationState<'s> {
    pub storage_client_pool: Pool<'s, S3Client>,
    pub options: RunOptions,
    pub logger: slog::Logger
}