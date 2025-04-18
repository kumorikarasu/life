use actix_web::dev::{forward_ready, ServiceRequest, ServiceResponse, Transform, Service};
use actix_web::Error;
use actix_web::error::ErrorUnauthorized;
use futures_util::future::{LocalBoxFuture, Ready, ok};

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
