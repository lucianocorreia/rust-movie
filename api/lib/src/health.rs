use actix_web::{get, web::Data, HttpResponse, Responder};
use sqlx::PgPool;
use tracing::info;

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[get("/version")]
async fn version(db: Data<PgPool>) -> String {
    info!("Getting version");
    let result = sqlx::query_scalar("SELECT version()")
        .fetch_one(db.get_ref())
        .await;

    match result {
        Ok(version) => version,
        Err(e) => format!("Error: {:?}", e),
    }
}
