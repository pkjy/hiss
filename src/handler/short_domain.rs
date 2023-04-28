use super::{get_conn, log_error};
use crate::{entity::short_domain, param, state::AppState, tool, AppError, AppResult};
use axum::{Extension, Form, Json};

use sea_orm::{ActiveModelTrait, Set};
use std::sync::Arc;

pub async fn add(
    Extension(state): Extension<Arc<AppState>>,
    Form(cu): Form<param::CreateShortDomain>,
) -> AppResult<Json<tool::ResponseFormat<short_domain::Model>>> {
    let time = tool::get_current_timestamp_tz();
    let url = short_domain::ActiveModel {
        id: Set(tool::generate_uuid()),
        domain: Set(cu.domain),
        created_at: Set(time),
        updated_at: Set(time),
        ..Default::default()
    };
    let handler_name = "short_domain/add";
    let conn = get_conn(&state);
    let pear = url
        .insert(conn)
        .await
        .map_err(AppError::from)
        .map_err(log_error(handler_name))?;
    tracing::info!("插入结果 {:?}", pear);
    Ok(tool::response_format_success(Some(pear), None))
}
