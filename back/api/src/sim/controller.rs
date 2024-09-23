use std::sync::Mutex;

use actix_web::{HttpResponse, error};
use actix_web::{get, post, delete, web::Data, Result, web::Path, Responder, web::Json};

use crate::entities::sim_stat::Model as SimStat;
use crate::entities::sim::Model as Sim;
use crate::sim::service::SimService;

#[get("{id}")]
pub async fn get_sim(service: Data<SimService>, path: Path<u64>) -> Result<impl Responder> {
   let id = path.into_inner();
   match service.get_sim(id).await {
       Ok(sim) => Ok(Json(sim)),
       Err(_) => Err(error::ErrorNotFound("Sim not found"))
   }
}

#[post("")]
pub async fn post_sim(service: Data<SimService>, payload: Json<Sim>) -> Result<impl Responder> {
    let sim = payload.into_inner();
    let save = service.post_sim(sim).await;
    if save.is_err() {
        return Err(error::ErrorBadRequest("Failed to save sim"));
    } else {
        return Ok(Json(save.unwrap()));
    }
}

#[post("{id}/stat")]
pub async fn post_stat(service: Data<SimService>, path: Path<u64>, payload: Json<SimStat>) -> Result<impl Responder> {
    let sim_id = path.into_inner();
    let stat = payload.into_inner();
    let save = service.post_stat(sim_id, stat).await;
    if save.is_err() {
        return Err(error::ErrorBadRequest("Failed to save stat"));
    } else {
        return Ok(Json(save.unwrap()));
    }
}
