//#![feature(trace_macros)]
//trace_macros!(true);

use actix_cors::Cors;
use actix_web::get;
#[allow(unused)]

use actix_web::{App, HttpServer, web, middleware::Logger};
mod entities;
mod sim;
mod dto;
mod auth;

use auth::middleware::AuthMiddleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Setup DB Connections
    let db = db_setup().await;

    // Initialize services
    let sim_service = sim::service(db.clone());
    let auth_service = auth::service(db);

    // Setup a tokio task that will run the decay function every minute
    /*
    actix_web::rt::spawn(async move {
        loop {
            actix_web::rt::time::sleep(std::time::Duration::from_secs(60)).await;
            sim_service_clone.run_decay(60.0).await.unwrap();
        }
    });
    */

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .service(default)
            .service(
                web::scope("api/v1")
                    .service(
                        web::scope("/auth")
                            .app_data(web::Data::new(auth_service.clone()))
                            .configure(auth::configure)
                    )
                    .service(
                        web::scope("/sim")
                            .wrap(AuthMiddleware::new())  // Note the () here
                            .app_data(web::Data::new(sim_service.clone()))
                            .configure(sim::configure)
                    )
            )
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}

#[get("/")]
async fn default() -> &'static str {
    "Hello world!"
}

async fn db_setup() -> sea_orm::DatabaseConnection {
    let pcs = std::env::var("POSTGRES_CONNECTION_STRING").expect("POSTGRES_CONNECTION_STRING must be set");
    let db = sea_orm::Database::connect(&pcs).await.unwrap();
    db
}
