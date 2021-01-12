use object_pool::Pool;
use crate::storage::S3Client;
use crate::RunOptions;

pub struct ApplicationState {
    pub storage_client_pool: Pool<S3Client>,
    pub options: RunOptions
}

impl ApplicationState{
    pub fn create_client(&self) -> S3Client {
        S3Client::new(
            &self.options.endpoint,
            &self.options.bucket_name,
            &self.options.access_key,
            &self.options.secret_key,
        )
    }
}
