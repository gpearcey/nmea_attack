use nmea_msg::NMEAMsg;

let mut last_lat;
let mut last_lon;

/// Modifies a message with latitude and longitude
pub fn lat_lon_attack(msg: NMEAMsg) -> NMEAMsg{

    // get lat and lon based on PNG
    // get the difference of each
    lat_diff = lat - last_lat;
    fake_lat = lat - lat_diff;
    lon_diff = lon - last_lon;
    fake_lon = lon - lon_diff;

    // put fake data into messages

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


