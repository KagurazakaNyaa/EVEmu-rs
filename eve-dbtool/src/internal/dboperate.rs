use sqlx::{
    any::{install_default_drivers, AnyPoolOptions},
    query, AnyPool, Error,
};

use super::config::Config;

async fn get_db_pool(config: &Config) -> Result<AnyPool, Error> {
    install_default_drivers();
    let pool = AnyPoolOptions::new().connect(&config.db_url).await?;
    Ok(pool)
}

pub async fn execute_sql(sql: &str, config: Config) -> Result<(), Error> {
    match get_db_pool(&config).await {
        Ok(pool) => {
            query(sql).execute_many(&pool).await;
            Ok(())
        }
        Err(error) => Err(error),
    }
}
