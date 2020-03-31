use sha2::Sha256;
use hmac::{Hmac, Mac};
use std::fmt::Write;

type HmacSha256 = Hmac<Sha256>;

pub struct HmacUtils;

impl HmacUtils {
    pub fn compute(data: &str, key: &str) -> String {
        let digest = Self::create_digest(data, key);

        Self::encode_for_web(base64::encode(digest))
    }

    fn create_digest(data: &str, key: &str) -> Vec<u8> {
        let mut mac = HmacSha256::new_varkey(key.as_bytes()).unwrap();

        mac.input(data.as_bytes());

        mac.result().code().to_vec()
    }
    
    fn encode_for_web(base64_hash: String) -> String {
        base64_hash
            .trim_end_matches("=")
            .replace("+", "-")
            .replace("/", "_")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::FileMetadata;

    #[test]
    fn compute_is_correct() {
        let key = "some_key";

        let data1 = String::from("SomeData1");
        let data2 = String::from("SomeData2");

        let data1_signature = HmacUtils::compute(&data1, key);
        let data1_again_signature = HmacUtils::compute(&data1, key);

        let data2_signature = HmacUtils::compute(&data2, key);

        assert_eq!(data1_signature, String::from("+p0GsMjfQuwjx5nzMqO63BAfH8Dgl8kA0PzJ5oHzO40="));
        assert_eq!(data2_signature, String::from("u8iGxCFZhCGiTANa8rh4TJOmZRnlYzsEI9kLWYretSM="));
        
        assert_eq!(data1_signature, data1_again_signature);
        assert_ne!(data1_signature, data2_signature);
    }
}