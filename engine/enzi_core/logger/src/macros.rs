// logging macros
#[macro_export]
macro_rules! log_error {
     ($($arg:tt)*) => {
          $crate::logger::log_message(
               $crate::logger::LogLevel::Error, 
               file!(),
               line!(),
               format!($($arg)*)
          );
     }
}

#[macro_export]
macro_rules! log_fatal {
     ($($arg:tt)*) => {{
          $crate::logger::log_message(
               $crate::logger::LogLevel::Fatal, 
               file!(),
               line!(),
               format!($($arg)*)
          );
         
     }}
}

#[macro_export]
macro_rules! log_info {
     ($($arg:tt)*) => {
          if $crate::logger::LOG_INFO_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
               $crate::logger::log_message(
                    $crate::logger::LogLevel::Info, 
                    file!(),
                    line!(),
                    format!($($arg)*)
               );
               
          }
     }
}

#[macro_export]
macro_rules! log_warn {
     ($($arg:tt)*) => {
          if $crate::logger::LOG_WARN_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
               $crate::logger::log_message(
                    $crate::logger::LogLevel::Warn, 
                    file!(),
                    line!(),
                    format!($($arg)*)
               );
          }
     }
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        if $crate::logger::LOG_DEBUG_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            $crate::logger::log_message(
                $crate::logger::LogLevel::Debug, 
                file!(), 
                line!(), 
                format!($($arg)*)
            );
        }
    }
}

#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        if $crate::logger::LOG_TRACE_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            $crate::logger::log_message(
                $crate::logger::LogLevel::Trace, 
                file!(), 
                line!(), 
                format!($($arg)*)
            );
        }
    }
}

