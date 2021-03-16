use std::io::Result;
use actix_web::{App, HttpServer, HttpResponse, Responder};
use actix_web::web::{ get, Data };
use sqlx::{Pool, Postgres, query_as};
use dotenv::dotenv;

mod settings;
mod db;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let settings = settings::Settings::from_env().unwrap();
    let pool = db::create_pool(&settings).await;

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", get().to(responder))
    })
        .bind(format!("{}:{}", settings.server_addr.clone(), settings.port.clone()))?
        .run()
        .await
}

async fn responder(db_pool: Data<Pool<Postgres>>) -> impl Responder {
    let sql = r#"
        SELECT 
            title
        FROM
            matthewbuscemi_com.publications
        WHERE
            id = $1
    "#;

    let row: (String,) = query_as(sql)
        .bind(1_i32)
        .fetch_one(db_pool.get_ref())
        .await
        .expect("could not execute query");

    HttpResponse::Ok()
        .content_type("text/html")
        .body(row.0)
}
