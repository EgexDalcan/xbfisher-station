use xbfisher_station::tcpserver::{receive_communication};
use xbfisher_station::station::{self, Station};

fn main() {
    println!("Hello, world!");
    let mut me = Station::new();
    receive_communication(&mut me);
    
}
