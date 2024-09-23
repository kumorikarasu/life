//#![feature(trace_macros)]
//trace_macros!(true);

use actix_cors::Cors;
#[allow(unused)]

use actix_web::{App, HttpServer, web};
mod entities;
mod sim;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Setup DB Connections
    let db = db_setup().await;

    // No idea how to get around this for now
    let s = sim::service(db);


    HttpServer::new(move || {
        // let cors = Cors::default().allow_any_origin().allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"]).send_wildcard();
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(web::scope("api/v1/sim")
                .app_data(web::Data::new(s.clone()))
                .configure(sim::configure))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}

async fn db_setup() -> sea_orm::DatabaseConnection {
    let pcs = std::env::var("POSTGRES_CONNECTION_STRING").expect("POSTGRES_CONNECTION_STRING must be set");
    let db = sea_orm::Database::connect(&pcs).await.unwrap();
    db
}
