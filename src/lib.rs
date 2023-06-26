#![warn(missing_docs)]

//! # nmea_attack
//!
//! A libary design to be compiled as a WebAssembley (wasm) app used to launch attacks on a NMEA 2000 network


// Modules
mod nmea_msg;
mod native_functions;

//fn convert_to_char_array(number:i32) -> (*const char, i32) {
//    let number_str = number.to_string();
//    let char_array = number_str.as_bytes();
//    let array_length = char_array.len() as i32;
//    let array_ptr = char_array.as_ptr() as *const char;
//
//    (array_ptr, array_length)
//}
use std::ffi::CString;
fn count_digits(number: u32) -> usize {
    let mut count = 0;
    let mut n = number;

    while n > 0 {
        n /= 10;
        count += 1;
    }

    count
}
fn convert_to_char_array(number:u32) -> ([char; 100], i32) {
    const size: usize = 100;
    let number_str = number.to_string();
    let mut char_array: [char; size] = ['\0'; size];
    let length = number_str.chars().enumerate().take(size).map(|(i, c)| {
        char_array[i] = c;
        i + 1
    }).last().unwrap_or(0);



    return (char_array, 5)
}
/// Points to a message requested from the native read queue
static mut MSG_PTR: *const u8 = std::ptr::null();

/// links MSG_PTR to the buffer allocated for the wasm app
///
/// Sets MSG_PTR to the allocated wasm buffer so that when the native code updates the buffer, the updates can be accessed from within this app. 
/// This function is called from the native code before the main() function is executed. 
#[no_mangle]
pub extern "C" fn link_msg_buffer(char_arr: *const u8, char_array_size: i32 ){
    unsafe {
        MSG_PTR = char_arr;
    }
}

use nmea_msg::NMEAMsg;
//// Main execution loop
////
//// Called from native code as a pthread. Requests messages from the read queue, modifies certain messages, and send messages out on the appropriate send queue. 
#[no_mangle]
fn main() {

    let mut msg = NMEAMsg::default();
//
    
//
    //println!("{:?}", msg);

    //let mut msg = NMEAMsg {
    //    controller_num:
    //}
    
//    / let mut array: [char; 8] = ['1','2','3','4','5','6','7','8'];
// /
//    / let ptr = array.as_ptr();
    unsafe{

        native_functions::Test();
        let message_received: i32 = native_functions::GetMsg();
        
        if message_received == 1 {
            
            //native_functions::PrintInt32(343);
            //native_functions::PrintStr(MSG_PTR, 8);

            msg = nmea_msg::string_to_nmea_no_ref(MSG_PTR,223);
            //native_functions::PrintInt32(msg.controller_num as i32);
            //native_functions::PrintInt32(msg.priority as i32);
            //native_functions::PrintInt32(msg.pgn as i32);  
            //native_functions::PrintInt32(msg.source as i32);    
                        //let msg: NMEAMsg = nmea_msg::string_to_nmea_no_ref(MSG_PTR, 233);
            //native_functions::PrintInt32(msg.pgn as i32);
            
        }

        //let message_converted = nmea_msg::string_to_nmea(MSG_PTR, 233, &mut msg);
        //if message_converted == true {
        //    native_functions::PrintInt32(msg.controller_num as i32);
        //}

        //msg.controller_num = 5;
        //
        //native_functions::PrintInt32(msg.controller_num as i32);
        //let suc = native_functions::SendMsg(0,2,12750,22,ptr,8);}
        
    }
    
    //println!("{:?}", msg);
    

}
