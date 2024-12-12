#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;

fn main() {
    // READ .env file
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DB URL NOT FOUND!");

    let mut conn = PgConnection::establish(&db_url).expect("CONNECTION INPOSIBLE");

    use self::models::Post;
    use self::schema::posts::dsl::*;

    //SELECT

    let posts_result = posts.load::<Post>(&mut conn).expect("Error en Query");

    for post in posts_result{
        println!("{:?}",post.title);
    }
}