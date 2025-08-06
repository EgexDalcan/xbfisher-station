use xbfisher_station::filecontrol::{parse_config_file, read_config};
use xbfisher_station::tcpserver::{receive_communication};
use xbfisher_station::station::Station;

fn main() {
    let mut me = Station::new();
    let config = parse_config_file(read_config());
    println!("Hello, listenning to: {}:{}", config.get_sock_ip(), config.get_port());
    receive_communication(&mut me, &config);
    
}
