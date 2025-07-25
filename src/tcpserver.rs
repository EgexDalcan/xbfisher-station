use std::{io::{Read, Write}, net::{Ipv4Addr, TcpListener, TcpStream}};

use crate::{commands::{check_alive, req_data, req_diag}, filecontrol::write_error, station::Station};

// TODO: CHANGE THIS SO THAT THIS IS READ FROM A CONFIG FILE
const SOCK_IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const PORT: &str = "2537";

pub fn receive_communication(station: &mut Station) {
    let listener = match TcpListener::bind(format!("{}:{}", SOCK_IP.to_string(), PORT)) {
        Ok(a) => a,
        Err(error) => panic!("Encountered error while binding to the IP Address: {}. Error: {}", format!("{}:{}", SOCK_IP.to_string(), PORT), error)
    };
    
    println!("Listening started, ready to accept.");

    for stream in listener.incoming() {
        match stream {
            // Invalid stream
            Err(e) => {eprintln!("Bad connection request! Error: {e}"); write_error(e);},

            // Valid stream
            Ok(mut stream) => {
                println!("Incoming connection from: {}", stream.peer_addr().unwrap().ip());
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

                println!("{}", String::from_utf8(cmd.to_vec()).unwrap());
                // Match the command
                match String::from_utf8(cmd.to_vec()) {
                    Ok(command) if command.contains("REQDIAG") => req_diag(&mut stream, station),
                    Ok(command) if command.contains("CHECKAL") => check_alive(&mut stream, station),
                    Ok(command) if command.contains("REQDATA") => req_data(&mut stream, station),
                    Ok(command)                                => {eprintln!("Non-recognized command. Error: {command}"); continue;},
                    Err(error)                          => {eprintln!("Non-utf8 command. Error: {error}"); continue;}
                }
            }
        }
    }
}

pub fn send_communication(stream: &mut TcpStream, data: &[u8]) {
    match stream.write(data) {
        Ok(a) if data.len() == a => return,
        Ok(a)                    => { eprintln!("Error while writing to socket. Only wrote {a}/{} bytes.\nRetrying.", data.len()); send_communication(stream, data) },
        Err(error)               => { eprintln!("Error while writing to socket. Error: {error}.\nRetrying."); send_communication(stream, data) },
    };
}
