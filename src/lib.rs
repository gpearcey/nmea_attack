#![warn(missing_docs)]

//! # nmea_attack
//!
//! A libary design to be compiled as a WebAssembley (wasm) app used to launch attacks on a NMEA 2000 network

use nmea_msg::NMEAMsg;

// Modules
mod nmea_msg;
mod native_functions;

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


/// Main execution loop
///
/// Called from native code as a pthread. Requests messages from the read queue, modifies certain messages, and send messages out on the appropriate send queue. 
#[no_mangle]
fn main() {

    let mut msg = NMEAMsg::default();

    unsafe{

        let message_received: i32 = native_functions::GetMsg();
        
        if message_received == 1 {

            msg = nmea_msg::chars_to_nmea(MSG_PTR,223);

            native_functions::SendMsg(msg.controller_num as i32, msg.priority as i32, msg.pgn as i32, 22, msg.data.as_ptr(), msg.data_length_bytes as i32);
            
        }

    }
   

}
