use std::env;

pub struct Config {
    pub db_url: String,
    pub seed_market: bool,
    pub seed_regions: Vec<String>,
    pub seed_saturation: u128,
}

impl Config {
    pub fn build() -> Result<Config, &'static str> {
        let db_url = env::var("DB_URL").unwrap();
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
            db_url,
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
