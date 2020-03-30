use sha2::Sha256;
use hmac::{Hmac, Mac};
use std::fmt::Write;

type HmacSha256 = Hmac<Sha256>;

pub struct HmacUtils;

impl HmacUtils {
    pub fn compute(data: &str, key: &str) -> String {
        let mut mac = HmacSha256::new_varkey(key.as_bytes()).unwrap();

        mac.input(data.as_bytes());

        let hash_vec = mac.result().code().to_vec();

        hash_vec.iter()
            .fold(String::with_capacity(2 * hash_vec.len()),
                  |mut res, b| {
                      res.push_str(&format!("{:02x}", b));
                      res
                  })
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

        assert_eq!(data1_signature, String::from("fa9d06b0c8df42ec23c799f332a3badc101f1fc0e097c900d0fcc9e681f33b8d"));
        assert_eq!(data2_signature, String::from("bbc886c421598421a24c035af2b8784c93a66519e5633b0423d90b598adeb523"));
        
        assert_eq!(data1_signature, data1_again_signature);
        assert_ne!(data1_signature, data2_signature);
    }
}