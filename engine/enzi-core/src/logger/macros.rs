
// logging macros
#[macro_export]
macro_rules! log_error {
     ($($arg:tt)*) => {
          $crate::log_message(LogLevel::Error, file!(),line!(),format!($($arg)*));
     }
}


// logging macros
#[macro_export]
macro_rules! log_fatal {
     ($($arg:tt)*) => {
          $crate::log_message(LogLevel::Fatal, file!(),line!(),format!($($arg)*));
     }
}


#[macro_export]
macro_rules! log_info {
     ($($arg:tt)*) => {
          if LOG_INFO_ENABLED.load(Ordering::Relaxed) {
               $crate::log_message(LogLevel::Info, file!(),line!(),format!($($arg)*));
          }

     }
}


#[macro_export]
macro_rules! log_warn {
     ($($arg:tt)*) => {
          if LOG_WARN_ENABLED.load(Ordering::Relaxed) {
               $crate::log_message(LogLevel::Warn, file!(),line!(),format!($($arg)*));
          }

     }
}


#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        if $crate::LOG_DEBUG_ENABLED.load(Ordering::Relaxed) {
            $crate::log_message(LogLevel::Debug, file!(), line!(), format!($($arg)*));
        }
    }
}

#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        if LOG_TRACE_ENABLED.load(Ordering::Relaxed) {
            $crate::log_message(LogLevel::Trace, file!(), line!(), format!($($arg)*));
        }
    }
}
