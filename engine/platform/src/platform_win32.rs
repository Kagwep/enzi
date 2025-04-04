
// 

use windows::{
    core::*, Data::Xml::Dom::*, Win32::Foundation::*, Win32::System::Threading::*,Win32::System::Console::*,
    Win32::System::Diagnostics::Debug::OutputDebugStringW,
    Win32::UI::WindowsAndMessaging::*,Win32::System::LibraryLoader::GetModuleHandleW
};

// Define color constants
pub const CONSOLE_COLOR_RED: u8 = FOREGROUND_RED | FOREGROUND_INTENSITY;
pub const CONSOLE_COLOR_YELLOW: u8 = FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_INTENSITY;
pub const CONSOLE_COLOR_GREEN: u8 = FOREGROUND_GREEN | FOREGROUND_INTENSITY;
pub const CONSOLE_COLOR_BLUE: u8 = FOREGROUND_BLUE | FOREGROUND_INTENSITY;
pub const CONSOLE_COLOR_GRAY: u8 = FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE;
pub const CONSOLE_COLOR_WHITE: u8 = FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | FOREGROUND_INTENSITY;



struct InternalState {
    h_instance: HINSTANCE,
    hwnd: HWND,
    clock_frequency: i64,  // Store the clock frequency
    start_time: i64,       // Store the application start time
}

type b8 = u8;
pub struct PlatformState {
    pub internal_state: InternalState
}

impl PlatformState {

   pub  fn new() -> Self {

        // Get the HINSTANCE for the current module
        let hinstance = unsafe { GetModuleHandleW(None) };

        // Initialize with zero values 
        let clock_frequency = 0;
        let start_time = 0;
        

        PlatformState {
            internal_state: InternalState { 
                h_instance: hinstance,
                hwnd: HWND(0),
                clock_frequency,
                start_time
            }
        }
    }

    fn platform_startup(&mut self, application_name: &str,x: i32, y: i32, width: i32, height: i32) -> bool {

        // create the window and store the ahndle
        

        unsafe  {
            // convert string t wide string

            let class_name: Vec<u16> = application_name.encode_utf16().chain(std::iter::once(0)).collect();

            let hicon = unsafe {
                LoadImageW(
                    None,                          // hInstance (None when loading from file)
                    w!("assets/icon.ico"),         // path to .ico
                    IMAGE_ICON,
                    32,
                    32,
                    LR_LOADFROMFILE,
                )
                .unwrap_or_default()
                .into() // Use into() instead of direct casting
            };

            let wnd_class = WNDCLASSW {
                hInstance: self.internal_state.h_instance,
                lpszClassName: PCWSTR(class_name.as_ptr()),
                lpfnWndProc: Some(DefWindowProcW),
                style: CS_HREDRAW | CS_VREDRAW | CS_DBLCLKS,
                cbClsExtra: 0,
                cbWndExtra: 0,
                hCursor: LoadCursor(None, IDC_ARROW),
                hbrBackground: HBRUSH(COLOR_WINDOW.0 as isize),
                ..Default::default()
            };

            let atom = RegisterClassW(&wnd_class);
            if atom == 0 {
                panic!("Failed to register window class");
            }


            let hwnd = CreateWindowExW(
                Default::default(),
                PCWSTR(class_name.as_ptr()),
                PCWSTR(class_name.as_ptr()),
                WS_OVERLAPPEDWINDOW,
                x,
                y,
                width,
                height,
                None,
                None,
                self.internal_state.h_instance,
                std::ptr::null_mut(),
            );

            if hwnd.0 == 0 {
                panic!("Failed to create window"); // TODO : use loggger - fatal
            }

            self.internal_state.hwnd = hwnd;
            ShowWindow(hwnd,SW_SHOW);

                    // Initialize timing information
            unsafe {
                QueryPerformanceFrequency(&mut self.internal_state.clock_frequency);
                QueryPerformanceCounter(&mut self.internal_state.start_time);
            }
        

            true
        }

    }

    fn platform_shutdown(&mut self) {
        if self.internal_state.hwnd.0 != 0 {
            unsafe {
                DestroyWindow(self.internal_state.hwnd);
            }
            self.internal_state.hwnd = HWND(0);
        }
    }
    

    fn platform_pump_messages(&self) -> i32{

        unsafe  {
            let mut msg = MSG::default();

            // check for messages but dont block if there are none

            while PeekMessageW(&mut msg, HWND(0), 0,0, PM_REMOVE).as_bool(){
                TranslateMessage(&msg);
                DispatchMessageW(&msg);

                if msg.message == WM_QUIT{
                    return 0;
                }
            }
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

pub fn platform_console_write(message: &str, colour: u8) -> bool {
    unsafe {
        let stdout_handle = GetStdHandle(STD_OUTPUT_HANDLE);
        if stdout_handle.is_invalid() {
            return false;
        }
        
        // Set the text color
        let mut console_info = CONSOLE_SCREEN_BUFFER_INFO::default();
        if GetConsoleScreenBufferInfo(stdout_handle, &mut console_info).as_bool() {
            // Save the original attributes to restore later
            let original_attributes = console_info.wAttributes;
            
            // Set the new color
            SetConsoleTextAttribute(stdout_handle, colour);
            
            // Convert the string to UTF-16 for WriteConsoleW
            let wide_message: Vec<u16> = message.encode_utf16().collect();
            let mut chars_written = 0u32;
            
            // Write the text
            let success = WriteConsoleW(
                stdout_handle,
                wide_message.as_ptr(),
                wide_message.len() as u32,
                &mut chars_written,
                std::ptr::null_mut(),
            ).as_bool();
            
            // Restore original color
            SetConsoleTextAttribute(stdout_handle, original_attributes);
            
            return success;
        }
        
        false
    }
}


// Define log levels
pub enum LogLevel {
    Fatal,   // Critical errors that cause the application to terminate
    Error,   // Errors that don't cause termination but indicate failure
    Warn,    // Warnings that don't indicate failure but might lead to issues
    Info,    // General information about application progress
    Debug,   // Detailed information useful for debugging
    Trace,   // Very detailed tracing information
}



// New function that uses log levels
pub fn platform_log(level: LogLevel, message: &str) -> bool {
    let (prefix, colour) = match level {
        LogLevel::Fatal => ("[FATAL]: ", CONSOLE_COLOR_RED),
        LogLevel::Error => ("[ERROR]: ", CONSOLE_COLOR_RED),
        LogLevel::Warn  => ("[WARN]:  ", CONSOLE_COLOR_YELLOW),
        LogLevel::Info  => ("[INFO]:  ", CONSOLE_COLOR_GREEN),
        LogLevel::Debug => ("[DEBUG]: ", CONSOLE_COLOR_BLUE),
        LogLevel::Trace => ("[TRACE]: ", CONSOLE_COLOR_GRAY),
    };
    
    let full_message = format!("{}{}", prefix, message);
    platform_console_write(&full_message, colour)
}

pub fn platform_debug_string(message: &str) {
    unsafe {
        // Convert the string to UTF-16 for Windows API
        let wide_message: Vec<u16> = message
            .encode_utf16()
            .chain(std::iter::once(0)) // Add null terminator
            .collect();
        
        // Output the debug string
        OutputDebugStringW(PCWSTR(wide_message.as_ptr()));
    }
}

pub fn platform_debug_format(fmt: &str, args: &[&str]) {
    // Simple formatting implementation
    let mut result = String::from(fmt);
    for arg in args {
        if let Some(pos) = result.find("{}") {
            result.replace_range(pos..pos+2, arg);
        }
    }
    platform_debug_string(&result);
}

pub fn platform_get_absolute_time() -> f64{
    unsafe {
        let mut current_time = 0i64;
        QueryPerformanceCounter(&mut current_time);
        
        // Calculate seconds since application start
        let elapsed = current_time - platform_state.internal_state.start_time;
        elapsed as f64 / platform_state.internal_state.clock_frequency as f64
    }
}

pub fn platform_sleep(ms: u64){
    unsafe {
        Sleep(ms as u32);
    }
}

pub unsafe extern "system" fn wnd_proc (
    hwnd: HWND,
    msg: u32,
    w_param: WPARAM,
    l_param: LPARAM
) -> LRESULT {
    
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        },
        WM_CLOSE => {
            // fire am event for the application to quit
            LRESULT(0)
        },
        WM_ERASEBKGND => {
            LRESULT(1)
        },
        WM_SIZE => {

            // let mut rect = RECT::default();
            // unsafe {
            //     GetClientRect(hwnd, &mut rect);
            // }
            // let width = rect.right - rect.left;
            // let height = rect.bottom - rect.top;
            LRESULT(0)

        },
        WM_KEYDOWN => {
            println!("Key down");
            LRESULT(0)
        }
        
        WM_SYSKEYDOWN => {
            println!("System key down (e.g. Alt)");
            LRESULT(0)
        }
        
        WM_KEYUP => {
            println!("Key up");
            LRESULT(0)
        }
        
        WM_SYSKEYUP => {
            println!("System key up");
            LRESULT(0)
        }
        
        WM_LBUTTONDBLCLK => {
            
            LRESULT(0)
        },
        WM_MOUSEWHEEL => {
            // let mut  delta = GET_WHEEL_DELTA_WPARAM(wparam);
            // println!("Mouse wheel delta: {}", delta);

            // if (delta != 0){
            //     delta = if delta < 0 {
            //         -1
            //     }else{
            //         1
            //     }
            // }
        
            LRESULT(0)
        },
        WM_MOUSEMOVE => {
            // let x_position = GET_X_LPARAM(l_param);
            // let y_position = GET_Y_LPARAM(l_param);
        },
        WM_LBUTTONDOWN => println!("Left click"),
        WM_RBUTTONDOWN => println!("Right click"),
        WM_MBUTTONDOWN => println!("Middle click"),
        WM_LBUTTONUP => println!("Left button released"),
        WM_RBUTTONUP => println!("Right button released"),
        WM_MBUTTONUP => println!("Middle button released"),

        WM_XBUTTONDOWN => {
            let button = HIWORD(wparam.0 as u32);
            match button {
                XBUTTON1 => println!("X1 (back) button clicked"),
                XBUTTON2 => println!("X2 (forward) button clicked"),
                _ => {}
            }
        },
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        

    }
}