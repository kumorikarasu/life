use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder, Result};
use oauth2::{AuthorizationCode, CsrfToken};
use serde::Deserialize;
use super::service::AuthService;

#[derive(Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
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
    
    match service.auth(code, state, session).await {
        Ok(token) => {
            println!("Token: {}", token);
            // In a real application, you'd want to:
            // 1. Create a session or JWT
            // 2. Set it in a cookie
            // 3. Redirect to the frontend
            Ok(HttpResponse::Ok().body(token))
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
