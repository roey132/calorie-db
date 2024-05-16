use crate::{models, users, DbPool};
use actix_web::{
    http::{header::ContentType, StatusCode},
    web, HttpResponse,
};
use futures::Future;
use std::pin::Pin;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("Database Error: {0}")]
    DbError(#[from] diesel::result::Error),
    #[error("R2D2 Connection Error")]
    ConnectionError(#[from] r2d2::Error),
    #[error("Parse Error {0}")]
    ParseError(#[from] uuid::Error),
    #[error("User Unauthorized")]
    Unauthorized,
}

impl actix_web::error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ServerError::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::ParseError(_) => StatusCode::BAD_REQUEST,
            ServerError::Unauthorized => StatusCode::UNAUTHORIZED,
            ServerError::ConnectionError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

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
            let user = users::get_user_by_id(&mut conn, user_id)?;
            Ok(user)
        })
    }
}
