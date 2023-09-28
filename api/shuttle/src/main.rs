use actix_web::{
    get,
    web::{Data, ServiceConfig},
};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::{Executor, PgPool};
use tracing::info;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
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

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // initialize the database if not already initialized
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;
    let pool = Data::new(pool);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(pool).service(hello_world).service(version);
    };

    Ok(config.into())
}
