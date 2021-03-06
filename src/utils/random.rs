use rand::distributions::Alphanumeric;
use rand::Rng;

pub struct Random;

impl Random {
    pub fn generate_random(characters_count: usize) -> String {
        let bytes_data: Vec<u8> = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(characters_count)
            .collect();
        
        String::from_utf8(bytes_data).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_random_is_random() {
        let value1 = Random::generate_random(30);
        let value2 = Random::generate_random(30);
        let value3 = Random::generate_random(30);

        assert_ne!(value1, value2);
        assert_ne!(value1, value3);
        assert_ne!(value2, value3);
    }
}
