
use std::env::{self, VarError};
use std::fmt::Error;

use chrono::{Local, DateTime};
use diesel::{Connection, ConnectionError, PgConnection};

use crate::db::config::Config;
use crate::request_input::UserInput;



pub struct UserTable{
    conn : PgConnection
}

impl UserTable {
    pub fn default() -> Result<Self, ConnectionError> {
        // reading the env var(read about unwrap_or and unwrap_or_else)
        let config = Config::default();
        let connection = PgConnection::establish(&config.db_url)?;

        Ok(Self { 
            conn : connection
        })
    }
}

impl UserTable{
    pub fn signin(&self){
        
    }
}