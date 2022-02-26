use actix_web::{web, App, HttpServer};
use rate_limiter;
use serde::Deserialize;
use std::sync::RwLock;

#[derive(Deserialize)]
struct ReduceCount {
    key: String,
}

async fn _index(data: web::Data<rate_limiter::AsyncAtomicRateLimiter>, params: web::Query<ReduceCount>) -> String {
    let (success, available_tokens) = data.reduce(params.key.to_owned(), 1).await;
    format!("{} - {}, {}", params.key, success, available_tokens)
}

// async fn _index2(data: web::Data<RwLock<rate_limiter::RateLimiter>>, params: web::Query<ReduceCount>) -> String {
//     let (success, available_tokens) = data.write().expect("lock poisoned").reduce(params.key.to_owned(), 1);
//     format!("{} - {}, {}", params.key, success, available_tokens)
// }

async fn test() -> String {
    String::from("test")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let default_rate_limiter = web::Data::new(rate_limiter::AsyncAtomicRateLimiter::new(5, 1, 1));
    //let default_rate_limiter2 = web::Data::new(RwLock::new(rate_limiter::RateLimiter::new(5, 1, 1)));

    HttpServer::new(move || {
        App::new()
            .app_data(default_rate_limiter.clone())
            .route("/", web::get().to(_index))
            .route("/hello", web::get().to(test))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
    // HttpServer::new(move || {
    //     App::new()
    //         .app_data(default_rate_limiter2.clone())
    //         .route("/", web::get().to(_index2))
    // })
    // .bind("127.0.0.1:8088")?
    // .run()
    // .await
}