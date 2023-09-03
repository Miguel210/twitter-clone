#![allow(unused_imports)]

use uuid::Uuid;
use chrono::{NaiveDateTime, Utc};
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager,Pool};
use diesel::{Insertable, Queryable, RunQueryDsl};
use diesel::query_dsl::methods::{FilterDsl, LimitDsl, OrFilterDsl};
use actix_web::{get,post,HttpResponse};
use actix_web::web::{Path, Data};
use serde::{Deserialize, Serialize};
use crate::constants::APPLICATION_JSON;
use super::schema::tweets;

#[derive(Queryable,Insertable, Serialize, Deserialize )]
#[table_name = "tweets"]
struct Tweet {
    id: Uuid,
    created_at: NaiveDateTime,
    message: String,
}
impl Tweet {
    fn new(message: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            message,
        }
    }
}
//api/tweets
#[get("/tweets")]
pub async fn get_tweets(pool: Data<Pool<ConnectionManager::<PgConnection>>>) -> HttpResponse {
    use crate::schema::tweets::dsl::*;
    let mut conn = pool.get().expect("No se pudo conectar a la BD");
    let result = tweets.load::<Tweet>(&mut conn);

    let response = match result {
        Ok(tws) => tws,
        Err(_) => vec![],
    };

    HttpResponse::Ok()
    .content_type(APPLICATION_JSON)
    .json(response)
}

#[post("/tweets")]
pub async fn create_tweet(req_body: String, pool: Data<Pool<ConnectionManager::<PgConnection>>>) -> HttpResponse {
    let nuevo_tweet = Tweet::new(req_body);
    let mut conn = pool.get().expect("No se pudo conectar a la BD");

    diesel::insert_into(tweets::table)
    .values(&nuevo_tweet)
    .execute(&mut conn).expect("Error al insertar tweet");

    HttpResponse::Created()
    .content_type(APPLICATION_JSON)
    .json(&nuevo_tweet)
}

#[get("/tweets/{id}")]
pub async fn get_tweet_by_id(path: Path<(String,)>) -> HttpResponse {
    let tweet = format!("este es el tweet {:?}", path.0);

    HttpResponse::Ok()
    .content_type(APPLICATION_JSON)
    .json(tweet)
}
/*
curl -X POST http://127.0.0.1:8000/tweets -H"Content-Type: application/json"-d"Esto es un tweet interesante" 
-X POST -d'Este es un tweet muy interesante'-H"Content-type:application/json" http://localhost:8000/tweets
curl -X POST -d 'Este es un tweet muy interesante' -H "Content-Type: application/json" http://localhost:8000/tweets
*/