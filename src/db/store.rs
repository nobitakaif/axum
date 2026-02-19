

use diesel::{Connection, ConnectionError, PgConnection};

use crate::db::config::Config;

pub struct Store{
    pub conn : PgConnection
}

impl Store {
    pub fn default() -> Result<Self, ConnectionError> {
        // reading the env var(read about unwrap_or and unwrap_or_else)
        let config = Config::default(); // here we will get env varible
        let connection = PgConnection::establish(&config.db_url)?;

        Ok(Self { 
            conn : connection
        })
    }
}

