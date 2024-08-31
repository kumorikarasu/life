
pub mod controller;
mod service;
mod entity;

use service::LifeService;

pub fn service() -> LifeService {
    LifeService::new()
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(controller::get_life);
}
