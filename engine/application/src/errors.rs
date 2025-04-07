use core::fmt;

use platform::errors::PlatformError;


#[derive(Debug)]
pub enum ApplicationError {
    // Engine initialization/shutdown errors
    InitializationFailed(PlatformError),
    ReinitializationError(String),
    ShutdownError(String),
    
    // Resource handling errors
    ResourceNotFound(String),
    ResourceLoadFailed(String),
    AssetLoadError { path: String, reason: String },
    
    // Rendering errors
    RenderingError(String),
    ShaderCompilationFailed(String),
    TextureCreationFailed(String),
    
    // Input/device errors
    InputDeviceError(String),
    ControllerDisconnected(u32),
    
    // System errors
    OutOfMemory,
    ThreadingError(String),
    
    // Game logic errors
    InvalidGameState(String),
    SceneTransitionFailed(String),
    
    // Generic errors
    InvalidOperation(String),
    NotImplemented(String),
    Unknown(String),
}

impl std::fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        match self {
            ApplicationError::InitializationFailed(msg) => write!(f, "Failed to Start {}", msg),
            ApplicationError:: ReinitializationError(msg) => write!(f,"Application already runnning {}", msg),
            _ => todo!()
        }
    }
}

impl std::error:: Error for ApplicationError {}