use crate::mailer::{parse_mail_options, send_mail_smtp, MailOptions, MailerError};
use crate::RestApiConfig;
use serde::Serialize;
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::reply;

#[derive(Debug, Serialize, Clone)]
pub struct ApiErrorResponse {
    message: String,
    status_code: u16,
}

impl ApiErrorResponse {
    pub fn new(error: MailerError, status_code: StatusCode) -> Self {
        Self {
            message: error.to_string(),
            status_code: status_code.as_u16(),
        }
    }

    pub fn with_error_message(error: String, status_code: StatusCode) -> Self {
        Self {
            message: error,
            status_code: status_code.as_u16(),
        }
    }
}

pub async fn send_mail(
    api_config: RestApiConfig,
    api_token: String,
    options: MailOptions,
) -> Result<impl warp::Reply, Infallible> {
    let message = match parse_mail_options(options.clone()) {
        Ok(message) => message,
        Err(err) => {
            return Ok(reply::with_status(
                reply::json(&ApiErrorResponse::new(err, StatusCode::BAD_REQUEST)),
                StatusCode::BAD_REQUEST,
            ))
        }
    };

    log::debug!("send_mail: {:?}", options);

    let connection_options = match api_config.api_keys.get(&api_token) {
        Some(opts) => opts,
        None => {
            return Ok(reply::with_status(
                reply::json(&ApiErrorResponse::with_error_message(
                    "Unauthorized".to_string(),
                    StatusCode::UNAUTHORIZED,
                )),
                StatusCode::UNAUTHORIZED,
            ))
        }
    };

    let mail_response = match send_mail_smtp(message, connection_options.clone()).await {
        Ok(res) => res,
        Err(err) => {
            println!("{err}");
            return Ok(reply::with_status(
                reply::json(&ApiErrorResponse::new(
                    err,
                    StatusCode::INTERNAL_SERVER_ERROR,
                )),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    Ok(reply::with_status(
        reply::json(&mail_response),
        StatusCode::CREATED,
    ))
}
