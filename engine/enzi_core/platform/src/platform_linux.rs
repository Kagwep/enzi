use core::panic;
use std::{error::Error, ffi::CString, time::UNIX_EPOCH};
use x11::{xlib::*, xlib_xcb::{XGetXCBConnection, XSetEventQueueOwner}};
use xcb;
use xcb::ffi::*;
use std::thread;
use std::time::{Duration, SystemTime};
use xcb::ffi::xcb_connection_t; // ✅ REAL TYPE
use image::{GenericImageView};
use std::fs::File;
use std::path::Path;
use crate::errors::{self, PlatformError};

struct InternalState {
    display: *mut x11::xlib::Display,
    connection: *mut xcb::ffi::xcb_connection_t,
    window: xcb::ffi::xcb_window_t,
    screen: *mut xcb::ffi::xcb_screen_t,
    wm_protocols: xcb::ffi::xcb_atom_t,
    wm_delete_win: xcb::ffi::xcb_atom_t,
}

type b8 = u8;
pub struct PlatformState {
    pub internal_state: InternalState
}

impl PlatformState {

    pub fn new() -> Self {
        PlatformState {
            internal_state: InternalState {
                display: std::ptr::null_mut(),
                connection: std::ptr::null_mut(),
                window: 0,
                screen: std::ptr::null_mut(),
                wm_protocols: 0,
                wm_delete_win: 0,
            }
        }
    }

    

    pub fn platform_startup(&mut self,application_name: &str, x: i32, y: i32, width: i32, height: i32) -> Result<(), PlatformError> {

       let internal_state = create_internal_state(application_name, x, y, width, height)?;
       self.internal_state = internal_state;

       Ok(())

    }

    pub fn platform_shutdown(&mut self){

        unsafe {
            XAutoRepeatOn(self.internal_state.display);


            xcb_destroy_window(self.internal_state.connection,self.internal_state.window);

           
            XCloseDisplay(self.internal_state.display);

            
        }
    }

    pub fn platform_pump_messages(&self) -> bool {
        unsafe {
            // Initialize quit_flagged as mutable
            let mut quit_flagged = false;
            
            // Poll for events
            let mut event = xcb_poll_for_event(self.internal_state.connection);
            
            // Process events while there are any
            while !event.is_null() {
                // Get the event type - mask out the high bits
                let event_type = (*event).response_type & 0x7F;
                
                match event_type {
                    XCB_KEY_PRESS => {
                        // Handle key press
                    },
                    XCB_KEY_RELEASE => {
                        // Handle key release
                    },
                    XCB_BUTTON_PRESS => {
                        // Handle button press
                    },
                    XCB_BUTTON_RELEASE => {
                        // Handle button release
                    },
                    XCB_MOTION_NOTIFY => {
                        // Handle motion
                    },
                    XCB_CONFIGURE_NOTIFY => {
                        // Handle window configuration changes
                    },
                    XCB_CLIENT_MESSAGE => {
                        // Cast to client message event
                        let client_message = event as *mut xcb_client_message_event_t;
                        
                        // Check if it's a window close message
                        if (*client_message).data.data32()[0] == self.internal_state.wm_delete_win {
                            quit_flagged = true;
                        }
                    },
                    _ => {
                        // Handle other events or ignore
                    }
                }
                
                // Free the event
                libc::free(event as *mut libc::c_void);
                
                // Get the next event
                event = xcb_poll_for_event(self.internal_state.connection);
            }
            
            // Return false if we should quit, true if we should continue
            !quit_flagged
        }
    }

}


pub fn platform_allocate(size: u64, alinged: b8) -> Vec<u8>{
    Vec::with_capacity(size as usize)
}


pub fn platform_zero_memory(block: &mut [u8]){
    block.fill(0);
}

pub fn platform_copy_memory(dest: &mut [u8],source:  &[u8]){
    dest.copy_from_slice(source);
}


pub fn platform_set_memory(dest: &mut [u8], value: i32){
    dest.fill(value as u8);
}

pub fn platform_console_write(message: &str, color: &str) {
    // ANSI color codes for Linux terminal
    let color_code = match color.to_lowercase().as_str() {
        "black" => "\x1b[30m",
        "red" => "\x1b[31m",
        "green" => "\x1b[32m",
        "yellow" => "\x1b[33m",
        "blue" => "\x1b[34m",
        "magenta" => "\x1b[35m",
        "cyan" => "\x1b[36m",
        "white" => "\x1b[37m",
        "bright_black" => "\x1b[90m",   // Gray
        "bright_red" => "\x1b[91m",
        "bright_green" => "\x1b[92m",
        "bright_yellow" => "\x1b[93m",
        "bright_blue" => "\x1b[94m",
        "bright_magenta" => "\x1b[95m",
        "bright_cyan" => "\x1b[96m",
        "bright_white" => "\x1b[97m",
        "error" => "\x1b[1;31m",        // Bold red for errors
        "warning" => "\x1b[1;33m",      // Bold yellow for warnings
        "success" => "\x1b[1;32m",      // Bold green for success
        "info" => "\x1b[1;36m",         // Bold cyan for info
        _ => "\x1b[0m"                  // Default/reset color
    };
    
    // ANSI reset code to return to default terminal color
    const RESET: &str = "\x1b[0m";
    
    // Print the message with color and reset afterward
    println!("{}{}{}", color_code, message, RESET);
}

pub fn platform_get_absolute_time() -> u64 {
    unsafe {
        let mut timespec = std::mem::MaybeUninit::<libc::timespec>::uninit();
        
        // Use CLOCK_MONOTONIC for consistent time measurement unaffected by system time changes
        if libc::clock_gettime(libc::CLOCK_MONOTONIC, timespec.as_mut_ptr()) == 0 {
            let timespec = timespec.assume_init();
            
            // Convert to microseconds
            (timespec.tv_sec as u64) * 1_000_000 + (timespec.tv_nsec as u64) / 1_000
        } else {
            // Fallback if clock_gettime fails
            match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(duration) => duration.as_secs() * 1_000_000 + duration.subsec_micros() as u64,
                Err(_) => 0,
            }
        }
    }
}

pub fn platform_sleep(ms: u64) {
    // Use Rust's standard library thread::sleep
    // Convert milliseconds to a Duration
    let duration = Duration::from_millis(ms);
    thread::sleep(duration);
}


pub fn platform_console_write_error(message: &str, colour: &str){
    platform_console_write(message, colour);
    
}

 fn create_internal_state(application_name: &str,x: i32, y: i32, width: i32, height: i32) -> Result<InternalState, PlatformError> {
        unsafe {

            //open Xlib display
            let display = XOpenDisplay(std::ptr::null());

            if display.is_null(){
                return Err(PlatformError::InitializationFailed("Failed to open X display".to_string()));
            }

            XAutoRepeatOff(display);

            // get acb connection from xcb

            let raw_connection = XGetXCBConnection(display);
            let connection = raw_connection as *mut xcb::ffi::xcb_connection_t;

            if connection.is_null(){
                XCloseDisplay(display);
                return Err(PlatformError::InitializationFailed("Failed to get xcb connection from xlib display".to_string()));
            }

            //XSetEventQueueOwner(display, 1);

            // get the screen

            let setup = xcb_get_setup(connection);
            let mut iter = xcb_setup_roots_iterator(setup);
            

            // Iterate through screens and print info
            let mut screen_count = 0;
            while iter.rem > 0 {
                let screen = iter.data;
                println!("Screen {}: {}x{}", 
                        screen_count,
                        (*screen).width_in_pixels,
                        (*screen).height_in_pixels);
                
                screen_count += 1;
                xcb_screen_next(&mut iter);
            }
            
            // Reset iterator to use the first screen
            iter = xcb_setup_roots_iterator(setup);
            let screen = iter.data;

            // generate a window

            let window = xcb_generate_id(connection);

            // value for window creation, empty for now

       // Register event types.
        // XCB_CW_BACK_PIXEL = filling the window bg with a single color
        // XCB_CW_EVENT_MASK is required.
        let event_mask: u32 = XCB_CW_BACK_PIXEL | XCB_CW_EVENT_MASK;
        
        // Listen for keyboard and mouse buttons
        let event_values: u32 = XCB_EVENT_MASK_BUTTON_PRESS 
                              | XCB_EVENT_MASK_BUTTON_RELEASE
                              | XCB_EVENT_MASK_KEY_PRESS 
                              | XCB_EVENT_MASK_KEY_RELEASE
                              | XCB_EVENT_MASK_EXPOSURE
                              | XCB_EVENT_MASK_POINTER_MOTION
                              | XCB_EVENT_MASK_STRUCTURE_NOTIFY;
        
        // Values to be sent over XCB (bg color, events)
        let value_list: [u32; 2] = [(*screen).black_pixel, event_values];
        


            xcb_create_window(
                connection,
                XCB_COPY_FROM_PARENT as u8,  // depth - usually use COPY_FROM_PARENT
                window,
                (*screen).root,
                x.try_into().unwrap(), y.try_into().unwrap(),                         // x, y position
                width.try_into().unwrap(), height.try_into().unwrap(),                     // width, height
                0,                           // border width
                XCB_WINDOW_CLASS_INPUT_OUTPUT as u16,
                (*screen).root_visual,
                event_mask,
                value_list.as_ptr(),            // value list
            );

            // set wm_delete window support 

            let app_name = CString::new(application_name).unwrap();

            // Modern: _NET_WM_NAME (UTF-8)
            let net_wm_name_atom = intern_atom(connection, &CString::new("_NET_WM_NAME").unwrap());
            let utf8_string_atom = intern_atom(connection, &CString::new("UTF8_STRING").unwrap());

            xcb_change_property(
                connection,
                XCB_PROP_MODE_REPLACE as u8,
                window,
                net_wm_name_atom,
                utf8_string_atom,
                8, // bits per character
                app_name.as_bytes().len() as u32,
                app_name.as_ptr() as *const std::ffi::c_void,
            );

            // Set WM_DELETE_WINDOW protocol
            let wm_protocols = intern_atom(connection, &CString::new("WM_PROTOCOLS").unwrap());
            let wm_delete_win = intern_atom(connection, &CString::new("WM_DELETE_WINDOW").unwrap());
            xcb_change_property(
                connection,
                XCB_PROP_MODE_REPLACE as u8,
                window,
                wm_protocols,
                4,
                32,
                1,
                &wm_delete_win as *const _ as *const std::ffi::c_void,
            );

            

            match set_window_icon_from_png(connection, window, "icon.png"){
                Ok(_) => (),
                Err(e) => platform_console_write_error(&format!("{}",e), "red"),
            }

            xcb_map_window(connection, window);
            xcb_flush(connection);

            Ok(InternalState {
                display,
                connection,
                window,
                screen,
                wm_protocols,
                wm_delete_win,
            })


        }
}


// Helper function to intern an atom
unsafe fn intern_atom(connection: *mut xcb_connection_t, name: &CString) -> xcb_atom_t {
    let cookie = xcb_intern_atom(
        connection,
        0,
        name.as_bytes().len() as u16,
        name.as_ptr(),
    );
    
    let reply = xcb_intern_atom_reply(connection, cookie, std::ptr::null_mut());
    let atom = (*reply).atom;
    libc::free(reply as *mut libc::c_void);
    atom
}


pub fn set_window_icon_from_png(connection: *mut xcb_connection_t, window: xcb_window_t, png: &str) -> Result<(), Box<dyn std::error::Error>>{

    let icon = &format!("assets/{}",png);

    let path_png = Path::new(icon);

    let img = image::open(path_png)?;
    let width = img.width();
    let height = img.height();

    // prepare the icon data for x11
    // format width height pixels data where pixels data is in ARGB format

    let mut icon_data = Vec::with_capacity((width *height) as usize + 2);

    icon_data.push(width);
    icon_data.push(height);

    for y in 0..height{
        for x in 0..width{
            let pixel = img.get_pixel(x, y);
            // convert RGBA to ARGBA 
            let argb = ((pixel[3] as u32) << 24) | // Alpha
            ((pixel[0] as u32) << 16) | // Red
            ((pixel[1] as u32) << 8)  | // Green
            (pixel[2] as u32);          // Blue
             icon_data.push(argb);
        }
    }

    let new_wm_icon_atom = unsafe { intern_atom(connection, &CString::new("_NET_WM_ICON").unwrap()) };

    // xcb_change_property setting 

    unsafe {
        xcb_change_property(connection, XCB_PROP_MODE_REPLACE as u8, window, new_wm_icon_atom, XCB_ATOM_CARDINAL, 32, icon_data.len() as u32, icon_data.as_ptr() as *const std::ffi::c_void)
    };

    Ok(())
    

}