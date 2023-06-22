//! # nmea_attack
//!
//! A libary that compiles to a wasm binary that can be used to launch attacks on a NMEA 2000 network

//links to C++ functions
extern "C" {

    
    #[link_name = "PrintStr"]
    fn PrintStr(input: *const char);

    #[link_name = "GetMsg"]
    fn GetMsg() -> bool;

    #[link_name = "SendMsg"]
    fn SendMsg(controller_number: i32, PGN: i32, source: i32, priority: i32, data_length_bytes: i32, data: [char; 223] ) -> bool;

}

mod nmea_msg;

static mut GLOBAL_CHAR_PTR: *const char = std::ptr::null();

#[no_mangle]
pub extern "C" fn link_msg_buffer(char_arr: *const char, char_array_size: i32 ){
    unsafe {
        GLOBAL_CHAR_PTR = char_arr;
    }
}



#[no_mangle]
fn main() {

    unsafe{
        let x: i32;
        //x = printHello(11);
        //printHello(x);
        //let first_char = unsafe {*GLOBAL_CHAR_PTR};
        //
        //printStr(GLOBAL_CHAR_PTR);

        //let my_array: [char; 223] = ['x'; 223];
        //ReceiveMsgFromApp(0,12757,3,5,8,my_array);
        //SendMsgToApp();

    }
}