use serde::{Deserialize, Serialize};
use std::error::Error;
use crate::RunOptions;
use crate::digest::HmacUtils;
use std::fmt::Write;

#[derive(Serialize, Deserialize)]
pub struct FileMetadata {
    #[serde(rename = "t")]
    pub content_type: String,
    #[serde(rename = "p")]
    pub path: String
}

impl FileMetadata {
    pub fn new(content_type: String, path: String) -> Self {
        FileMetadata {
            content_type,
            path
        }
    }
    
    // {data}.{signature}
    pub fn from_id(file_id: &str, token_secret: &str) -> Result<FileMetadata, String> {
        let splitted: Vec<&str> = file_id.split('.').collect();
        
        if splitted.len() != 2 {
            return Err(String::from("format error"))
        }
        
        let data = splitted[0];
        let signature = splitted[1];

        let computed_signature = HmacUtils::compute(&data, token_secret);
        
        if signature != computed_signature {
            return Err(String::from("invalid signature"))
        }
        
        let decoded_data = base64::decode(data).expect("cannot decode base64 data");

        match serde_json::from_slice::<FileMetadata>(&decoded_data){
            Ok(file_metadata) => Ok(file_metadata),
            Err(err) => Err(format!("Deserialization error: {}", err.to_string()))
        }
    }
    
    pub fn into_id(self, token_secret: &str) -> String {
        let json_data = serde_json::to_string(&self).expect("cannot serialize metadata");
 
        let base64_json_data = base64::encode(json_data);

        let signature = HmacUtils::compute(&base64_json_data, token_secret);

        let mut result_string = String::with_capacity(base64_json_data.len() + 1 + signature.len());

        result_string.write_str(&base64_json_data);
        result_string.write_char('.');
        result_string.write_str(&signature);

        result_string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_hash_from_hash_is_correct() {
        let token_key = "some_key";
        
        let metadata = FileMetadata::new(
            "image/jpg".to_string(), 
            "/path".to_string());
        
        let decoded_metadata= FileMetadata::from_id(&metadata.into_id(&token_key), &token_key).unwrap();
        
        assert_eq!(decoded_metadata.content_type, "image/jpg".to_string());
        assert_eq!(decoded_metadata.path, "/path".to_string());
    }
}