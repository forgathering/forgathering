use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(
        "
    <!DOCTYPE html>
    <html lang=\"en\">
    <head>
        <meta charset=\"UTF-8\">
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
        <title>forgather</title>
    </head>
    <body>
        <h1>hi... #hitwt</h1>
    </body>
    </html>
    ",
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 3000))?
        .run()
        .await
}
