pub struct DBConfig {
    db: String,
    username: String,
    password: String,
}

impl DBConfig {
    pub fn default() -> DBConfig {
        DBConfig{
            db: "osm".to_string(),
            username: "postgres".to_string(),
            password: "password".to_string(),
        }
    }

    pub fn generate_url(&self) -> String {
        let gen = format!("postgresql://{}:{}@localhost:5432/{}", self.username, &self.password, self.db);
        return  gen.to_string();
    }
}

pub struct QueryConfig {

}