use std::{fs::read_to_string, thread, time::{Duration, SystemTime}};

use sysinfo::Disks;
use systemstat::{saturating_sub_bytes, Platform, System};

pub struct Station {
    sys: System,
    disks: Disks,
    last_check: usize,
}

impl Station {
    pub fn new() -> Station {
        Station { sys: System::new(), disks: Disks::new_with_refreshed_list(), last_check: 0 }
    }

    pub fn get_last_check(&mut self) -> usize {
        return self.last_check
    }

    pub fn set_last_check(&mut self, value: usize) {
        self.last_check = value;
    }

    pub fn get_data(&mut self) -> String {
        let date = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("The system clock shows time before UNIX EPOCH!");
        let data: String = format!("StartDiag\nDate:\n{}\nEnd\n{}\nEnd\n{}\nEnd\n{}\nEnd\n{}\nEnd\n{}\nEnd\n{}\nEnd\n{}\nEnd\n{}\nEnd\n{}\nENDAll",
        date.as_secs(),
        // Uptime
        match self.sys.uptime() {
            Ok(uptime) => format!("System Uptime:\n{:?}", uptime),
            Err(x) => format!("Error getting uptime data: {}\n", x)
        },
        // Network
        match self.sys.networks() {
            Ok(netifs) => {
                let mut list: String = format!("Network Data:\n");
                for netif in netifs.values() {
                    list = list + format!("Interface:\nName: ({})\nData: ({:?})\nStatistics: ({:?})\nENDInt\n", netif.name, netif.addrs, self.sys.network_stats(&netif.name)).as_str();
                }
                list.trim_end().to_string()
            }
            Err(x) => format!("Error getting network data: {}", x)
        },
        // Network Sockets
        match self.sys.socket_stats() {
            Ok(stats) => format!("System socket statistics:\n{:?}", stats),
            Err(x) => format!("Error getting system socket statistics: {}", x)
        },
        // Memory
        match self.sys.memory() {
            Ok(mem) => format!("Memory:\n{} / {} total\nDetails: ({:?})", saturating_sub_bytes(mem.total, mem.free), mem.total, mem.platform_memory),
            Err(x) => format!("Error getting memory data: {}", x)
        },
        // Swap Memory
        match self.sys.swap() {
            Ok(swap) => format!("Swap Memory:\n{} / {} total\nDetails: ({:?})", saturating_sub_bytes(swap.total, swap.free), swap.total, swap.platform_swap),
            Err(x) => format!("Error getting swap memory data: {}", x)
        },
        // CPU
        match self.sys.cpu_load_aggregate() {
            Ok(cpu)=> {
                thread::sleep(Duration::from_secs(1));
                let cpu = cpu.done().unwrap();
                format!("CPU Load:\n{}% user, {}% nice, {}% system, {}% intr, {}% idle ",
                    cpu.user * 100.0, cpu.nice * 100.0, cpu.system * 100.0, cpu.interrupt * 100.0, cpu.idle * 100.0)
            },
            Err(x) => format!("Error getting CPU load data {}", x)
        },
        // CPU Load (# of processes in the system run queue averaged over various timeframes)
        match self.sys.load_average() {
            Ok(loadavg) => format!("Load average (number of processes in the system run queue averaged over one, five, and fifteen minutes):\n{} {} {}", loadavg.one, loadavg.five, loadavg.fifteen),
            Err(x) => format!("Error getting load average data {}", x)
        },
        // CPU Temp (only the first temperature probe if there are multiple)
        match self.sys.cpu_temp() {
            Ok(cpu_temp) if cpu_temp > -100.0 => format!("CPU temp:\n{}", cpu_temp),
            Ok(cpu_temp) => match read_to_string("/sys/class/thermal/thermal_zone1/temp") {
                Ok(a) => {format!("CPU temp:\n{}", a.trim().parse::<f32>().expect("This file always only has numbers written in") / 1000.0)},
                Err(_) => {eprintln!("Error while getting CPU temp data: thermal_zone0 is reading < -100 and there is no other thermal zone."); format!("CPU temp:\n{}", cpu_temp)},
            },
            Err(x) => format!("Error getting CPU temp data: {}", x)
        },
        // Disk Space
        {
            let mut list = format!("Disk Data:\n");
            for disk in &self.disks {
                list = list + format!("Disk: {:?} usage: {:?} / {}\nENDDisk\n", disk.name(), (disk.total_space() - disk.available_space()), disk.total_space()).as_str();
            }
            list
        }
        
        );
        return data;
    }
}

