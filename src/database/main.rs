use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

// use diesel::prelude::*;
use diesel::QueryDsl;
use super::schema::posts::dsl::*;
use super::models::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


pub fn test() {
    let connection = &mut establish_connection();
    println!("Successfully connected to database!");
    
    create_post(connection);
}

pub fn create_post(conn: &mut PgConnection) {
    let post = NewPost {
        title: "Nigga!",
        content: "grid:set_pixel(0, 0, 255, 255, 255)",
        id_user: 1,
        height: 256,
        version: "0",
        width: 256,
        url_bucket: "nigga.com"
    };


}