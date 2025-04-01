use std::sync::atomic::{AtomicBool};
use crate::{common::RESET, logger::logger::{level_color, level_label, LogLevel}};
use  chrono::Local;



pub static ENZI_ASSERRTIONS_ENABLED: AtomicBool = AtomicBool::new(true);


pub fn report_assertion_failure(expression: &str, message: &str, file : &str, line: u32) {

    //get the current time for timestamp
    let now = Local::now();
    // format the log message
    let formatted_info = format!(
        "[{}] {} [{}] {}", 
        now.format("%Y-%m-%d %H:%M:%S%.3f"), // e.g., "2025-04-01 14:33:12.123"
        level_color(LogLevel::Fatal),
        level_label(LogLevel::Fatal),
        RESET
        
    );

    eprintln!("{} Assertion failed: '{}' at {}:{}",formatted_info, expression, file, line);
    if !message.is_empty() {
        eprintln!("Message: {}", message);
    }
}