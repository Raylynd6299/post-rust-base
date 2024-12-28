use super::schema::posts;
use serde::{ Serialize, Deserialize };
use diesel::prelude::*;
use diesel::pg::PgConnection;
use actix_web::{ web, HttpResponse };

#[derive(Queryable, Debug, Deserialize, Serialize)]
pub struct PostSimple {
    pub title: String,
    pub slug: String,
}

#[derive(Debug, Queryable, Deserialize, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewPostHandler {
    pub title: String,
    pub body: String,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub slug: &'a str,
}

impl Post {
    pub fn slugify(title: &String) -> String {
        return title.replace(" ", "_").to_lowercase();
    }

    pub fn create_post<'a>(
        conn: &mut PgConnection,
        post: &NewPostHandler
    ) -> Result<Post, diesel::result::Error> {
        let slug = Post::slugify(&post.title.clone());

        let new_post = NewPost {
            title: &post.title,
            body: &post.body,
            slug: &slug,
        };

        diesel::insert_into(posts::table).values(new_post).get_result::<Post>(conn)
    }
}
