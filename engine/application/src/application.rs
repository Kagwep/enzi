use logger::{log_error, log_fatal, log_info, logger::initialize_logging};
use platform::platform_api::{platform_console_write, platform_get_absolute_time, platform_sleep, platform_zero_memory, PlatformState};

use crate::errors::ApplicationError;

pub struct  ApplicationState {
    is_running: bool,
    is_suspended: bool,
    platform_state: PlatformState,
    width: i32,
    height: i32,
    last_time: f64,
    initialized: bool,

}

pub struct  ApplicationConfig {
    pub application_name: String, 
    pub  start_pos_x: i32, 
    pub start_pos_y: i32, 
    pub width: i32, 
    pub height: i32
}

impl ApplicationState {
    pub fn new() -> Self {
        ApplicationState {
            is_running: false,
            is_suspended: false,
            platform_state: PlatformState::new(),
            width: 0,
            height: 0,
            last_time: 0.0,
            initialized: false,
        
        }
    }
}

impl ApplicationConfig {
    pub fn new (application_name: &str, start_pos_x: i32, start_pos_y: i32, width: i32, height: i32) -> Self {
        ApplicationConfig { application_name: application_name.to_string(), start_pos_x, start_pos_y, width, height}
    }


}


pub fn platform_create(config: ApplicationConfig, mut application_state: ApplicationState) -> Result<ApplicationState, ApplicationError>{

    if application_state.initialized {
        return Err(ApplicationError::ReinitializationError("".to_string()));
    }

    initialize_logging();

    application_state.is_running = true;
    application_state.is_suspended = false;

         // Initialize the platform with a window
     let platform = application_state.platform_state.platform_startup(&config.application_name, config.start_pos_x, config.start_pos_y, config.width, config.height);


    match platform {
        Ok(_) => log_info!(" Application Initialized ... "),
        Err(err) => {
            return  Err(ApplicationError::InitializationFailed(err));  
        },
    };

    application_state.initialized = true;


    Ok(application_state)



}


pub fn application_run(mut application_state: ApplicationState) -> bool {

     platform_console_write("Starting main loop...", "info");
     
     // Main application loop
     
     let start_time = platform_get_absolute_time();
     
     while application_state.is_running {
         // Process window events (returns false when the window should close)
         application_state.is_running = application_state.platform_state.platform_pump_messages();
         
         // Calculate elapsed time
         let current_time = platform_get_absolute_time();
         let elapsed = current_time - start_time;
         
         // Every second, print a status message
         if elapsed % 1_000_000 < 16_667 {  // Check roughly every frame at 60fps
             let seconds = elapsed / 1_000_000;
             if seconds % 5 == 0 && seconds > 0 {  // Every 5 seconds
                 platform_console_write(&format!("Application running for {} seconds", seconds), "yellow");
             }
         }
         
         // Example of memory operations
         let mut buffer = vec![0u8; 1024];
         platform_zero_memory(&mut buffer);
         
         // Do application-specific work here
         // ...
         
         // Limit the frame rate
         platform_sleep(16);  // ~60fps
     }
     
     application_state.is_running = false;
     // Cleanup
     application_state.platform_state.platform_shutdown();
     platform_console_write("Application terminated gracefully.", "bright_green");

     true
}