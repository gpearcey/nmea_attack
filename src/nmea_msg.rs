//! Contains a struct to represent a NMEA message and functions to convert between NMEAMsg and strings

/// Represents a NMEA 2000 Message
#[derive(Debug, Default)]
pub struct NMEAMsg {
    controller_num:         u8,
    priority:               u8,
    pgn:                    u32,
    source:                 u8,
    data_length_bytes:      u8,
    data:                   Vec<u8>  
}

//impl Default for NMEAMsg {
//    fn default () -> NMEAMsg {
//        NMEAMsg{controller_num: 0, priority: 0, pgn:0, source:0, data_length_bytes:0, data:Vec::new()}
//    }
//}

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
pub fn nmea_to_string(){}

