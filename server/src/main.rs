pub mod web_server;

use actix_web::{App, HttpServer};

use crate::web_server::serve;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(serve))
        .bind(("0.0.0.0", 3000))?
        .run()
        .await
}
