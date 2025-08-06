use std::{net::TcpStream, str::FromStr};

use crate::{station::Station, tcpserver::send_communication};

pub fn req_diag(stream: &mut TcpStream, station: &mut Station) {
    station.set_last_check(0);
    let diag_data = station.get_data();
    send_communication(stream, diag_data.as_bytes());
}

pub fn check_alive(stream: &mut TcpStream, station: &mut Station) {
    station.set_last_check(0);
    let msg = String::from_str("ALIVE").expect("Hardcoded.");
    send_communication(stream, msg.as_bytes());
}
