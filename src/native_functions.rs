//! Links to native C++ functions that can be called from the app


extern "C" {

    /// Prints a char array as a string. Used for debugging purposes.
    #[link_name = "PrintStr"]
    fn PrintStr(input: *const char);

    /// Puts a new message in the wasm app buffer from the read queue
    #[link_name = "GetMsg"]
    fn GetMsg() -> bool;
    
    /// Sends a NMEA message and specifies the controller queue it should be sent to  
    #[link_name = "SendMsg"]
    fn SendMsg(controller_number: i32, PGN: i32, source: i32, priority: i32, data_length_bytes: i32, data: [char; 223] ) -> bool;
}
