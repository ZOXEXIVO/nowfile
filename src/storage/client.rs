use s3::bucket::Bucket;
use s3::region::Region;
use s3::credentials::Credentials;

pub struct S3Client {
    bucket: Bucket
}

impl S3Client {
    pub fn new(endpoint: &str, bucket_name: &str, access_key: &str, secret_key: &str) -> Self {
        let credentials = Credentials::new(
            Some(access_key.to_string()), 
            Some(secret_key.to_string()), 
            None, None);

        let region = Region::Custom {
            region: "Custom".to_string(),
            endpoint: endpoint.to_string()
        };

        S3Client {
            bucket: Bucket::new(&bucket_name, region, credentials).unwrap()
        }
    }
    
    pub async fn download(&self, path: &str) -> Vec<u8> {
        let result = self.bucket.get_object(path).await.unwrap();  
        
        result.0
    }

    pub async fn upload(&self, path: &str, content_type: &str, file_content: Vec<u8>) {
        self.bucket.put_object(&path, &file_content, &content_type).await.unwrap();
    }
}