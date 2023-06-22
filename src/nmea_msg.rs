mod nmea_msg {
    //! Contains a struct to represent a NMEA message and functions to convert between NMEAMsg and strings

    /// Represents a NMEA 2000 Message
    strcuct NMEAMsg {
        controller_num:         u8,
        source:                 u8,
        priority:               u8.
        pgn:                    u32,
        data_length_bytes:      u8,
        data:                   [u8; 223],  
    }

    /// Converts a char array to a NMEA message
    fn string_to_nmea(){}

    /// Converts a NMEA messages to a char array
    fn nmea_to_string(){}

}