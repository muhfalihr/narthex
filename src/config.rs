use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub app_host: String,
    pub app_port: u16,
    pub db_host: String,
    pub db_port: u16,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    pub app_username: Option<String>,
    pub app_password: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self, envy::Error> {
        dotenvy::dotenv().ok();
        envy::from_env::<Config>()
    }

    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.db_user, self.db_password, self.db_host, self.db_port, self.db_name
        )
    }
}
