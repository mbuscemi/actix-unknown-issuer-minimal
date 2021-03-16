use std::io::Result;
use actix_web::{App, HttpServer, HttpResponse, Responder};
use actix_web::web::Data;
use deadpool_postgres::{ Client, Pool };
use actix_web::web::get;
use dotenv::dotenv;

mod settings;
mod db;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    //create config, and force hosts to be empty, otherwise I won't see real error messages on Heroku
    //https://githubmemory.com/repo/bikeshedder/deadpool/issues/84
    let mut settings = settings::Settings::from_env().unwrap();
    settings.pg.hosts = Some(Vec::new());

    let pool = db::create_pool(&settings);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", get().to(responder))
    })
        .bind(format!("{}:{}", settings.server_addr.clone(), settings.port.clone()))?
        .run()
        .await
}

async fn responder(db_pool: Data<Pool>) -> impl Responder {
    let _client: Client = db_pool.get().await.expect("could not create db client from pool");

    HttpResponse::Ok()
        .content_type("text/html")
        .body("")
}
