#![warn(missing_docs)]

//! # nmea_attack
//!
//! A libary design to be compiled as a WebAssembley (wasm) app used to launch attacks on a NMEA 2000 network


// Modules
mod nmea_msg;
mod native_functions;

/// Points to a message requested from the native read queue
static mut MSG_PTR: *const char = std::ptr::null();

/// links MSG_PTR to the buffer allocated for the wasm app
///
/// Sets MSG_PTR to the allocated wasm buffer so that when the native code updates the buffer, the updates can be accessed from within this app. 
/// This function is called from the native code before the main() function is executed. 
#[no_mangle]
pub extern "C" fn link_msg_buffer(char_arr: *const char, char_array_size: i32 ){
    unsafe {
        MSG_PTR = char_arr;
    }
}

use nmea_msg::NMEAMsg;
// Main execution loop
//
// Called from native code as a pthread. Requests messages from the read queue, modifies certain messages, and send messages out on the appropriate send queue. 
//#[no_mangle]
fn main() {

    let array: [char; 20] = ['0',                                                 //Controller number
                               '5',                                                 //priority
                               '1',  'E', '9', '5', '6',                            //PGN
                               '2',                                                 //Source
                               '0', '5',                                            //Data length
                               'F', '3', '4', 'E', '9', 'E', '2', '0', '2','5'];    // Data
                               
    let array_ptr: *const char = array.as_ptr();

    let mut msg = NMEAMsg::default();

    let sucess = nmea_msg::string_to_nmea(array_ptr, 20, &mut msg);

    println!("{:?}", msg);

    //let mut msg = NMEAMsg {
    //    controller_num:
    //}


}
