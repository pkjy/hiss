use super::{get_conn, log_error, redirect, RedirectResponse};
use crate::{
    entity::{short_url, visit},
    err, param,
    state::AppState,
    tool, AppError, AppResult,
};
use axum::extract::Query;
use axum::{
    extract::Path,
    http::{HeaderMap, HeaderValue},
    Extension,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder,
};
use std::sync::Arc;
// use chrono::DateTime;
use url::Url;

const NOT_FOUND_REDIRECT: &str = "https://pkjy.xyz";

pub async fn add(
    Extension(state): Extension<Arc<AppState>>,
    err::CustomJson(payload): err::CustomJson<param::CreateShortUrl>,
) -> AppResult<tool::ResponseStruct<param::ShortUrlResponse>> {
    // let exp_at = payload.expired_at.unwrap();
    // let native_date_time = chrono::DateTime::parse_from_rfc3339(&exp_at);
    // if native_date_time.is_err(){
    //   native_date_time.expect("时间格式解析错误！");
    // }
    // tracing::info!("native_date_time {:?}", native_date_time);

    if payload.original_url.clone().is_empty() || payload.short_domain.clone().is_empty() {
        return Err(err::AppError::field_empty("original_url short_domain"));
    }

    if Url::parse(payload.original_url.as_str()).is_err() {
        return Err(err::AppError::field_struct_error("original_url"));
    }

    let result = tool::generate(6);
    let url = short_url::ActiveModel {
        id: Set(tool::generate_uuid()),
        short_domain: Set(payload.short_domain),
        original_url: Set(payload.original_url),
        short_code: Set(result),
        // short_hash: Set("G1htWU".to_string()),
        // expired_at: Set(Some(native_date_time.unwrap())),
        ..Default::default()
    };
    let handler_name = "short_url/add";
    let conn = get_conn(&state);
    let pear = url
        .insert(conn)
        .await
        .map_err(AppError::from)
        .map_err(log_error(handler_name))?;
    tracing::info!("插入结果 {:?}", pear);
    Ok(tool::response_format_success(
        Some(param::ShortUrlResponse {
            id: pear.id.to_string(),
            short_url: format!(
                "http://{}/{}",
                pear.short_domain.to_string(),
                pear.short_code.to_string()
            ),
            original_url: pear.original_url.to_string(),
        }),
        None,
    ))
}

pub async fn do_redirect(
    Extension(state): Extension<Arc<AppState>>,
    path: Path<String>,
    headers: HeaderMap,
) -> AppResult<RedirectResponse> {
    let handler_name = "short_url/do_redirect";
    let source_ip = &headers.get("host").unwrap().to_str().unwrap();
    let conn = get_conn(&state);
    let result = short_url::Entity::find()
        .filter(short_url::Column::ShortCode.eq(path.to_string()))
        .filter(short_url::Column::ShortDomain.eq(source_ip.clone()))
        // .filter(short_url::Column::ExpireAt.lt(Local::now().naive_utc())) // 过期时间过滤，暂时不加过期时间
        .order_by_desc(short_url::Column::Id)
        .one(conn)
        .await
        .map_err(AppError::from)
        .map_err(log_error(handler_name))?;

    // 通过raw sql更新写法
    //   let result = conn.query_one(Statement::from_string(
    //     conn,
    //     "UPDATE url SET visit_count=visit_count+1 WHERE short_code=$1 and short_domain=$2 RETURNING url",
    //     &[&path.to_string(),headers.get("host").unwrap().to_str().unwrap()]
    // );

    // 更新短链的点击次数
    // 可能没必要加，但是得看情况，因为数据量大的时候，更不适合做count 还是不加了 避免大量轻量锁导致性能下降
    // let mut result_active_model: short_url::ActiveModel = result.clone().unwrap().into();
    // result_active_model.visit_count =
    //     sea_orm::ActiveValue::Set(result_active_model.visit_count.unwrap() + 1);
    // result_active_model.update(conn).await?;

    // 访问记录插入数据
    // todo: 改为异步插入
    // tracing::info!("short_url_model {:?}", short_url_model);
    let short_url_model: Option<short_url::Model> = result.clone();
    match short_url_model {
        Some(short_url_model) => {
            // thread::spawn(async move {
            //     assert_visit_info(
            //         conn,
            //         &headers,
            //         source_ip.to_string(),
            //         short_url_model.id.to_string(),
            //     )
            //     .await;
            // });

            assert_visit_info(
                conn,
                &headers,
                source_ip.to_string(),
                short_url_model.id.to_string(),
            )
            .await?;

            // let vis: visit::ActiveModel = visit::ActiveModel {
            //     ua: Set(headers
            //         .get("user-agent")
            //         .unwrap_or(&HeaderValue::from_static("empty"))
            //         .to_str()
            //         .unwrap()
            //         .to_owned()),
            //     ip: Set(source_ip.to_string()),
            //     short_url_id: Set(Some(short_url_model.id)),
            //     ..Default::default()
            // };
            // vis.insert(conn).await?;
        }
        None => {}
    }

    // let headers_c = headers.clone();
    // let result_c = result.clone();
    // thread::spawn(move || {
    //     let vis: visit::ActiveModel = visit::ActiveModel {
    //         ua: Set(headers_c
    //             .get("user-agent")
    //             .unwrap_or(&HeaderValue::from_static("empty"))
    //             .to_str()
    //             .unwrap()
    //             .to_owned()),
    //         ip: Set(source_ip.to_string()),
    //         short_url_id: Set(Some(result_c.unwrap().id.to_string())),
    //         ..Default::default()
    //     };
    //     vis.insert(conn);
    // });

    match result {
        Some(result) => redirect(result.original_url.as_str()),
        None => redirect(NOT_FOUND_REDIRECT),
    }
}

async fn assert_visit_info(
    conn: &sea_orm::DatabaseConnection,
    headers: &HeaderMap,
    source_ip: String,
    id: String,
) -> Result<(), AppError> {
    let vis: visit::ActiveModel = visit::ActiveModel {
        ua: Set(headers
            .get("user-agent")
            .unwrap_or(&HeaderValue::from_static("empty"))
            .to_str()
            .unwrap()
            .to_owned()),
        ip: Set(source_ip),
        short_url_id: Set(Some(id.to_string())),
        ..Default::default()
    };
    vis.insert(conn).await?;
    Ok(())
}

pub async fn list(
    Extension(state): Extension<Arc<AppState>>,
    pagination: Option<Query<param::Pagination>>,
) -> AppResult<tool::ResponseStruct<param::ListTemplate<short_url::Model>>> {
    let handler_name = "short_url/list";
    let conn = get_conn(&state);

    let limit = pagination.clone().unwrap_or_default().limit();
    let offset = pagination.clone().unwrap_or_default().offset();

    let paginator = short_url::Entity::find().paginate(conn, limit);

    // let item_pages = paginator
    //     .num_items_and_pages()
    //     .await
    //     .map_err(AppError::from)?;

    let items = paginator.num_items().await.map_err(AppError::from)?;

    let short_urls: Vec<short_url::Model> = paginator
        .fetch_page(offset)
        .await
        .map_err(AppError::from)
        .map_err(log_error(handler_name))?;

    //     let result = short_url::Entity::find()
    //     // .filter(short_url::Column::DeletedAt.is_null())
    //     // .order_by_desc(short_url::Column::Id)
    //     .offset(offset)
    //     .limit(limit)
    //     .all(conn)
    //     .await
    //     // .map(axum::Json)
    //     .map_err(AppError::from)
    //     .map_err(log_error(handler_name))?;
    let list = param::ListTemplate {
        results: short_urls,
        count: items,
        limit,
        offset,
    };
    Ok(tool::response_format_success(Some(list), None))
}

// async fn insert(
//     conn: &DatabaseConnection,
//     headers: &HeaderMap,
//     source_ip: &str,
//     result: Option<short_url::Model>,
// ) -> AppResult<()> {
//     let vis: visit::ActiveModel = visit::ActiveModel {
//         ua: Set(headers
//             .get("user-agent")
//             .unwrap()
//             .to_str()
//             .unwrap()
//             .to_owned()),
//         ip: Set(source_ip.to_string()),
//         short_url_id: Set(Some(result.clone().unwrap().id.to_string())),
//         ..Default::default()
//     };
//     vis.insert(conn).await?;
//     Ok(())
// }
