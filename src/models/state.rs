use object_pool::Pool;
use crate::storage::S3Client;

pub struct ApplicationState<'s> {
    pub storage_client_pool: Pool<'s, S3Client>,
}