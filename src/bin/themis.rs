use std::io;

use sqlx::postgres::PgPoolOptions;

use themis::{configuration::Setting, startup::run};

#[actix_web::main]
async fn main() -> Result<(), io::Error> {
    let conf = Setting::get().expect("conf parsing error");
    let address = format!("{}:{}", conf.application.host, conf.application.port,);
    let listener = std::net::TcpListener::bind(&address)?;
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(conf.postgres.with_db());
    println!(
        "server listening on port {}:{}",
        &conf.application.host, conf.application.port
    );
    run(listener, connection_pool)?.await
}
