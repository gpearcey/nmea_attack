//! Contains a struct to represent a NMEA message and functions to convert between NMEAMsg and strings
extern "C" {

    #[link_name = "Test"]
    pub fn Test() -> i32;

    /// Prints a char array as a string. Used for debugging purposes.
    #[link_name = "PrintStr"]
    pub fn PrintStr(input: *const u8, length: i32);

    /// Prints an integer. Used for debugging purposes.
    #[link_name = "PrintInt32"]
    pub fn PrintInt32(input: i32);


    /// Puts a new message in the wasm app buffer from the read queue
    #[link_name = "GetMsg"]
    pub fn GetMsg() -> i32;
    
    /// Sends a NMEA message and specifies the controller queue it should be sent to  
    #[link_name = "SendMsg"]
    pub fn SendMsg(controller_number: i32, priority: i32, PGN: i32, source: i32, data: *const char, data_length_bytes: i32 ) -> i32;
}
use std::str::FromStr;
/// Represents a NMEA 2000 Message
#[derive(Debug)]
pub struct NMEAMsg {
    pub controller_num:         u8,
    pub priority:               u8,
    pub pgn:                    u32,
    pub source:                 u8,
    pub data_length_bytes:      u8,
    pub data:                   Vec<u8>  
}

impl Default for NMEAMsg {
    fn default () -> NMEAMsg {
        NMEAMsg{controller_num: 9, priority: 9, pgn:13954, source:9, data_length_bytes:9, data:Vec::new()}
    }
}

/// Converts a char array to a NMEA message no reference
pub fn string_to_nmea_no_ref(chars: *const u8, length: i32) -> NMEAMsg{
    let mut msg = NMEAMsg::default();
    if length < 10 {        
        return msg;
    }

    unsafe{
        let char_array = std::slice::from_raw_parts(chars, length as usize);
        let ptr = char_array.as_ptr();
        PrintStr(chars, 9);

        //let value: char = 'a';
        let value: u8 = char_array[2];
        let mut x:u8;


        // Set controller number
        
        //let ptr_a = array.as_ptr();
        //let value: char = *ptr;
        //et value:u8 = u8::from_str_radix(*ptr, 16);

        let priority: char = '7';
        //msg.controller_num = chars[0].to_digit(16).unwrap_or(0) as u8
         //Set priority
        PrintInt32(msg.priority as i32);
        msg.priority = value;
        PrintInt32(msg.priority as i32);
         //Set PGN
        //let pgn4 = chars[2].to_digit(16).unwrap_or(0) as u32;
        //let pgn3 = chars[3].to_digit(16).unwrap_or(0) as u32;
        //let pgn2 = chars[4].to_digit(16).unwrap_or(0) as u32;
        //let pgn1 = chars[5].to_digit(16).unwrap_or(0) as u32;
        //let pgn0 = chars[6].to_digit(16).unwrap_or(0) as u32;
//
        //msg.pgn = (pgn4 << 16) | (pgn3 << 12) | (pgn2 << 8) | (pgn1 << 4) | pgn0;
////
        //// Set source
        //msg.source = chars[7].to_digit(16).unwrap_or(0) as u8;
//
        //// Set data length
        //let length1 = chars[8].to_digit(16).unwrap_or(0) as u8;
        //let length0 = chars[9].to_digit(16).unwrap_or(0) as u8;
//
        //msg.data_length_bytes = (length1 << 4) | length0;
//
        //// Set data
        //let mut i: usize = 10;
//
        //while i < ((2 * msg.data_length_bytes + 10)).into() { // each char is 4 bytes, so two elements = one 8 byte piece of data
        //    let value1 = chars[i].to_digit(16).unwrap_or(0) as u8;
        //    i += 1;
        //    let value0 = chars[i].to_digit(16).unwrap_or(0) as u8;
        //    i += 1;
        //    let value = (value1 << 4) | value0;
        //    msg.data.push(value);
        //}

    }

    return msg;
}
/// Converts a char array to a NMEA message
pub fn string_to_nmea(chars: *const char, length: i32, msg: &mut NMEAMsg) -> bool{
    if length < 10 {        
        return false;
    }

    unsafe{
        let chars = std::slice::from_raw_parts(chars, length as usize);

        // Set controller number
        msg.controller_num = chars[0].to_digit(16).unwrap_or(0) as u8;
        // Set priority
        msg.priority = chars[1].to_digit(16).unwrap_or(0) as u8;

        // Set PGN
        let pgn4 = chars[2].to_digit(16).unwrap_or(0) as u32;
        let pgn3 = chars[3].to_digit(16).unwrap_or(0) as u32;
        let pgn2 = chars[4].to_digit(16).unwrap_or(0) as u32;
        let pgn1 = chars[5].to_digit(16).unwrap_or(0) as u32;
        let pgn0 = chars[6].to_digit(16).unwrap_or(0) as u32;

        msg.pgn = (pgn4 << 16) | (pgn3 << 12) | (pgn2 << 8) | (pgn1 << 4) | pgn0;

        // Set source
        msg.source = chars[7].to_digit(16).unwrap_or(0) as u8;

        // Set data length
        let length1 = chars[8].to_digit(16).unwrap_or(0) as u8;
        let length0 = chars[9].to_digit(16).unwrap_or(0) as u8;

        msg.data_length_bytes = (length1 << 4) | length0;

        // Set data
        let mut i: usize = 10;

        while i < ((2 * msg.data_length_bytes + 10)).into() { // each char is 4 bytes, so two elements = one 8 byte piece of data
            let value1 = chars[i].to_digit(16).unwrap_or(0) as u8;
            i += 1;
            let value0 = chars[i].to_digit(16).unwrap_or(0) as u8;
            i += 1;
            let value = (value1 << 4) | value0;
            msg.data.push(value);
        }

    }

    return true;
}

/// Converts a NMEA messages to a char array
pub fn nmea_to_string(){
}

