use std::fmt;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct CanvasError {
    message: String,
}

impl CanvasError {
    pub fn new(message: String) -> CanvasError {
        CanvasError { message }
    }
}

impl fmt::Display for CanvasError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CanvasError {{ {} }}", self.message)
    }
}

impl std::error::Error for CanvasError {}

#[macro_export]
macro_rules! canvas_error {
    ($($arg:tt)*) => { Box::new(CanvasError::new(format!($($arg)*))) }
}
