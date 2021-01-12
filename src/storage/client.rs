use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;

pub struct S3Client {
    bucket: Bucket,
}

impl S3Client {
    pub fn new(endpoint: &str, bucket_name: &str, access_key: &str, secret_key: &str) -> Self {
        let credentials =
            Credentials::new(Some(access_key), Some(secret_key), None, None, None).unwrap();

        let region = Region::Custom {
            region: "Custom".to_string(),
            endpoint: endpoint.to_string(),
        };

        S3Client {
            bucket: Bucket::new_with_path_style(&bucket_name, region, credentials).unwrap(),
        }
    }

    pub async fn download(&self, path: &str) -> Result<Vec<u8>, String> {
        match self.bucket.get_object(path).await {
            Ok((data, status_code)) => match status_code {
                200 => Ok(data),
                _ => Err(format!("StatusCode: {0}", status_code)),
            },
            Err(err) => Err(err.description.unwrap()),
        }
    }

    pub async fn upload(
        &self,
        path: &str,
        content_type: &str,
        file_content: Vec<u8>,
    ) -> Result<(), String> {
        match self
            .bucket
            .put_object_with_content_type(&path, &file_content, &content_type)
            .await
        {
            Ok((_, status_code)) => match status_code {
                200 => Ok(()),
                _ => Err(format!("StatusCode: {0}", status_code)),
            },
            Err(err) => Err(String::from(err.description.unwrap())),
        }
    }
}
