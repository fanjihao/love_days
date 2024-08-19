use std::fmt;
use chrono::ParseError;
use tokio::task::JoinError;

#[derive(Debug, Clone)]
pub enum CustomError {
    InternalServerError(String),
    BadRequest(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::BadRequest(e) => write!(f, "{e}"),
            CustomError::InternalServerError(e) => write!(f, "{e}"),
        }
    }
}

impl From<std::io::Error> for CustomError {
    fn from(e: std::io::Error) -> Self {
        CustomError::InternalServerError(e.to_string())
    }
}

impl From<std::num::ParseIntError> for CustomError {
    fn from(value: std::num::ParseIntError) -> Self {
        CustomError::BadRequest(format!("格式转换异常: {}", value.to_string()))
    }
}

impl From<reqwest::Error> for CustomError {
    fn from(e: reqwest::Error) -> Self {
        CustomError::BadRequest(e.to_string())
    }
}

impl From<JoinError> for CustomError {
    fn from(value: JoinError) -> Self {
        CustomError::InternalServerError(format!("tokio 线程错误: {:#?}", value))
    }
}

impl From<serde_json::Error> for CustomError {
    fn from(value: serde_json::Error) -> Self {
        CustomError::BadRequest(format!("json 数据解析失败: {:#?}", value))
    }
}

impl From<ParseError> for CustomError {
    fn from(value: ParseError) -> Self {
        CustomError::BadRequest(format!("日期转换失败: {:#?}", value))
    }
}