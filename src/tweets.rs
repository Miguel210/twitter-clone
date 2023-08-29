use actix_web::{get,post,HttpResponse};
use actix_web::web::Path;

//api/tweets
#[get("/tweets")]
pub async fn get_tweets() -> HttpResponse {
    let tweets = ["tweet 1: hola","tweet 2: chao"];

    HttpResponse::Ok()
    .content_type("application/json")
    .json(tweets)
}

#[post("/tweets")]
pub async fn create_tweet() -> HttpResponse {
    let nuevo_tweet = "Este es un nuevo tweet";

    HttpResponse::Created()
    .content_type("application/json")
    .json(nuevo_tweet)
}

#[get("/tweets/{id}")]
pub async fn get_tweet_by_id(path: Path<(String,)>) -> HttpResponse {
    let tweet = format!("este es el tweet {:?}", path.0);

    HttpResponse::Ok()
    .content_type("application/json")
    .json(tweet)
}
