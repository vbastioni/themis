use crate::doc::scope as doc_scope;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;

pub fn run(
    listener: std::net::TcpListener,
    db_pool: PgPool,
) -> std::io::Result<actix_web::dev::Server> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || App::new().service(doc_scope()).app_data(db_pool.clone()))
        .listen(listener)?
        .run();
    Ok(server)
}
