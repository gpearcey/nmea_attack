//! Contains a struct to represent a NMEA message and functions to convert between NMEAMsg and strings
extern "C" {

    #[link_name = "Test"]
    pub fn Test() -> i32;

    /// Prints a char array as a string. Used for debugging purposes.
    #[link_name = "PrintStr"]
    pub fn PrintStr(input: *const u8, length: i32);

    /// Prints an integer. Used for debugging purposes.
    #[link_name = "PrintInt32"]
    pub fn PrintInt32(input: i32, hex: i32);


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
    pub data:                   [u8; 223],
}

impl Default for NMEAMsg {
    fn default () -> NMEAMsg {
        NMEAMsg{controller_num: 9, priority: 9, pgn:13954, source:9, data_length_bytes:9, data: [0; 223], }
    }
}

fn unicode_convert(unicode: u8) -> u8{
    let mut num: u8;
    num = match unicode {
        0x30 => 0x0,
        0x31 => 0x1,
        0x32 => 0x2,
        0x33 => 0x3, 
        0x34 => 0x4, 
        0x35 => 0x5, 
        0x36 => 0x6, 
        0x37 => 0x7, 
        0x38 => 0x8, 
        0x39 => 0x9,  
        0x41 | 0x61 => 0xa,
        0x42 | 0x62 => 0xb, 
        0x43 | 0x63 => 0xc,
        0x44 | 0x64 => 0xd, 
        0x45 | 0x65 => 0xe, 
        0x46 | 0x66 => 0xf, 
        _ => 0xff,//error if we get here
    };
    return num
}

/// Converts a char array to a NMEA message no reference
pub fn string_to_nmea_no_ref(chars: *const u8, length: i32) -> NMEAMsg{
    let mut msg = NMEAMsg::default();
    //let mut_data = &mut msg.data; //mutable reference ti 
    if length < 10 {        
        return msg;
    }

    let id_length = 10;
    unsafe{
        let id_array = std::slice::from_raw_parts(chars, id_length as usize);
        //let ptr = char_array.as_ptr();
        PrintStr(chars, 15);

        //let value: char = 'a';
        //let value: u8 = char_array[2];
        //let mut x:u8;


        // Set controller number
        
        //let ptr_a = array.as_ptr();
        //let value: char = *ptr;
        //et value:u8 = u8::from_str_radix(*ptr, 16);

        //let priority: char = '7';
        //let t0: u8 = unicode_convert(char_array[0]);
        msg.controller_num = unicode_convert(id_array[0]);
         //Set priority
        PrintInt32(msg.priority as i32, 0);
        //let t1: u8 = unicode_convert(char_array[1]);
        msg.priority = unicode_convert(id_array[1]);
        PrintInt32(msg.priority as i32, 0);
        //Set PGN
        let pgn4 = unicode_convert(id_array[2]) as u32;
        let pgn3 = unicode_convert(id_array[3]) as u32;
        let pgn2 = unicode_convert(id_array[4]) as u32;
        let pgn1 = unicode_convert(id_array[5]) as u32;
        let pgn0 = unicode_convert(id_array[6]) as u32;
//
        msg.pgn = (pgn4 << 16) | (pgn3 << 12) | (pgn2 << 8) | (pgn1 << 4) | pgn0;
        PrintInt32(msg.pgn as i32, 0);
////
        // Set source
        msg.source = unicode_convert(id_array[7]);
//
        // Set data length
        let length1 = unicode_convert(id_array[8]);
        let length0 = unicode_convert(id_array[9]);
//
        msg.data_length_bytes = (length1 << 4) | length0;
//
        // Set data
        //let data_array = std::slice::from_raw_parts(chars.offset(10), (msg.data_length_bytes*2) as usize); //get data array now that we know the size
        //let data_array_length = msg.data_length_bytes;
        let data_array = std::slice::from_raw_parts(chars.offset(10), 223*2 as usize);
        let ptr = data_array.as_ptr();
        PrintStr(ptr, 8);
        let mut i = 0;
        let mut ms: u8 = unicode_convert(data_array[4]);
        let mut ls: u8 = unicode_convert(data_array[5]);
        let mut data_byte: u8 = (ms << 4) | ls;
        let mut count = 0;
        
        while i < (((2 * msg.data_length_bytes) -1 ) as usize) { // each char is 4 bytes, so two elements = one 8 byte piece of data
            ms = unicode_convert(data_array[i]);
            i = i + 1;
            ls = unicode_convert(data_array[i]);
            PrintInt32(ls as i32, 1);
            i += i+ 1;
            data_byte = (ms << 4) | ls;
            PrintInt32(data_byte as i32, 1);
            msg.data[count] = data_byte;
            PrintInt32(msg.data[count] as i32, 1);
            count = count + 1;
            
            //PrintInt32(m)
        }


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
            //msg.data.push(value);
        }

    }

    return true;
}

/// Converts a NMEA messages to a char array
pub fn nmea_to_string(){
}

