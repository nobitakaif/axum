
use chrono::{Local, DateTime};

pub struct UserTable{
    pub email : String,
    pub username : String,
    pub phone : u64,
    pub created_at : u32
}

impl UserTable{
    pub fn create_user(&self)-> Option<String>{
        Some(String::from("kuch bhi"))
    }
}