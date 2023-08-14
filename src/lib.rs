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
static mut MODE_PTR: *const u8 = std::ptr::null();


/// links MSG_PTR to the message buffer allocated for the wasm app
///
/// Sets MSG_PTR to the allocated wasm buffer so that when the native code updates the buffer, the updates can be accessed from within this app. 
/// This function is called from the native code before the main() function is executed. 
#[no_mangle]
pub extern "C" fn link_msg_buffer(char_arr: *const u8, _char_array_size: i32 ){
    unsafe {
        MSG_PTR = char_arr;
    }
}

/// links MODE_PTR to the mode buffer allocated for the wasm app
///
/// Sets MODE_PTR to the allocated wasm buffer so that when the native code updates the buffer, the updates can be accessed from within this app. 
/// This function is called from the native code before the main() function is executed. 
#[no_mangle]
pub extern "C" fn link_mode_buffer(char_arr: *const u8, _char_array_size: i32 ){
    unsafe {
        MODE_PTR = char_arr;
    }
}


/// Main execution loop
///
/// Called from native code as a pthread. Reads messages from MSG_PTR, modifies certain messages, and send messages out on the appropriate send queue. 
#[no_mangle]
fn main() {

    let mut msg = NMEAMsg::default();

    unsafe{
        // Get Mode
        let id_array = std::slice::from_raw_parts(MODE_PTR, 1 as usize);
        let mode = id_array[0];

        if mode == 48
        {
            // OFF Mode
            // Do Nothing
        }
        else if (mode == 49) //0x31 in decimal
        {
            // PASSIVE Mode
            msg = nmea_msg::chars_to_nmea(MSG_PTR,nmea_msg::MAX_DATA_LENGTH_BYTES);

            if msg.controller_num == 0 {
                native_functions::SendMsg(1 as i32, msg.priority as i32, msg.pgn as i32, msg.source as i32, msg.data.as_ptr(), msg.data_length_bytes as i32); // Send to controller 1
                native_functions::SendMsg(2 as i32, msg.priority as i32, msg.pgn as i32, msg.source as i32, msg.data.as_ptr(), msg.data_length_bytes as i32); // Send to controller 2
            }
            else if msg.controller_num == 1{
                native_functions::SendMsg(0 as i32, msg.priority as i32, msg.pgn as i32, msg.source as i32, msg.data.as_ptr(), msg.data_length_bytes as i32); // Send to controller 0
                native_functions::SendMsg(2 as i32, msg.priority as i32, msg.pgn as i32, msg.source as i32, msg.data.as_ptr(), msg.data_length_bytes as i32); // Send to controller 2
            }
            else if msg.controller_num == 2{
                native_functions::SendMsg(0 as i32, msg.priority as i32, msg.pgn as i32, msg.source as i32, msg.data.as_ptr(), msg.data_length_bytes as i32); // Send to controller 0
                native_functions::SendMsg(1 as i32, msg.priority as i32, msg.pgn as i32, msg.source as i32, msg.data.as_ptr(), msg.data_length_bytes as i32); // Send to controller 1
            }    
        }
        else if mode == 50
        {
            // TODO - GPS Attack Mode
        }

    
        
    }  

}
