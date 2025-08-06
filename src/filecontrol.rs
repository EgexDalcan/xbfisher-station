use std::{fs::{self, File, OpenOptions}, io::{self, BufRead, ErrorKind}};

use regex::Regex;

pub struct ConfigData {
    sock_ip: String,
    port: String,
}

impl ConfigData {
    pub fn get_sock_ip(&self) -> &str {
        &self.sock_ip
    }

    pub fn get_port(&self) -> &str {
        &self.port
    }
}

/// Reads the lines from the config file (used specifically for the config file (/etc/xbfisher/config) so writes config info if the file does not exist).
pub fn read_config() -> io::Result<io::Lines<io::BufReader<File>>> {
    let config_path = "/etc/xbfisher-station/config".to_string();
    fs::create_dir("/etc/xbfisher-station").unwrap_or_else(| error | {
        if error.kind() != ErrorKind::AlreadyExists {
            panic!("Could not create a directory for the config file. Please create the directory /etc/xbfisher-station. Error: {error}");
        }
    });
    let file = File::open(&config_path).unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound {
            let _ = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&config_path)
            .unwrap_or_else(|error|{
                panic!("Config file not found. Problem creating the config file: {}. Error: {error:?}", &config_path);
            });
            let info: String = "# To comment on this file, use a '#' at the start of the line.\n\
                                # The '#' in the middle of a line is not accepted as a comment!\n\
                                # These configurations are static. You will need to restart the program after changing.\n\
                                # Port for the server. Uses 2537 as default:\n\
                                server_port=2537\n\n\
                                # IP Address of the outfacing network interface:\n\
                                sock_ip_address=127.0.0.1\n".to_string();
            fs::write(&config_path, info).unwrap_or_else(|error| {panic!("Problem writing the use information to the config file. Error: {error}")});
            panic!("Couldn't find 'config' file. 'config' file created in /etc/xbfisher-station/config. Please configure before running again.");
        } else {
            panic!("Problem accessing the config file: {}. Error: {error:?}", &config_path)
        }
    });
    Ok(io::BufReader::new(file).lines())
}

pub fn parse_config_file(file: io::Result<io::Lines<io::BufReader<File>>>) -> ConfigData {
    let mut sock_address = "127.0.0.1".to_string();
    let mut port: String = "2537".to_string();
    if let Ok(lines) = file {
        // Consumes the iterator, returns a String
        for line in lines.flatten() {
            let com = Regex::new(r"^[#]").unwrap();
            if !line.is_empty() && !com.is_match(&line) {
                if line.contains("sock_ip_address=") {
                    sock_address = match line.split("sock_ip_address=").last() {
                        Some(addr) => addr.trim().parse().unwrap_or_else(| error | {panic!("Invalid diagnostic data interval input. Make sure it is an unsigned integer. {error}")}),
                        None => "127.0.0.1".to_string(),
                    }
                } else if line.contains("server_port=") {
                    match line.split("server_port=").last() {
                        Some(prt) => port = prt.to_string(),
                        None => ()
                    }
                } else {
                    panic!("Misconfigured config file. The line: {} is not correctly configured.", line)
                }
            };
        }
    } else {
        panic!{"Misconfigured config file."};
    }
    ConfigData { sock_ip: sock_address, port: port }
}