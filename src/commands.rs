use std::{net::TcpStream, str::FromStr};

use crate::{station::Station, tcpserver::send_communication};

pub fn req_diag(stream: &mut TcpStream, station: &mut Station) {
    station.get_data();
    let msg = String::from_str("Hello!").expect("Hardcoded");
    send_communication(stream, msg.as_bytes());
}

pub fn check_alive(stream: &mut TcpStream, station: &Station) {
    todo!()
}

pub fn req_data(stream: &mut TcpStream, station: &Station) {
    todo!()
}

