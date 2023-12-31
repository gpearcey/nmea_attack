//! Links to native C++ functions that can be called from the app


extern "C" {

    /// Prints a char array as a string. Used for debugging purposes.
    #[link_name = "PrintStr"]
    pub fn PrintStr(input: *const u8, length: i32);

    /// Prints an integer. Used for debugging purposes.
    #[link_name = "PrintInt32"]
    pub fn PrintInt32(input: i32, hex: i32);

    /// Sends a NMEA message and specifies the controller queue it should be sent to  
    #[link_name = "SendMsg"]
    pub fn SendMsg(controller_number: i32, priority: i32, PGN: i32, source: i32, data: *const u8, data_length_bytes: i32 ) -> i32;
}
