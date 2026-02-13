use axum::{
    Json, Router, extract::Path, routing::{get, post}
};
use chrono::{Local, Timelike};
use store::*;
mod store;

use crate::request_input::UserInput;
mod request_input;
#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(get_user))
        .route("/user", post(user_input))
        .route("/check/{user_id}",post(check))
        .route("/path/{pathname}", get(path_check))
        .route("/seedata", get(see_data))
        .route("/store", get(store_data));
        
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    print!("server is running on port {}",3000);
    axum::serve(listener, app).await.unwrap();
    print!("after all et");
}

async fn get_user()->&'static str{
    "home route"
}

async fn path_check(Path(pathname): Path<String>) -> String{
    format!("hello : {}", pathname.as_str())
}

async fn check(Path(user_id):Path<String>)->String{ // url is check/<1424242343242323>
    format!("user id for string: {}", user_id.to_string())
}

async fn user_input(Json(data): Json<UserInput>)-> String{
    format!("email : {}, password : {}, phone_no : {}", data.email, data.password, data.phone_no)
}

async fn see_data(Json(data): Json<UserInput>) -> Json<UserInput>{
   Json(UserInput { email: String::from(data.email), password: data.password, phone_no: data.phone_no })
}

async fn store_data(Json(data) : Json<UserInput>) -> String{
   let a = UserTable{
        username : String::from("nobita"),
        email  : String::from("nobit"),
        phone : 96555555,
        created_at : Local::now().hour()
   };
   match a.create_user() {
    Some(a) => a,
    None => String::from("") 
   }

}