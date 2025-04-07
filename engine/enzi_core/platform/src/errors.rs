use std::fmt::write;

#[derive(Debug)]
pub enum PlatformError {
    InitializationFailed(String),
    InValidDimensions,
}


impl std::fmt::Display for PlatformError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlatformError::InitializationFailed(msg) => write!(f," Platform Initialization Failed {}", msg ),
            PlatformError::InValidDimensions => write!(f, "Invalid window dimensions"),
        
        }
    }
}

impl std::error::Error for PlatformError {}