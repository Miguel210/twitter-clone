#[allow(unused_imports)]

#[macro_use] //use external macro
extern crate diesel;

use actix_web::{App,HttpServer};
use std::env;
use diesel::r2d2::{ConnectionManager,Pool};
use diesel::PgConnection;
use dotenv::dotenv;
mod tweets;
mod likes;
mod constants;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL NOT FOUND");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager).expect("could not create pool");

    HttpServer::new( move || {
        App::new()
        .app_data(pool.clone())
        .service(tweets::get_tweets)
        .service(tweets::create_tweet)
        .service(tweets::get_tweet_by_id)
        .service(likes::get_likes_by_tweet)
        .service(likes::like_tweet)
        .service(likes::remove_likes)

    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}