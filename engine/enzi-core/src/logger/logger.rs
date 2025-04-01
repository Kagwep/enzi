use std::sync::atomic::AtomicBool;
use chrono::Local;
use crate::common::{RED,YELLOW,GREEN,WHITE,CYAN,MAGENTA,BOLD,RESET};


#[cfg(debug_assertions)]
const INIT_DEBUG: bool = true;

#[cfg(not(debug_assertions))]
const INIT_DEBUG: bool = false;


#[cfg(debug_assertions)]
const INIT_TRACE: bool = true;

#[cfg(not(debug_assertions))]
const INIT_TRACE: bool = false;



// Runtime-configurable log level flags
pub static LOG_WARN_ENABLED: AtomicBool = AtomicBool::new(true);
pub static LOG_INFO_ENABLED: AtomicBool = AtomicBool::new(true);
pub static LOG_DEBUG_ENABLED: AtomicBool = AtomicBool::new(INIT_DEBUG);
pub static LOG_TRACE_ENABLED: AtomicBool = AtomicBool::new(INIT_TRACE);

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd, Ord)]
pub enum LogLevel {
     None = 0,
     Fatal = 1,
     Error = 2,
     Warn = 3,
     Info = 4,
     Debug = 5,
     Trace = 6
}


pub fn log_message(level: LogLevel, file:&str,line:u32, message: String){

          //get the current time for timestamp
     let now = Local::now();
          // format the log message
     let formatted_message = format!(
          "{} {}{}:{} {}{} {}",
          now.format("%Y-%m-%d %H:%M:%S%.3f"), // e.g., "2025-04-01 14:33:12.123"
          level_color(level),
          level_label(level),
          RESET,
          file,
          line,
          message,  
     );

     //print to console (for now)
     println!("{}", formatted_message);

     //panic if fatal
     if level == LogLevel :: Fatal {
          panic!("Fatal Error {}", message);
     }

}


pub fn level_label(level: LogLevel) -> &'static str {
     match level {
         LogLevel::None  => "NONE",
         LogLevel::Fatal => "FATAL",
         LogLevel::Error => "ERROR",
         LogLevel::Warn  => "WARN",
         LogLevel::Info  => "INFO",
         LogLevel::Debug => "DEBUG",
         LogLevel::Trace => "TRACE",
     }
 }
 
 pub fn level_color(level: LogLevel) -> &'static str {
     match level {
         LogLevel::None  => WHITE,    
         LogLevel::Fatal => RED,      
         LogLevel::Error => RED,      
         LogLevel::Warn  => YELLOW,  
         LogLevel::Info  => GREEN,    
         LogLevel::Debug => CYAN,     
         LogLevel::Trace => MAGENTA,  
     }
 }

pub fn level_to_string(level: LogLevel) -> &'static str {
     match level {
          LogLevel::None => "NONE",
          LogLevel::Fatal => "FATAL",
          LogLevel::Error => "ERROR",
          LogLevel::Warn => "WARN",
          LogLevel::Info => "INFO",
          LogLevel::Debug => "DEBUG",
          LogLevel::Trace => "TRACE",
     }
}



// create a log file 
pub fn initialize_logging() -> bool{

     todo!();
}

pub fn shutdown_logging(){
     //create a looging entry
    todo!();
}


