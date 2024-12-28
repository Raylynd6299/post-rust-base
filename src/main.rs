#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{ Pool, self, ConnectionManager };

use actix_web::{ get, post, web, App, HttpResponse, HttpServer, Responder };

use self::models::{ Post, NewPost, PostSimple, NewPostHandler };
use self::schema::posts;
use self::schema::posts::dsl::*;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn index(pool: web::Data<DBPool>) -> impl Responder {
    let mut conn = pool.get().expect("Error getting connectiong of db pool");

    match web::block(move || { posts.load::<Post>(&mut conn) }).await {
        Ok(data) => { HttpResponse::Ok().body(format!("{:?}\n", data)) }
        Err(err) => HttpResponse::Ok().body(format!("Error: {:?}\n", err)),
    }
}

#[post("/new_post")]
async fn new_post(pool: web::Data<DBPool>, item: web::Json<NewPostHandler>) -> impl Responder {
    let mut conn = pool.get().expect("Error getting connectiong of db pool");

    match
        web::block(move || {
            Post::create_post(&mut conn,&item)
        }).await
    {
        Ok(data) => { HttpResponse::Ok().body(format!("{:?}\n", data)) }
        Err(err) => HttpResponse::Ok().body(format!("Error: {:?}\n", err)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DB URL NOT FOUND!");
    // let mut conn = PgConnection::establish(&db_url).expect("CONNECTION INPOSIBLE");

    let connection = ConnectionManager::<PgConnection>::new(db_url);

    let pool = Pool::builder().build(connection).expect("Error creating pool");

    // let post_res: Post = diesel
    //     ::insert_into(posts::table)
    //     .values(&new_post)
    //     .get_result(&mut conn)
    //     .expect("Error insertando datos");

    // let posts_result = posts.load::<Post>(&mut conn).expect("Error en Query");

    // println!("Test ");
    // for post in posts_result {
    //     println!("{:?} {}", post.title, post.slug);
    // }

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone())).service(index).service(new_post)
    })
        .bind(("0.0.0.0", 9000))?
        .run().await

    // //SELECT
    // // let posts_result = posts.load::<Post>(&mut conn).expect("Error en Query");

    // // let posts_result = posts.order(id.desc()).select((title,body)).limit(1).load::<PostSimple>(&mut conn).expect("Error en Query");

    // // let posts_result = posts.filter(id.eq(2)).limit(1).load::<Post>(&mut conn).expect("Error en Query");

    // // Update
    // // let post_update = diesel::update(posts.filter(id.eq(2))).set((title.eq("Tercer blog"),slug.eq("New_slug"))).get_result::<Post>(&mut conn).expect("Error en Query");

    // let posts_result = posts.load::<Post>(&mut conn).expect("Error en Query");

    // //DElete
    // diesel::delete(posts.filter(slug.like("%post%"))).execute(&mut conn).expect("Fallo la eliminacion");

    // for post in posts_result{
    //     println!("{:?} {}",post.title, post.slug);
    // }
}
