use actix_web::{HttpResponse, error};
use actix_web::{get, post, delete, web::Data, Result, web::Path, Responder, web::Json};

use crate::life::service::LifeService;

#[get("")]
pub async fn get_life(service: Data<LifeService>) -> Result<impl Responder> {
   Ok(Json(0))
}

