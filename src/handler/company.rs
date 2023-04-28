use super::{get_conn, log_error};
use crate::{entity::company, param, state::AppState, tool, AppError, AppResult};
use axum::{
    extract::{Form, Query},
    Extension,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect,
};
use std::sync::Arc;

pub async fn list(
    Extension(state): Extension<Arc<AppState>>,
    pagination: Option<Query<param::Pagination>>,
) -> AppResult<tool::ResponseStruct<Vec<company::Model>>> {
    let handler_name = "company/index";
    let conn = get_conn(&state);

    let limit = pagination.clone().unwrap_or_default().limit();
    let offset = pagination.clone().unwrap_or_default().offset();
    let result = company::Entity::find()
        .filter(company::Column::DeletedAt.is_null())
        .order_by_desc(company::Column::Id)
        .offset(offset)
        .limit(limit)
        .all(conn)
        .await
        .map_err(AppError::from)
        .map_err(log_error(handler_name))?;

    Ok(tool::response_format_success(Some(result), None))
}

pub async fn add(
    Extension(state): Extension<Arc<AppState>>,
    Form(cu): Form<param::CreateCompany>,
) -> AppResult<tool::ResponseStruct<String>> {
    let time = tool::get_current_timestamp_tz();
    let company = company::ActiveModel {
        id: Set(tool::generate_uuid()),
        company_name: Set(cu.name),
        created_at: Set(time),
        updated_at: Set(time),
        ..Default::default()
    };
    let handler_name = "company/add";
    let conn = get_conn(&state);
    let pear: company::Model = company
        .insert(conn)
        .await
        .map_err(AppError::from)
        .map_err(log_error(handler_name))?;
    tracing::info!("插入结果 {:?}", pear);
    Ok(tool::response_format_success(Some("".to_string()), None))
}
