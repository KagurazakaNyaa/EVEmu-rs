use std::env;

pub struct Config {
    pub db_url: String,
    pub seed_market: bool,
    pub seed_regions: Vec<String>,
    pub seed_saturation: u128,
}

impl Config {
    pub fn build() -> Option<Config> {
        let db_url = env::var("DB_URL").unwrap_or("sqlite://evemu.db".to_string());
        let seed_market = env::var("SEED_MARKET");
        let seed_market = match seed_market {
            Ok(cond) => match cond.to_lowercase().as_str() {
                "true" => true,
                "t" => true,
                "1" => true,
                _ => false,
            },
            Err(_) => false,
        };
        let seed_regions = env::var("SEED_REGIONS")
            .unwrap_or("Derelik,The Citadel,The Forge".to_string())
            .split(",")
            .map(|v| v.to_string())
            .collect::<Vec<String>>();
        let seed_saturation = env::var("SEED_SATURATION")
            .unwrap_or("80".to_string())
            .parse::<u128>()
            .unwrap();
        Some(Config {
            db_url,
            seed_market,
            seed_regions,
            seed_saturation,
        })
    }
}
