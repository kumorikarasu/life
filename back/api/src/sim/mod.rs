
pub mod controller;
mod service;
mod entity;

use sea_orm::DatabaseConnection;
use service::SimService;

pub fn service(db: DatabaseConnection) -> SimService {
    SimService::new(db)
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(controller::get_sim)
       .service(controller::post_sim)
       .service(controller::post_stat);
}

