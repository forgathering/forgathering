use actix_web::{HttpResponse, Responder, dev::ResourcePath, get, web};
use include_directory::{Dir, include_directory};

static STATIC_FILES: Dir<'_> = include_directory!("$CARGO_MANIFEST_DIR/../dist");

#[get("/{tail:.*}")]
async fn serve(path: web::Path<String>) -> impl Responder {
    let path_string = path.path();

    if let Some(file) = STATIC_FILES.get_file(path_string) {
        let file_contents = file.contents();

        return HttpResponse::Ok().body(file_contents);
    }

    if let Some(file) = STATIC_FILES.get_file(format!("{path_string}.html")) {
        let file_contents = file.contents();

        return HttpResponse::Ok().body(file_contents);
    }

    if let Some(file) =
        STATIC_FILES.get_file(if path_string.ends_with('/') || path_string.is_empty() {
            format!("{path_string}index.html")
        } else {
            format!("{path_string}/index.html")
        })
    {
        let file_contents = file.contents();

        return HttpResponse::Ok().body(file_contents);
    }

    if let Some(file) = STATIC_FILES.get_file("404.html") {
        let file_contents = file.contents();

        return HttpResponse::NotFound().body(file_contents);
    }

    HttpResponse::NotFound().body("Not Found")
}
