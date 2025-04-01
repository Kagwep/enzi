mod logger;
mod asserts;
use std::sync::atomic::Ordering;
use crate::logger::logger::{LOG_DEBUG_ENABLED,LOG_INFO_ENABLED,LOG_WARN_ENABLED,LOG_TRACE_ENABLED,log_message,LogLevel};
use crate::asserts::asserts::{ENZI_ASSERRTIONS_ENABLED,report_assertion_failure};
mod common;

fn main() {

   log_info!("🧱 Enzi {}", 3.14_f64);
   log_warn!("🧱 Enzi {}", 3.14_f64);
   log_debug!("🧱 Enzi {}", 3.14_f64);
   log_trace!("🧱 Enzi {}", 3.14_f64);
   log_error!("🧱 Enzi {}", 3.14_f64);

   enzi_assert!( 1 == 0 );

    
}
