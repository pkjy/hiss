use axum::http::{header, HeaderMap, StatusCode};
use sea_orm::DatabaseConnection;

use crate::{state::AppState, AppError, AppResult};

pub mod company;
pub mod short_domain;
pub mod short_url;

type RedirectResponse = (StatusCode, HeaderMap, ());

/// 记录错误
fn log_error(handler_name: &str) -> Box<dyn Fn(AppError) -> AppError> {
    let handler_name = handler_name.to_string();
    Box::new(move |err| {
        tracing::error!("{}: {:?}", handler_name, err);
        err
    })
}

fn get_conn<'a>(state: &'a AppState) -> &'a DatabaseConnection {
    &state.conn
}

fn redirect(url: &str) -> AppResult<RedirectResponse> {
    let mut header = HeaderMap::new();
    header.insert(header::LOCATION, url.parse().unwrap());
    Ok((StatusCode::FOUND, header, ()))
}
