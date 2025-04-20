//#![feature(trace_macros)]
//trace_macros!(true);

use actix_cors::Cors;
use actix_session::{Session, SessionMiddleware};
use actix_web::{cookie::Key, get};
#[allow(unused)]

use actix_web::{App, HttpServer, web, middleware::Logger};
mod entities;
mod sim;
mod dto;
mod auth;

use auth::middleware::AuthMiddleware;
use sea_orm::DatabaseConnection;

// App state to store shared resources
pub struct AppState {
    pub db: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // TODO: Load from env/config
    let session_key = Key::from(b"r+7Ow+Ne6gArI3LnzoYTD+1WsiLzVaB09NSin1d2MlPfgldgxMw9QdxmP6E1WbfnUOi7qGK2vBgI4JlrTCo5N1f9");

    // Setup DB Connections
    let db = db_setup().await;

    // Create app state with database connection
    let app_state = web::Data::new(AppState {
        db: db.clone(),
    });

    // Initialize services
    let sim_service = sim::service(db.clone());
    let auth_service = auth::service(db.clone());

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
        let session_store = actix_session::storage::CookieSessionStore::default();
        let app_state = app_state.clone();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(SessionMiddleware::new(session_store, session_key.clone()))
            .app_data(app_state.clone())
            .service(default)
            .service(
                web::scope("api/v1")
                    .app_data(app_state.clone())
                    .service(
                        web::scope("/auth")
                            .app_data(web::Data::new(auth_service.clone()))
                            .configure(auth::configure)
                    )
                    .service(
                        web::scope("/sim")
                            .wrap(AuthMiddleware::new())
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
async fn default(session: Session) -> String {
    let g = session.get::<String>("csrf_token").unwrap().unwrap_or("None".to_string());
    println!("Session {:?}", session.entries());

    println!("Session key: {:?}", g);

    //session.insert("key", "value").unwrap();
    format!("Hello world! {:?}", g)
}

async fn db_setup() -> sea_orm::DatabaseConnection {
    let pcs = std::env::var("POSTGRES_CONNECTION_STRING").expect("POSTGRES_CONNECTION_STRING must be set");
    let db = sea_orm::Database::connect(&pcs).await.unwrap();
    db
}
