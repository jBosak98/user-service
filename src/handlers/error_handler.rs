use serde::Serialize;
use std::convert::Infallible;
use thiserror::Error;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

#[derive(Error, Debug)]
pub enum Error {
    #[error("wrong credentials")]
    WrongCredentialsError,
    #[error("jwt token not valid")]
    JWTTokenError,
    #[error("jwt token creation error")]
    JWTTokenCreationError,
    #[error("no auth header")]
    NoAuthHeaderError,
    #[error("invalid auth header")]
    InvalidAuthHeaderError,
    #[error("parking not found")]
    WrongParkingError,
    #[error("This login is taken. Try another.")]
    LoginInUseError,
    #[error("no permission")]
    NoPermissionError,
}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    message: String,
    status: String,
}

impl warp::reject::Reject for Error {}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if let Some(error) = err.find::<Error>() {
        match error {
            Error::WrongCredentialsError => (StatusCode::FORBIDDEN, error.to_string()),
            Error::JWTTokenError => (StatusCode::UNAUTHORIZED, error.to_string()),
            Error::JWTTokenCreationError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
            Error::WrongParkingError => (StatusCode::BAD_REQUEST, error.to_string()),
            Error::LoginInUseError => (StatusCode::BAD_REQUEST, error.to_string()),
            Error::NoPermissionError => (StatusCode::UNAUTHORIZED, error.to_string()),
            _ => (StatusCode::BAD_REQUEST, error.to_string()),
        }
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    let json = warp::reply::json(&ErrorResponse {
        status: code.to_string(),
        message,
    });
    Ok(warp::reply::with_status(json, code))
}
