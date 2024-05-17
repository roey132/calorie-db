use crate::ServerError;
use crate::{models, users, DbPool};
use actix_web::web;
use futures::Future;
use std::pin::Pin;

impl actix_web::FromRequest for models::User {
    type Error = ServerError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        println!("Request: {}", req.path());
        let req = req.clone();
        Box::pin(async move {
            let mut conn = req.app_data::<web::Data<DbPool>>().unwrap().get()?;

            let headers = req.headers();
            let id = headers
                .get("user_id")
                .ok_or(ServerError::Unauthorized)?
                .to_str()
                .map_err(|_| ServerError::Unauthorized)?;
            let user_id = uuid::Uuid::parse_str(id)?;
            let user =
                users::get_user_by_id(&mut conn, user_id).map_err(|_| ServerError::Unauthorized)?;
            Ok(user)
        })
    }
}
