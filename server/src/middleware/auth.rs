use std::{
    future::{ready, Ready}, rc::Rc,
};

use actix_web::{
    body::BoxBody, dev::{Service, ServiceRequest, ServiceResponse, Transform}, http::Error, HttpResponse
};
use futures::{future::LocalBoxFuture};
use std::task::{Context, Poll};


pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: actix_web::body::MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>; // Always use BoxBody
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
        }))
    }
}


pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: actix_web::body::MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>; // Use BoxBody for consistency
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        // Extract the Authorization header
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "));

        match auth_header {
            Some(_claims) => {
                // If the Authorization header is valid, proceed with the service call
                let fut = service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res.map_into_boxed_body()) // Ensure the response body is BoxBody
                })
            }
            None => {
                // If the Authorization header is missing, return an Unauthorized response
                let (req, _) = req.into_parts();
                let response = HttpResponse::Unauthorized()
                    .body("Missing Authorization header")
                    .map_into_boxed_body(); // Make sure the response body is BoxBody
                Box::pin(async move { Ok(ServiceResponse::new(req, response)) })
            }
        }
    }
}