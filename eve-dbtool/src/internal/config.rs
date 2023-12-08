use std::env;

pub struct Config {
    pub db_host: String,
    pub db_port: u16,
    pub db_username: String,
    pub db_password: String,
    pub db_database: String,
    pub seed_market: bool,
    pub seed_regions: Vec<String>,
    pub seed_saturation: u128,
}

impl Config {
    pub fn build() -> Result<Config, &'static str> {
        let db_host = env::var("MARIADB_HOST").unwrap();
        let db_port = env::var("MARIADB_PORT").unwrap().parse::<u16>().unwrap();
        let db_username = env::var("MARIADB_USER").unwrap();
        let db_password = env::var("MARIADB_PASSWORD").unwrap();
        let db_database = env::var("MARIADB_DATABASE").unwrap();
        let seed_market = env::var("SEED_MARKET").is_ok();
        let seed_regions = env::var("SEED_REGIONS")
            .unwrap()
            .split(",")
            .map(|v| v.to_string())
            .collect::<Vec<String>>();
        let seed_saturation = env::var("SEED_SATURATION")
            .unwrap()
            .parse::<u128>()
            .unwrap();
        Ok(Config {
            db_host,
            db_port,
            db_username,
            db_password,
            db_database,
            seed_market,
            seed_regions,
            seed_saturation,
        })
    }
}

pub fn get_config() -> Option<Config> {
    if let Ok(config) = Config::build() {
        return Some(config);
    }
    None
}
