mod platform_linux;
//mod platform_win32;
use std::ffi::CString;
use std::time::Duration;

use platform_linux::{platform_console_write, platform_get_absolute_time, platform_sleep, platform_zero_memory};


fn main() {
    // let mut platform = platform_win32::PlatformState::new();

    // platform_win32.platform_startup("Rusty Window 🚀", 100, 100, 800, 600);

    // while platform_win32.platform_pump_messages() != 0 {
    //     // You can do game/app logic here
    // }

    // platform_win32.platform_shutdown();

     // Initialize the platform with a window
     let mut platform = platform_linux::PlatformState::new("Enzi", 100, 100, 800, 600);
    
     // Print startup message
     platform_console_write("Platform initialized successfully!", "green");
     platform_console_write("Starting main loop...", "info");
     
     // Main application loop
     let mut running = true;
     let start_time = platform_get_absolute_time();
     
     while running {
         // Process window events (returns false when the window should close)
         running = platform.platform_pump_messages();
         
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
     
     // Cleanup
     platform.platform_shutdown();
     platform_console_write("Application terminated gracefully.", "bright_green");
}
