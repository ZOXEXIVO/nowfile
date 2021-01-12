use fnv::FnvHasher;
use std::hash::{Hash, Hasher};

pub struct HashUtils;

impl HashUtils {
    pub fn compute(data: &str) -> String {
        Self::create_digest(data).to_string()
    }

    fn create_digest(data: &str) -> u64 {
        let mut state = FnvHasher::default();

        data.as_bytes().hash(&mut state);

        state.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_is_correct() {
        let data1 = String::from("SomeData1");
        let data2 = String::from("SomeData2");

        let data1_signature = HashUtils::compute(&data1);
        let data1_again_signature = HashUtils::compute(&data1);

        let data2_signature = HashUtils::compute(&data2);

        assert_eq!(data1_signature, String::from("10429865961791751215"));
        assert_eq!(data2_signature, String::from("10429867061303379426"));

        assert_eq!(data1_signature, data1_again_signature);
        assert_ne!(data1_signature, data2_signature);
    }
}
