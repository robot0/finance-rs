mod api;
mod middleware;

use crate::api::currency_converter::converter::convert_currency;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::resource("/convert").route(web::post().to(convert_currency)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
