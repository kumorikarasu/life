use actix_web::dev::{forward_ready, ServiceRequest, ServiceResponse, Transform, Service};
use actix_web::{Error, FromRequest, HttpRequest};
use actix_web::error::ErrorUnauthorized;
use futures_util::future::{LocalBoxFuture, Ready, ok, ready};
use std::pin::Pin;
use actix_web::web::Data;
use actix_session::SessionExt;
use sea_orm::{ColumnTrait, QueryFilter, DatabaseConnection};
use sea_orm::*;
use crate::entities::users::{Entity as User, Column};

pub struct AuthenticatedUser {
    pub id: i32,
    pub email: String,
    pub name: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Pin<Box<dyn futures_util::Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();
        
        Box::pin(async move {
            // Get session
            let session = req.get_session();
            
            // Retrieve user information from session
            let google_id = session.get::<String>("user_id")
                .map_err(|_| ErrorUnauthorized("Session error"))?
                .ok_or_else(|| ErrorUnauthorized("Not authenticated"))?;
                
            // Get a reference to the database
            let app_data = req.app_data::<Data<DatabaseConnection>>()
                .ok_or_else(|| ErrorUnauthorized("Database connection not available"))?;
                
            // Look up the user in the database
            let user = User::find()
                .filter(Column::GoogleId.eq(google_id))
                .one(app_data.get_ref())
                .await
                .map_err(|_| ErrorUnauthorized("Database error"))?
                .ok_or_else(|| ErrorUnauthorized("User not found"))?;
                
            Ok(AuthenticatedUser {
                id: user.id,
                email: user.email,
                name: user.name,
            })
        })
    }
}

pub struct AuthMiddleware;

impl AuthMiddleware {
    pub fn new() -> Self {
        AuthMiddleware
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService { service })
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Skip auth check for login and callback routes
        if req.path().contains("/auth/login") || req.path().contains("/auth/callback") {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }

        // Check for Authorization header
        let auth_header = req.headers().get("Authorization");
        match auth_header {
            Some(header) => {
                if let Ok(auth_str) = header.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let fut = self.service.call(req);
                        return Box::pin(async move {
                            let res = fut.await?;
                            Ok(res)
                        });
                    }
                }
            }
            None => {}
        }

        Box::pin(async move {
            Err(ErrorUnauthorized("Unauthorized"))
        })
    }
}
