use std::{
    future::{Ready, ready},
    rc::Rc,
};

use actix_web::{
    HttpResponse,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    http::Error,
};
use futures::future::LocalBoxFuture;

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        // Extract and verify the Authorization header
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .and_then(|token| verify_jwt(token));

        match auth_header {
            Some(claims) => {
                let fut = service.call(req);
                Box::pin(async move { fut.await })
            }
            None => {
                let res = req.into_response(HttpResponse::Unauthorized().finish());
                Box::pin(async move { Ok(res.map_into_left_body()) })
            }
        }
    }
}
