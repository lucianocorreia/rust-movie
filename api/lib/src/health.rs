use actix_web::{get, HttpResponse};

#[get("/")]
async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("version", "0.0.1"))
        .finish()
}
