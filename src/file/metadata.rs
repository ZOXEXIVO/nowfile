use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FileMetadata {
    #[serde(rename = "c")]
    pub content_type: String,
    #[serde(rename = "p")]
    pub path: String,
    #[serde(rename = "s")]
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
    
    pub fn from_hash(hash: &str) -> Self {
        let base64_decoded = base64::decode(hash).unwrap();
        
        serde_json::from_slice(&base64_decoded).unwrap()
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
        
        let decoded_metadata= FileMetadata::from_hash(&metadata.into_hash());
        
        assert_eq!(decoded_metadata.content_type, "image/jpg".to_string());
        assert_eq!(decoded_metadata.path, "/path".to_string());
        assert_eq!(decoded_metadata.size, 123);
    }
}