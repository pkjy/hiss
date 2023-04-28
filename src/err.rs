use crate::tool;

#[derive(Debug)]
pub enum AppErrorType {
    Database,
    Template,
    Notfound,
    MissingField,
    WrongField,
}

impl std::fmt::Display for AppErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppErrorType::Database => write!(f, "Linux"),
            AppErrorType::Template => write!(f, "MacOS"),
            AppErrorType::Notfound => write!(f, "Notfound"),
            AppErrorType::MissingField => write!(f, "MissingField"),
            AppErrorType::WrongField => write!(f, "WrongField"),
        }
    }
}

type Cause = Box<dyn std::error::Error>;

#[derive(Debug)]
pub enum AppErrorItem {
    Message(String),
    Cause(Cause),
}

#[derive(Debug)]
pub struct AppError {
    pub types: AppErrorType,
    pub error: AppErrorItem,
}

impl AppError {
    pub fn new(types: AppErrorType, error: AppErrorItem) -> Self {
        tracing::info!("AppError new err!!!!!!!!!!");
        Self { types, error }
    }
    pub fn from_err(cause: Cause, types: AppErrorType) -> Self {
        tracing::info!("AppError from_err err!!!!!!!!!!");
        Self::new(types, AppErrorItem::Cause(cause))
    }
    pub fn from_msg(msg: &str, types: AppErrorType) -> Self {
        tracing::info!("AppError from_msg err!!!!!!!!!!");
        Self::new(types, AppErrorItem::Message(msg.to_string()))
    }
    pub fn notfound() -> Self {
        tracing::info!("AppError notfound err!!!!!!!!!!");
        Self::from_msg("不存在的记录", AppErrorType::Notfound)
    }

    pub fn field_empty(info: &str) -> Self {
        tracing::info!("AppError field_empty err!!!!!!!!!!");
        Self::from_msg(
            format!("请求参数不不能为空: {}", info).as_str(),
            AppErrorType::MissingField,
        )
    }

    pub fn field_struct_error(info: &str) -> Self {
        tracing::info!("AppError field_struct_error err!!!!!!!!!!");
        Self::from_msg(
            format!("请求参数格式错误，请输入正确的格式: {}", info).as_str(),
            AppErrorType::WrongField,
        )
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        tracing::info!("Display err!!!!!!!!!!");
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AppError {}

impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        tracing::info!("AppError sea_orm err!!!!!!!!!!{:?}", err);
        Self::from_err(Box::new(err), AppErrorType::Database)
    }
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        tracing::info!("AppError IntoResponse err!!!!!!!!!!{:?}", self);

        let msg = match self.error {
            AppErrorItem::Cause(err) => err.to_string(),
            AppErrorItem::Message(msg) => msg.to_string(),
            // AppErrorItem::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            // AppErrorItem::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            // AppErrorItem::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            // AppErrorItem::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };

        // tool::response_format_failed(100, Some("AppErrorAppError".to_string()));
        // 自定义预期内的错误处理
        tool::response_format_failed(tool::ResponseFormat::<String> {
            code: 100,
            msg,
            data: None,
        })
        .into_response()
    }
}

// 预期外的错误处理，原始会回复status 412 等，接下来给他封装为同一的json格式体response
// https://axum.rs/topic/roaming-axum/error-handling
// axum 官方已经提供了很多 extractor，其中包括 axum::Json。现在，我们要实现自己的 Json extractor——当然，为了避免混乱，建议取别的名字，比如MyJson等。
// 自定义 extractor 使用 derive_from_request 的方式
// 通过 CustomJson 来修饰请求体，而非axum::Json
// https://github.com/tokio-rs/axum/tree/main/examples/customize-extractor-error
use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse};
use axum_macros::FromRequest;
use serde_json::json;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(ApiError))]
pub struct CustomJson<T>(pub T);

// We create our own rejection type
#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    message: String,
}

// We implement `From<JsonRejection> for ApiError`
impl From<JsonRejection> for ApiError {
    fn from(rejection: JsonRejection) -> Self {
        tracing::info!("From JsonRejection err!!!!!!!!!!{:?}", rejection);
        Self {
            status: rejection.status(),
            message: rejection.body_text(),
        }
    }
}

// We implement `IntoResponse` so `ApiError` can be used as a response
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        tracing::info!("into_response err!!!!!!!!!!{:?}", self);

        let payload = json!({
            "code": self.status.as_u16(),
            "msg": self.message,
            "data": Some(""),
            "origin": "derive_from_request"
        });

        (self.status, axum::Json(payload)).into_response()
    }
}
