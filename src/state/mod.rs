use object_pool::Pool;
use crate::storage::S3Client;

pub struct ApplicationState<'c> {
    storage_client: Pool<'c, S3Client>,
}