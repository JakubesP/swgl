use std;
use std::fmt;

pub type SWGLResult<T> = Result<T, SWGLRuntimeError>;

#[derive(Debug, Clone)]
pub struct SWGLRuntimeError {
    pub message: String,
}

impl SWGLRuntimeError {
    pub fn new(message: &str) -> Self {
        Self {
            message: String::from(message),
        }
    }
}

impl fmt::Display for SWGLRuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "SWGL Error: {}", self.message)
    }
}

impl std::error::Error for SWGLRuntimeError {
    fn description(&self) -> &str {
        &self.message
    }
}

