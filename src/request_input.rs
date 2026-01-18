use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UserInput{
    pub email : String,
    pub password : String,
    pub phone_no : u32
}