use actix_web::HttpMessage;
use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;

#[derive(Clone)]
pub struct TestContext {
    pub msg: String,
}

pub struct ContextInjector;

impl<S, B> Transform<S, ServiceRequest> for ContextInjector
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
        println!("Hi from start. You requested: {}", req.path());

        let context = TestContext {
            msg: "Middleware Works!".to_string(),
        };
        req.extensions_mut().insert(context);

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        })
    }
}
