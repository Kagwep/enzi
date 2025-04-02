struct InternalState {}

type b8 = u8;
pub struct PlatformState {
    pub internal_state: InternalState
}

impl PlatformState {

    fn new() -> Self {
        PlatformState {
            internal_state: InternalState {  }
        }
    }

    fn platform_startup(&mut self, application_name: &str,x: i32, y: i32, width: i32, height: i32){

    }

    fn platform_shutdown(&mut self){

    }

    fn platform_pump_messages(&self){

    }


}


pub fn platform_allocate(size: u64, alinged: b8){

}


pub fn platform_free<T>(block: T , alinged: b8){

}

pub fn platform_zero_memory(block: &mut [u8]){

}

pub fn platform_copy_memory(dest: &mut [u8],source:  &[u8]){

}


pub fn platform_set_memory(dest: &mut [u8], value: i32){

}

pub fn platform_console_write(message: &str, color: &str){

}

pub fn platform_get_absolute_time(){

}

pub fn platform_sleep(ms: u64){

}