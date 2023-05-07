use std::fs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DBConfig {
    db: String,
    username: String,
    password: String,
}

impl std::default::Default for DBConfig {
    fn default() -> DBConfig {
        DBConfig{
            db: "osm".to_string(),
            username: "postgres".to_string(),
            password: "password".to_string(),
        }
    }
}

impl DBConfig {
    pub fn from_file(path: &str) -> DBConfig {
        let config_raw = fs::read_to_string(path).unwrap();
        let settings: DBConfig = serde_json::from_str(&config_raw).expect("JSON did not work");
        settings
    }

    pub fn generate_url(&self) -> String {
        format!("postgresql://{}:{}@localhost:5432/{}", self.username, &self.password, self.db)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let settings = DBConfig::default();
        assert_eq!(settings.db, "osm");
        assert_eq!(settings.username, "postgres");
        assert_eq!(settings.password, "password");
    }

    #[test]
    fn test_config_from_file() {
        let path = "tests/data/dbconfig.json";
        let settings = DBConfig::from_file(path);
        assert_eq!(settings.db, "osm");
        assert_eq!(settings.username, "postgres");
        assert_eq!(settings.password, "password");
    }
}
