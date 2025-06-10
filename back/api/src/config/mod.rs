use actix_web::{web, HttpResponse, Result};
use serde::Serialize;

#[derive(Serialize)]
pub struct FirebaseConfig {
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[serde(rename = "authDomain")]
    pub auth_domain: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "storageBucket")]
    pub storage_bucket: String,
    #[serde(rename = "messagingSenderId")]
    pub messaging_sender_id: String,
    #[serde(rename = "appId")]
    pub app_id: String,
}

pub async fn get_firebase_config() -> Result<HttpResponse> {
    let config = FirebaseConfig {
        api_key: std::env::var("FIREBASE_API_KEY").unwrap_or_default(),
        auth_domain: std::env::var("FIREBASE_AUTH_DOMAIN").unwrap_or_default(),
        project_id: std::env::var("FIREBASE_PROJECT_ID").unwrap_or_default(),
        storage_bucket: std::env::var("FIREBASE_STORAGE_BUCKET").unwrap_or_default(),
        messaging_sender_id: std::env::var("FIREBASE_MESSAGING_SENDER_ID").unwrap_or_default(),
        app_id: std::env::var("FIREBASE_APP_ID").unwrap_or_default(),
    };

    Ok(HttpResponse::Ok().json(config))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/firebase", web::get().to(get_firebase_config));
}