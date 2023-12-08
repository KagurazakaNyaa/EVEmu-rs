use mysql::{Error, OptsBuilder, Pool};

use super::config::Config;

fn get_db_pool(config: Config) -> Result<Pool, Error> {
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(config.db_host))
        .tcp_port(config.db_port)
        .user(Some(config.db_username))
        .pass(Some(config.db_password))
        .db_name(Some(config.db_database));
    Pool::new(opts)
}
