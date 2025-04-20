use std::sync::Mutex;

use actix_web::{HttpResponse, error};
use actix_web::{get, post, delete, web::Data, Result, web::Path, Responder, web::Json};

use crate::dto::sim_stat::Model as SimStat;
use crate::entities::sim::Model as Sim;
use crate::sim::service::SimService;
use crate::auth::middleware::AuthenticatedUser;

#[get("{id}")]
pub async fn get_sim(service: Data<SimService>, path: Path<u64>) -> Result<impl Responder> {
   let id = path.into_inner();
   match service.get_sim(id).await {
       Ok(sim) => Ok(Json(sim)),
       Err(_) => Err(error::ErrorNotFound("Sim not found"))
   }
}

#[get("user/{user}")]
pub async fn get_user_sims(service: Data<SimService>, path: Path<i32>) -> Result<impl Responder> {
   let id = path.into_inner();
   match service.get_user_sims(id).await {
       Ok(sims) => Ok(Json(sims)),
       Err(_) => Err(error::ErrorInternalServerError("Failed to retrieve user simulations"))
   }
}

#[get("active_user")]
pub async fn get_logged_in_user_sims(service: Data<SimService>, user: AuthenticatedUser) -> Result<impl Responder> {
   match service.get_user_sims(user.id).await {
       Ok(sims) => Ok(Json(sims)),
       Err(_) => Err(error::ErrorInternalServerError("Failed to retrieve user simulations"))
   }
}

#[post("")]
pub async fn post_sim(service: Data<SimService>, mut payload: Json<Sim>, user: AuthenticatedUser) -> Result<impl Responder> {
    let mut sim = payload.into_inner();
    // Set the user_id from the authenticated user
    sim.user_id = user.id;
    
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

#[delete("{id}/stat/{name}")]
pub async fn delete_stat(service: Data<SimService>, path: Path<(u64, String)>) -> Result<impl Responder> {
    let (sim_id, name) = path.into_inner();
    let delete = service.delete_stat(sim_id, name).await;

    if delete.is_err() {
        return Err(error::ErrorBadRequest("Failed to delete stat"));
    } else {
        return Ok(HttpResponse::Ok());
    }
}

#[delete("{id}")]
pub async fn delete_sim(service: Data<SimService>, path: Path<u64>, user: AuthenticatedUser) -> Result<impl Responder> {
    let sim_id = path.into_inner();
    match service.delete_sim(sim_id, user.id).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => {
            match e {
                sea_orm::DbErr::RecordNotFound(_) => Err(error::ErrorNotFound("Sim not found or does not belong to the user")),
                _ => Err(error::ErrorInternalServerError("Failed to delete sim"))
            }
        }
    }
}
