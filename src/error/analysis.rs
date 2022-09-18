use thiserror::Error;

#[derive(Error, Debug)]
pub enum Analysis {
    //关键字出错
    #[error("KeywordError")]
    KeywordError,
    //类型不匹配
    #[error("TypeMismatch")]
    TypeMismatch,
    //溢出
    #[error("Overflow")]
    Overflow,
}
