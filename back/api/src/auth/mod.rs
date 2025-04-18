pub mod middleware;
mod controller;
mod service;

use actix_web::web;
use sea_orm::DatabaseConnection;
use service::AuthService;

pub fn service(db: DatabaseConnection) -> AuthService {
    AuthService::new(db)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(controller::login)
       .service(controller::callback)
       .service(controller::logout);
}
