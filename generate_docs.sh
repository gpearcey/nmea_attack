rm -r doc/
rustdoc src/lib.rs --document-private-items --crate-name "nmea_attack"
echo "<meta http-equiv=\"refresh\" content=\"0; url=nmea_attack\">" > doc/index.html