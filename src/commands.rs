use std::net::TcpStream;

use crate::{station::Station, tcpserver::{Msg, send_communication}};

pub fn req_diag(stream: &mut TcpStream, station: &mut Station) {
    station.set_last_check(0);
    let diag_msg = Msg::DiagDataBin(station.get_data());
    
    send_communication(stream, diag_msg);
}

pub fn check_alive(stream: &mut TcpStream, station: &mut Station) {
    station.set_last_check(0);
    let msg = Msg::CheckAlive;

    send_communication(stream, msg);
}
