use std::str::FromStr;

use uuid::Uuid;
use chrono::{NaiveDateTime, Utc};
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager,Pool};
use diesel::{Insertable, Queryable, RunQueryDsl};
use diesel::query_dsl::methods::{FilterDsl, LimitDsl, OrFilterDsl};
use actix_web::{get,post,delete,HttpResponse};
use actix_web::web::{Path, Data};
use serde::{Deserialize, Serialize};
use crate::constants::APPLICATION_JSON;
use super::schema::likes;


#[derive(Queryable,Insertable, Serialize, Deserialize )]
#[table_name = "likes"]
struct Like {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub tweet_id: Uuid,
}
impl Like {
    fn new(tweet_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            tweet_id,
        }
    }
}


#[get("/tweets/{id}/likes")]
pub async fn get_likes_by_tweet(_path: Path<(String,)>) -> HttpResponse {
    let likes = 100;

    HttpResponse::Ok()
    .content_type(APPLICATION_JSON)
    .json(likes)
}


#[post("/tweets/{id}/likes")]
pub async fn like_tweet(path: Path<(String,)>,pool: Data<Pool<ConnectionManager::<PgConnection>>>) -> HttpResponse {
    use crate::schema::likes::dsl::*;

    let  t_id = &path.0;//&path.0.0
    let mut conn = pool.get().expect("No se pudo conectar a la BD");

    let like = Like::new(Uuid::from_str(t_id).unwrap());
    let result = diesel::insert_into(likes).values(&like).execute(&mut conn).unwrap();

    HttpResponse::Created()
    .content_type(APPLICATION_JSON)
    .json(result)
}

#[delete("/tweets/{id}/likes")]
pub async fn remove_likes(_path: Path<(String,)>) -> HttpResponse {

    HttpResponse::NoContent()
    .content_type(APPLICATION_JSON)
    .await
    .unwrap()
}