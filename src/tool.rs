use axum::Json;
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::iter;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseFormat<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

pub type ResponseStruct<T> = Json<ResponseFormat<T>>;

// 生成指定位数的字符串
pub fn generate(len: usize) -> String {
    const CHARSET: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut rng = rand::thread_rng();
    let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
    iter::repeat_with(one_char).take(len).collect()
}

pub fn generate_uuid() -> String {
    Uuid::new_v4().simple().to_string()
}

// 请求成功的结构体
pub fn response_format_success<T>(data: Option<T>, msg: Option<String>) -> Json<ResponseFormat<T>> {
    Json(ResponseFormat {
        code: 0,
        msg: msg.unwrap_or_else(|| "success".to_string()),
        data,
    })
}

// 请求失败的结构体
pub fn response_format_failed<T>(r: ResponseFormat<T>) -> Json<ResponseFormat<T>> {
    Json(ResponseFormat {
        code: r.code,
        msg: r.msg,
        data: r.data,
    })
}

pub fn get_current_timestamp_tz() -> DateTime<FixedOffset> {
    let time_without_zone = NaiveDateTime::from_timestamp_opt(Local::now().timestamp(), 0);
    let zoned: DateTime<FixedOffset> = DateTime::from_utc(
        time_without_zone.unwrap(),
        FixedOffset::east_opt(8 * 3600).unwrap(),
    );
    zoned
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_len() {
        assert_eq!(generate(6).len(), 6);
        assert_eq!(generate(10).len(), 10);
        assert_eq!(generate(32).len(), 32);
    }

    #[test]
    fn test_generate_content_different() {
        assert_ne!(generate(6), generate(6));
        assert_ne!(generate(10), generate(10));
        assert_ne!(generate(32), generate(32));
    }

    #[test]
    fn test_date() {
        let native_date_time = chrono::DateTime::parse_from_rfc3339("2025-12-19T16:39:57-08:00");
        if native_date_time.is_err() {
            native_date_time.expect("时间格式解析错误！");
        }
        println!("native_date_time {:?}", native_date_time);
    }

    #[test]
    fn test_response_format_success() {
        let temp_data = response_format_success(Some("mock_data"), None);
        assert_eq!(temp_data.data.unwrap(), "mock_data");
        assert_eq!(temp_data.msg, "success");
        assert_eq!(temp_data.code, 0);
    }

    #[test]
    fn test_response_format_success_msg() {
        let temp_data = response_format_success(Some("mock_data"), Some("ding!".to_string()));
        assert_eq!(temp_data.data.unwrap(), "mock_data");
        assert_eq!(temp_data.msg, "ding!");
        assert_eq!(temp_data.code, 0);
    }
}
