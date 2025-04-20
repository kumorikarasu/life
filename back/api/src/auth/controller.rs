use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder, Result};
use oauth2::{AuthorizationCode, CsrfToken};
use serde::{Deserialize, Serialize};
use super::service::AuthService;

#[derive(Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    token: String,
    user: UserResponse,
}

#[derive(Serialize)]
pub struct UserResponse {
    name: String,
    email: String,
    picture: Option<String>,
}

#[get("/login")]
pub async fn login(service: web::Data<AuthService>, session: Session) -> Result<impl Responder> {
    println!("Session {:?}", session.entries());
    let (auth_url, _csrf_token, pkce) = service.login(session);
    // In a real application, you'd want to store the CSRF token in a session or cookie
    Ok(HttpResponse::Found()
        .append_header(("Location", auth_url))
        .finish())
}

#[post("/callback")]
pub async fn callback(
    service: web::Data<AuthService>,
    body: web::Json<AuthRequest>,
    session: Session,
) -> Result<impl Responder> {
    println!("Callback body: {:?}", body.code);
    let code = AuthorizationCode::new(body.code.clone());
    let state = CsrfToken::new(body.state.clone());
    println!("Session {:?}", session.entries());
    session.insert("test_value", "test_value").unwrap();
    
    match service.auth(code, state, session.clone()).await {
        Ok(token) => {
            println!("Token: {}", token);
            
            // Get user data from session
            let name = session.get::<String>("user_name").unwrap().unwrap_or_default();
            let email = session.get::<String>("user_email").unwrap().unwrap_or_default();
            let picture = session.get::<String>("user_picture").unwrap();
            
            let response = AuthResponse {
                token,
                user: UserResponse {
                    name,
                    email,
                    picture,
                },
            };
            
            Ok(HttpResponse::Ok().json(response))
        },
        Err(e) => Ok(HttpResponse::Ok().body(e)),
    }
}

#[get("/logout")]
pub async fn logout() -> Result<impl Responder> {
    // In a real application, you'd want to:
    // 1. Clear the session or invalidate the JWT
    // 2. Clear cookies
    Ok(HttpResponse::Ok().finish())
}
