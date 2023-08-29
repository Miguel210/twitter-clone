use actix_web::{App,HttpServer};

mod tweets;
mod likes;
mod constants;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        App::new()
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