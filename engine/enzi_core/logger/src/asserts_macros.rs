
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_break {
    () => {

        
        if $crate::ENZI_ASSERRTIONS_ENABLED.load(std::sync::atomic::Ordering::Relaxed){
            {
                std::panic!("Debug break");
                // Or if using nightly: std::intrinsics::breakpoint();
             }
        }
    }
}

#[macro_export]
macro_rules! enzi_assert {
    ($expr:expr) => {
        {
            if $crate::ENZI_ASSERRTIONS_ENABLED.load(std::sync::atomic::Ordering::Relaxed){
                if !($expr) {
                    $crate::report_assertion_failure(stringify!($expr),"",file!(),line!());
                    $crate::debug_break!();
                }
            }
        }
    }
}

#[macro_export]
macro_rules! enzi_assert_msg {
    ($expr:expr, $($arg:tt)*) => {
        {
            if $crate::ENZI_ASSERRTIONS_ENABLED.load(std::sync::atomic::Ordering::Relaxed){
                if !($expr){
                    $crate::report_assertion_failure(stringify!($expr),format!($($arg)*),file!(),line!());
                    $crate::debug_break!();
                }
            }
        }
    }


}

#[cfg(debug_assertions)]
#[macro_export]

macro_rules! enzi_assert_debug {
    ($expr: expr) => {
        
        {
            $crate::ENZI_ASSERRTIONS_ENABLED.load(std::sync::atomic::Ordering::Relaxed){}
            if (!expr) {
                $crate::report_assertion_failure(stringify!($expr),"",file!(),line!());
                $crate::debug_break!();
            }
        }
    };
}