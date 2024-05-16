use actix_web::HttpMessage;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use uuid::Uuid;

use crate::{users, DbPool};
use actix_web::web;

#[derive(Clone)]
pub struct Ctx {
    pub user: crate::models::User,
}

pub struct Context {
    pub pool: web::Data<DbPool>,
}

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
        ready(Ok(ContextMiddleware {
            service: service,
            pool: self.pool.clone(),
        }))
    }
}

pub struct ContextMiddleware<S> {
    service: S,
    pool: web::Data<DbPool>,
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

        let mut conn = match self.pool.get() {
            Ok(conn) => conn,
            Err(_) => {
                let response: Error = actix_web::error::ErrorFailedDependency("");
                return Box::pin(async move {
                    println!("Response: {} failed - failed to connect to database.", path);
                    Err(response)
                });
            }
        };

        if let Some(id) = headers.get("user_id") {
            let id = match id.to_str() {
                Ok(id) => id,
                Err(_) => {
                    let response: Error = actix_web::error::ErrorFailedDependency("");
                    return Box::pin(async move {
                        println!("Response: {} failed - failed to parse id as str", path);
                        Err(response)
                    });
                }
            };
            let user_id = match Uuid::parse_str(id) {
                Ok(id) => id,
                Err(_) => {
                    let response: Error = actix_web::error::ErrorFailedDependency("");
                    return Box::pin(async move {
                        println!("Response: {} failed - failed to parse id to UUID", path);
                        Err(response)
                    });
                }
            };
            let user = match users::get_user_by_id(&mut conn, user_id) {
                Ok(user) => user,
                Err(_) => {
                    let response: Error = actix_web::error::ErrorFailedDependency("");
                    return Box::pin(async move {
                        println!("Response: {} failed - failed due to db err", path);
                        Err(response)
                    });
                }
            };

            let context = Ctx { user: user };
            req.extensions_mut().insert(context);

            let fut: <S as Service<ServiceRequest>>::Future = self.service.call(req);

            return Box::pin(async move {
                let res = fut.await?;

                println!("Response: {} finished successfully!", path);
                Ok(res)
            });
        }
        let response: Error = actix_web::error::ErrorUnauthorized("");
        Box::pin(async move {
            println!("Response: {} unauthorized - user_id not found", path);
            Err(response)
        })
    }
}
