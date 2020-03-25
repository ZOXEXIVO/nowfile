use chrono::{DateTime, Datelike, Utc};
use crate::utils::Random;

pub struct PathUtils;

impl PathUtils{
    pub fn get_unique_file_path() -> String{
        let folder = Self::get_current_date_folder();
        let filename = Random::generate_random(30);
        
        format!("{0}/{1}", folder, filename)
    } 
    
    fn get_current_date_folder() -> String {
        let utc: DateTime<Utc> = Utc::now();

        format!("{0:02}_{1:02}_{2}", utc.day(), utc.month(), utc.year())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_unique_file_path_is_unique() {
        let value1 = PathUtils::get_unique_file_path();
        let value2 = PathUtils::get_unique_file_path();
        let value3 = PathUtils::get_unique_file_path();

        assert_ne!(value1, value2);
        assert_ne!(value1, value3);
        assert_ne!(value2, value3);
    }
}