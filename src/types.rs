use std::error;

pub type Error = Box<dyn error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
pub type StaticStr = &'static str;
pub type Int = i64;
