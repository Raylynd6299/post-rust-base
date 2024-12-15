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

    use self::models::{Post, NewPost, PostSimple};
    use self::schema::posts;
    use self::schema::posts::dsl::*;

    // let new_post = NewPost{
    //     title: "Mi segundo blog",
    //     body: "lorem impsu",
    //     slug: "primer_post"
    // };

    // let post_res:Post = diesel::insert_into(posts::table).values(&new_post).get_result(&mut conn).expect("Error insertando datos");

    //SELECT
    // let posts_result = posts.load::<Post>(&mut conn).expect("Error en Query");

    // let posts_result = posts.order(id.desc()).select((title,body)).limit(1).load::<PostSimple>(&mut conn).expect("Error en Query");

    // let posts_result = posts.filter(id.eq(2)).limit(1).load::<Post>(&mut conn).expect("Error en Query");

    // Update
    // let post_update = diesel::update(posts.filter(id.eq(2))).set((title.eq("Tercer blog"),slug.eq("New_slug"))).get_result::<Post>(&mut conn).expect("Error en Query");

    let posts_result = posts.load::<Post>(&mut conn).expect("Error en Query");

    //DElete
    diesel::delete(posts.filter(slug.like("%post%"))).execute(&mut conn).expect("Fallo la eliminacion");

    for post in posts_result{
        println!("{:?} {}",post.title, post.slug);
    }
}