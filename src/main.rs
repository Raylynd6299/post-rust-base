// #[macro_use]}
extern crate diesel;

pub mod schema;
pub mod models;

use tera::Tera;
use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{ Pool, self, ConnectionManager };

use actix_web::{ get, post, web, App, HttpResponse, HttpServer, Responder };

use self::models::{ Post, NewPostHandler }; //NewPost, PostSimple,
// use self::schema::posts;
use self::schema::posts::dsl::*;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn index(pool: web::Data<DBPool>, template_manager: web::Data<tera::Tera>) -> impl Responder {
    let mut conn = pool.get().expect("Error getting connectiong of db pool");

    match web::block(move || { posts.load::<Post>(&mut conn) }).await {
        Ok(data) => {
            let data = data.unwrap();

            let mut context = tera::Context::new();
            context.insert("posts",&data);

            HttpResponse::Ok()
                .content_type("text/html")
                .body(template_manager.render("index.html", &context).unwrap())
        }
        Err(err) => HttpResponse::Ok().body(format!("Error: {:?}\n", err)),
    }
}

#[get("/blog/{blog_slug}")]
async fn get_post(pool: web::Data<DBPool>, template_manager: web::Data<tera::Tera>, path: web::Path<String>) -> impl Responder {
    let mut conn = pool.get().expect("Error getting connectiong of db pool");
    let blog_slug = path.into_inner();

    match web::block(move || { posts.filter(slug.eq(blog_slug)).load::<Post>(&mut conn) }).await {
        Ok(data) => {
            let data = data.unwrap();

            if data.len() == 0{
                return HttpResponse::NotFound().finish();
            }
            let data = &data[0];

            let mut context = tera::Context::new();
            context.insert("post",&data);

            HttpResponse::Ok()
                .content_type("text/html")
                .body(template_manager.render("post.html", &context).unwrap())
        }
        Err(err) => HttpResponse::Ok().body(format!("Error: {:?}\n", err)),
    }
}

#[post("/new_post")]
async fn new_post(pool: web::Data<DBPool>, item: web::Json<NewPostHandler>) -> impl Responder {
    let mut conn = pool.get().expect("Error getting connectiong of db pool");

    match web::block(move || { Post::create_post(&mut conn, &item) }).await {
        Ok(data) => { HttpResponse::Ok().body(format!("{:?}\n", data)) }
        Err(err) => HttpResponse::Ok().body(format!("Error: {:?}\n", err)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DB URL NOT FOUND!");

    let connection = ConnectionManager::<PgConnection>::new(db_url);

    let pool = Pool::builder().build(connection).expect("Error creating pool");

    HttpServer::new(move || {
        println!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"));
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .app_data(web::Data::new(tera))
            .app_data(web::Data::new(pool.clone()))
            .service(index)
            .service(get_post)
            .service(new_post)
    })
        .bind(("0.0.0.0", 9000))?
        .run().await
}
