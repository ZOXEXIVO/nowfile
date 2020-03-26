use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct FileMetadata {
    pub content_type: String,
    pub path: String,
    pub size: usize
}

impl FileMetadata {
    pub fn new(content_type: String, path: String, size: usize) -> Self {
        FileMetadata{
            content_type,
            path,
            size
        }
    }
    
    pub fn from_hash(hash: &str) -> Result<FileMetadata, String> {
        match base64::decode(hash) {
            Ok(decoded_data) => {
                match serde_json::from_slice::<FileMetadata>(&decoded_data){
                    Ok(file_metadata) => Ok(file_metadata),
                    Err(err) => Err(format!("Deserialization error: {}", err.to_string()))
                }
            },
            Err(_) => {
                Err(String::from("Base64 decoding error"))
            }
        }
    }
    
    pub fn into_hash(self) -> String {
        let json_data = serde_json::to_string(&self).unwrap();
 
        base64::encode(json_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_hash_from_hash_is_correct() {
        let metadata = FileMetadata::new(
            "image/jpg".to_string(), 
            "/path".to_string(), 
            123);
        
        let decoded_metadata= FileMetadata::from_hash(&metadata.into_hash()).unwrap();
        
        assert_eq!(decoded_metadata.content_type, "image/jpg".to_string());
        assert_eq!(decoded_metadata.path, "/path".to_string());
        assert_eq!(decoded_metadata.size, 123);
    }
}