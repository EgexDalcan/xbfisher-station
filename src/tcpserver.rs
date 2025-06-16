use std::{io::{Read, Write}, net::{Ipv4Addr, TcpListener}};

use crate::filecontrol::write_error;

/// TODO: CHANGE THIS SO THAT THIS IS READ FROM A CONFIG FILE
const DATA_CENTER_IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const PORT: &str = "2537";

pub fn receive_communication() {
    let listener = TcpListener::bind(format!("{}:{}", DATA_CENTER_IP.to_string(), PORT)).unwrap();
    let cmd:&mut [u8; 2048]  = &mut [0; 2048];
    println!("listening started, ready to accept");
    for stream in listener.incoming() {
        match stream {
            Err(e) => {println!("Bad connection request! Error: {e}"); write_error(e);},
            Ok(mut stream) => {
                let _ = stream.read(cmd); // TODO: Check bytes written to see if reached end of page.

                let rmsg = String::from_utf8(cmd.to_vec()).expect("Hardcoded.");
                println!("They said: {}", rmsg);

                let reply = format!("Hello! How may I help you?");
                println!("Writing back: {}", reply);
                let _ = stream.write(reply.as_bytes());
            }
        }
    }
}