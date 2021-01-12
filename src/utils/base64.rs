pub struct Base64Utils;

impl Base64Utils {
    pub fn encode_for_web(data: &str) -> String {
        data.trim_end_matches("=")
            .replace("+", "-")
            .replace("/", "_")
    }

    pub fn decode_for_web(data: &str) -> String {
        let mut data = data.replace("-", "+").replace("_", "/");

        match data.len() % 4 {
            2 => data.push_str("=="),
            3 => data.push_str("="),
            _ => {}
        }

        data
    }
}
