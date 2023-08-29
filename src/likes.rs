use actix_web::{get,post,delete,HttpResponse};
use actix_web::web::Path;
use crate::constants::APPLICATION_JSON;
#[get("/tweets/{id}/likes")]
pub async fn get_likes_by_tweet(path: Path<(String,)>) -> HttpResponse {
    let likes = 100;

    HttpResponse::Ok()
    .content_type(APPLICATION_JSON)
    .json(likes)
}


#[post("/tweets/{id}/likes")]
pub async fn like_tweet(path: Path<(String,)>) -> HttpResponse {
    let like = "ok";

    HttpResponse::Created()
    .content_type(APPLICATION_JSON)
    .json(like)
}

#[delete("/tweets/{id}/likes")]
pub async fn remove_likes(path: Path<(String,)>) -> HttpResponse {

    HttpResponse::NoContent()
    .content_type(APPLICATION_JSON)
    .await
    .unwrap()
}