use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};
use crate::{commands::{check_alive, req_diag}, filecontrol::ConfigData, station::{DataToSend, Station}};

pub enum Msg {
    DiagDataBin(DataToSend),
    CheckAlive
}

pub fn receive_communication(station: &mut Station, config: &ConfigData) {
    let sock_ip = config.get_sock_ip();
    let port = config.get_port();
    let listener = match TcpListener::bind(format!("{}:{}", sock_ip.to_string(), port)) {
        Ok(a) => a,
        Err(error) => panic!("Encountered error while binding to the IP Address: {}. Error: {}", format!("{}:{}", sock_ip.to_string(), port), error)
    };
    
    println!("Listening started, ready to accept.");

    for stream in listener.incoming() {
        match stream {
            // Invalid stream
            Err(e) => eprintln!("Bad connection request! Error: {e}"),

            // Valid stream
            Ok(mut stream) => {
                // Read the incoming command and the IP address + port of the sender.
                let cmd:&mut [u8; 256]  = &mut [0; 256];
                let _ = match stream.read(cmd) {
                    Ok(a) => a,
                    Err(error) => { eprintln!("Error while reading the socket. Error: {error}"); 0 as usize },
                };
                // Check if the IP address is valid and is IPv4. This program assumes we use IPv4 for all connections.
                match stream.peer_addr() {
                    Ok(sock) if sock.is_ipv4()  => sock,
                    Ok(sock)                    => { eprintln!("Request from an IPv6 address: {}:{}. Any communications must use IPv4", sock.ip(), sock.port()); continue; }
                    Err(error)                       => { eprintln!("Invalid socket address from peer. Error: {error}"); continue; },
                };

                // Match the command
                match String::from_utf8(cmd.to_vec()) {
                    Ok(command) if command.contains("REQDIAG") => req_diag(&mut stream, station),
                    Ok(command) if command.contains("CHECKAL") => check_alive(&mut stream, station),
                    Ok(command)                                => {eprintln!("Non-recognized command. Error: {command}"); continue;},
                    Err(error)                          => {eprintln!("Non-utf8 command. Error: {error}"); continue;}
                }
            }
        }
    }
}

pub fn send_communication(stream: &mut TcpStream, data: Msg) {
    match data {
        Msg::DiagDataBin(diag_data) => {
            let config = bincode::config::standard();
            match bincode::encode_into_std_write(&diag_data, stream, config) {
                Ok(_) => return,
                Err(error)                => { eprintln!("Error while writing to socket. Error: {error}."); },
            };
        },
        
        Msg::CheckAlive => {
            let msg = "ALIVE".as_bytes();
            match stream.write(msg) {
                Ok(_) => return,
                Err(error)               => { eprintln!("Error while writing to socket. Error: {error}."); },
            };
        }
    }

    
}
