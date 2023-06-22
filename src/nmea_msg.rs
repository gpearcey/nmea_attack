//! Contains a struct to represent a NMEA message and functions to convert between NMEAMsg and strings

/// Represents a NMEA 2000 Message
struct NMEAMsg {
    controller_num:         u8,
    priority:               u8,
    pgn:                    u32,
    source:                 u8,
    data_length_bytes:      u8,
    data:                   Vec<u8>  
}

/// Converts a char array to a NMEA message
fn string_to_nmea(chars: *const char, length: i32, msg: &mut NMEAMsg) -> bool{
    if (length < 10) {        
        return false;
    }

    unsafe{
        let chars = std::slice::from_raw_parts(chars, length as usize);

        msg.controller_num = chars[0].to_digit(16);
        msg.priority = chars[1].to_digit(16);

        let pgn4 = chars[2].to_digit(16) : u32;
        let pgn3 = chars[3].to_digit(16) : u32;
        let pgn2 = chars[4].to_digit(16) : u32;
        let pgn1 = chars[5].to_digit(16) : u32;
        let pgn0 = chars[6].to_digit(16) : u32;

        msg.pgn = ((pgn4 as u32) << 16) | ((pgn3 as u32) << 12) | ((pgn2 as u32) << 8) | ((pgn1 as u32) << 4) | (pgn0 as u32);

        msg.source = chars[7].to_digit(16) : u8;

        let length1 = chars[8].to_digit(16) : u8;
        let length0 = chars[9].to_digit(16) : u8;

        msg.data_length_bytes = ((length1 as u8) << 4) | (length0 as u8);

        let mut i = 0;

        while( i < msg.data_length_bytes){
            let value = chars[i+9].to_digit(16) : u8;
            msg.data.push(value);
            i += 1;
        }

    }


}

/// Converts a NMEA messages to a char array
fn nmea_to_string(){}

