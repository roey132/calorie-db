use actix_web::HttpMessage;
use std::future::{ready, Ready};
use uuid::Uuid;

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;

#[derive(Clone)]
pub struct Ctx {
    pub user_id: Uuid,
}

pub struct Context;

impl<S, B> Transform<S, ServiceRequest> for Context
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ContextMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ContextMiddleware { service }))
    }
}

pub struct ContextMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ContextMiddleware<S>
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
        let path = req.path().to_string();

        println!("Request: {}", req.path());
        let headers = req.headers();
        let id = headers.get("user_id").unwrap().to_str().unwrap();
        let user_id = Uuid::parse_str(id).unwrap();
        let context = Ctx { user_id: user_id };

        req.extensions_mut().insert(context);

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;

            println!("Request: {} finished", path);
            Ok(res)
        })
    }
}
