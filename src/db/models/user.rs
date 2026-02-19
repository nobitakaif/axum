use crate::{db::store::Store};

use uuid::Uuid;
use diesel::{prelude::*};
use schema::*;

#[derive(Queryable,Insertable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct User{
    id : String,
    username : String,
    password : String
}


impl Store{
    pub fn signup(&self, username: String, password : String) -> Result<String, diesel::result::Error>{
        let id = Uuid::new_v4();
        let user = User{
            id : id,
            username,
            password,
        };
        diesel::insert_into(crate::schema::user::table)
            .values(&user).returning(User::as_returning()).get_result(self.conn)?;
        Ok(id)        
    }

    pub fn signin(&self) -> String{
        String::from("123321")
    }
}