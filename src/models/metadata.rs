use serde::{Deserialize, Serialize};
use crate::digest::HashUtils;
use std::fmt::Write;
use crate::utils::Base64Utils;

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
    pub fn from_id(file_id: &str) -> Result<FileMetadata, String> {
        let decoding_result;
        
        match base64::decode(Base64Utils::decode_for_web(file_id)) {
            Ok(res) => decoding_result = res,
            Err(_) => return Err(String::from("format error"))
        }
        
        let decoded = String::from_utf8(decoding_result).unwrap();
        
        let data_with_signature: Vec<&str> = decoded.split('.').collect();
        
        if data_with_signature.len() != 2 {
            return Err(String::from("format error"))
        }
        
        let data = data_with_signature[0];
        let signature = data_with_signature[1];

        let computed_signature = HashUtils::compute(&data);
        
        if signature != computed_signature {
            return Err(String::from("invalid signature"))
        }
        
        let decoded_data = base64::decode(data).expect("cannot decode base64 data");

        match serde_json::from_slice::<FileMetadata>(&decoded_data){
            Ok(file_metadata) => Ok(file_metadata),
            Err(err) => Err(format!("Deserialization error: {}", err.to_string()))
        }
    }
    
    pub fn into_id(self) -> String {
        let json_data = serde_json::to_string(&self).expect("cannot serialize metadata");
 
        let base64_json_data = base64::encode(json_data);

        let signature = HashUtils::compute(&base64_json_data);

        let mut result_string = String::with_capacity(base64_json_data.len() + 1 + signature.len());

        result_string.write_str(&base64_json_data).unwrap();
        result_string.write_char('.').unwrap();
        result_string.write_str(&signature).unwrap();

        Base64Utils::encode_for_web(&base64::encode(result_string))      
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_hash_from_hash_is_correct() {
        let metadata = FileMetadata::new(
            "image/jpg".to_string(), 
            "/path".to_string());
        
        let decoded_metadata= FileMetadata::from_id(&metadata.into_id()).unwrap();
        
        assert_eq!(decoded_metadata.content_type, "image/jpg".to_string());
        assert_eq!(decoded_metadata.path, "/path".to_string());
    }
}