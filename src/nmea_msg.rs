//! Contains a struct to represent a NMEA message and functions to convert between NMEAMsg and strings
//!
//! Messages are obtained from MSG_PTR which points to a buffer of u8s. 
//! The u8 values are unicode for the hexadecimal values that make up the message.
//! Each unicode value represents a nibble of data, as we can only represent 0x0 to 0xF. 
//! Some values such as PGN and data_length_bytes are too large to fit within a single 
//! nibble so data is transmit most significant nibble first. 
//! 
//! Messages are received from the buffer in the following format
//! ```
//! buffer[0] = controller number
//! buffer[1] = priority
//! buffer[2] = most significant nibble of PGN
//! buffer[3] = 2nd nibble of PGN
//! buffer[4] = 3rd nibble of PGN
//! buffer[5] = 4th nibble of PGN
//! buffer[6] = least significan nibble of PGN
//! buffer[7] = source
//! buffer[8] = most significant nibble of data_length in bytes
//! buffer[9] = least significant nibble of data_length in bytes
//! buffer[10] and beyond = data 
//!```
//! Note that `buffer[10]` and `buffer[11]` would form the first byte of data
//! 
//! ## Example Message Conversion
//! 
//! An example message obtained from the buffer might look like this:
//!
//! `0x30 0x32 0x32 0x46 0x31 0x31 0x32 0x33 0x31 0x38 0x46 0x46 0x46 0x45 0x46 0x46 0x46 0x46 0x37 0x46 0x46 0x46 0x37 0x46 0x46 0x43`
//! 
//! Those unicode values represent the following numbers:
//!
//! `0x0 0x2 0x1 0xF 0x1 0x1 0x2 0x3 0x0 0x8 0xF 0xF 0xF 0xE 0xF 0xF 0xF 0xF 0x7 0xF 0xF 0xF 0x7 0xF 0xF 0xC`
//!
//! Which then translates to:
//!
//! controller number = 0x00
//!
//! priority = 0x02 
//!
//! PGN = 0x01F112
//!
//! source = 0x03
//!
//! data_length_bytes = 0x08
//!
//! data = [0xFF, 0xFE, 0xFF, 0xFF, 0x7F, 0xFF, 0x7F, 0xFC]
//!
//!



//only here temp. will be deleted after
extern "C" {

    // Prints a char array as a string. Used for debugging purposes.
    #[link_name = "PrintStr"]
    pub fn PrintStr(input: *const u8, length: i32);

    // Prints an integer. Used for debugging purposes.
    #[link_name = "PrintInt32"]
    pub fn PrintInt32(input: i32, hex: i32);
}

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

/// Used to create a message with default values that are easier to identify as errors. If they don't get set properly, it will be easier to tell.
impl Default for NMEAMsg {
    fn default () -> NMEAMsg {
        NMEAMsg{controller_num: 3,    // there are only 3 controllers labeled 0,1,2
                priority: 0,
                pgn:0,                // no pgns are 0         
                source: 255,          // unlikely that the source is 255
                data_length_bytes:0,  // data)length bytes should always be greater than 0
                data: [0; 223],
        }
    }
}

/// Converts a unicode number to the actual value it represents. 
///
/// Only meant for unicode inputs between 0x30 - 0x39 (0-9) and 0x41 - 0x46 (A - F)
fn unicode_convert(unicode: u8) -> u8{
    let num: u8;
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

/// Converts a char array to a NMEA message
pub fn chars_to_nmea(chars: *const u8, length: i32) -> NMEAMsg{
    let mut msg = NMEAMsg::default();
    let id_length = 10;

    // a valid message has at least 10 elements
    if length < id_length {      
        return msg;
    }


    unsafe{
        let id_array = std::slice::from_raw_parts(chars, id_length as usize);

        // Set controller number
        msg.controller_num = unicode_convert(id_array[0]);
         
        //Set priority
        msg.priority = unicode_convert(id_array[1]);

        //Set PGN
        let pgn4 = unicode_convert(id_array[2]) as u32;
        let pgn3 = unicode_convert(id_array[3]) as u32;
        let pgn2 = unicode_convert(id_array[4]) as u32;
        let pgn1 = unicode_convert(id_array[5]) as u32;
        let pgn0 = unicode_convert(id_array[6]) as u32;

        msg.pgn = (pgn4 << 16) | (pgn3 << 12) | (pgn2 << 8) | (pgn1 << 4) | pgn0;

        // Set source
        msg.source = unicode_convert(id_array[7]);

        // Set data length
        let length1 = unicode_convert(id_array[8]);
        let length0 = unicode_convert(id_array[9]);

        msg.data_length_bytes = (length1 << 4) | length0;

        // Set data
        let data_array = std::slice::from_raw_parts(chars.offset(10), 223*2 as usize);

        let mut i = 0;
        let mut count = 0;
        let mut ms: u8; // most significant nibble
        let mut ls: u8; // least significant nibble
        let mut data_byte: u8; //complete byte of data

        
        while i < (((2 * msg.data_length_bytes) -1 ) as usize) { 
            ms = unicode_convert(data_array[i]);
            i = i + 1;
            ls = unicode_convert(data_array[i]);
            i += i + 1;
            data_byte = (ms << 4) | ls;

            msg.data[count] = data_byte;

            count = count + 1;
        }
    }

    return msg;
}


