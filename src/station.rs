use std::{fs::read_to_string, mem, thread, time::{Duration, SystemTime}};

use sysinfo::Disks;
use systemstat::{saturating_sub_bytes, Platform, System};

#[derive(bincode::Encode)]
struct DiskData {
    name: String,
    used: u64,
    max: u64
}

impl DiskData {
    fn new(name: String, used: u64, max: u64) -> Self {
        Self {
            name: name,
            used: used,
            max: max
        }
    }
}

#[derive(bincode::Encode)]
pub struct DataToSend {
    date: u128,
    uptime: u64,
    networks: String,
    socket_stats: String,
    memory_used: u64,
    memory_max: u64,
    memory_details: String,
    swap_used: u64,
    swap_max: u64,
    swap_details: String,
    cpu_user: f32,
    cpu_nice: f32,
    cpu_system: f32,
    cpu_intr: f32,
    cpu_idle: f32,
    load_onem: f32,
    load_fivem: f32,
    load_fifteenm: f32,
    cpu_temp: f32,
    disks: Vec<DiskData>
}

impl DataToSend {
    fn new(date: u128, uptime: u64, networks: String, sock_stats: String, mem_used: u64, mem_max: u64, mem_dets: String,
           swap_used: u64, swap_max: u64, swap_dets: String, cpu_data: (f32, f32, f32, f32, f32), 
           load_data: (f32, f32, f32), cpu_temp: f32, disks: Vec<DiskData>) -> DataToSend {

        DataToSend { date: date, uptime: uptime, networks: networks, socket_stats: sock_stats, memory_used: mem_used, 
        memory_max: mem_max, memory_details: mem_dets, swap_used: swap_used, swap_max: swap_max, 
        swap_details: swap_dets, cpu_user: cpu_data.0, cpu_nice: cpu_data.1, cpu_system: cpu_data.2, 
        cpu_intr: cpu_data.3, cpu_idle: cpu_data.4, load_onem: load_data.0, load_fivem: load_data.1, 
        load_fifteenm: load_data.2, cpu_temp: cpu_temp, disks: disks }
    }

    pub fn len(&self) -> usize {
        return mem::size_of_val(&self);
    }
}

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

    pub fn get_data(&mut self) -> DataToSend {
        let date = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("The system clock shows time before UNIX EPOCH!").as_nanos();

        let uptime = match self.sys.uptime() {
            Ok(uptime) => uptime,
            Err(x) => {eprintln!("Error getting the uptime: {}", x); Duration::from_secs(0)},
        }.as_secs();

        // Network
        let networks = match self.sys.networks() {
            Ok(netifs) => {
                let mut list: String = format!("");
                for netif in netifs.values() {
                    list = list + format!("Interface:\nName: ({})\nData: ({:?})\nStatistics: ({:?})\nENDInt\n", netif.name, netif.addrs, self.sys.network_stats(&netif.name)).as_str();
                }
                list.trim_end().to_string()
            }
            Err(x) => {eprintln!("Error getting network data: {}", x); format!("Error getting network data. Check station logs.")}
        };

        // Network Sockets
        let sock_stats = match self.sys.socket_stats() {
            Ok(stats) => format!("{:?}", stats),
            Err(x) => {eprintln!("Error getting system socket statistics: {}", x); format!("Error getting system socket statistics. Check station logs.")}
        };

        // Memory
        let mut mem_used = 0;
        let mut mem_max = 0;
        let mem_dets = match self.sys.memory() {
            Ok(mem) => {mem_used = saturating_sub_bytes(mem.total, mem.free).as_u64(); mem_max = mem.total.as_u64(); format!("Details: ({:?})", mem.platform_memory)},
            Err(x) => {eprintln!("Error getting memory data: {}", x); format!("Error getting memory data. Check station logs.")}
        };

        // Swap Memory
        let mut swap_used = 0;
        let mut swap_max = 0;
        let swap_dets = match self.sys.swap() {
            Ok(swap) => {swap_used = saturating_sub_bytes(swap.total, swap.free).as_u64(); swap_max = swap.total.as_u64(); format!("Details: ({:?})", swap.platform_swap)},
            Err(x) => {eprintln!("Error getting swap memory data: {}", x); format!("Error getting swap memory data. Check station logs.")}
        };

        // CPU
        let cpu_data = match self.sys.cpu_load_aggregate() {
            Ok(cpu) => {
                thread::sleep(Duration::from_secs(1));
                let cpu = cpu.done().unwrap();
                (cpu.user * 100.0, cpu.nice * 100.0, cpu.system * 100.0, cpu.interrupt * 100.0, cpu.idle * 100.0)
            },
            Err(x) => {eprintln!("Error getting cpu data: {}", x); (f32::MIN, f32::MIN, f32::MIN, f32::MIN, f32::MIN)}
        };

        // CPU Load (# of processes in the system run queue averaged over various timeframes)
        let load_data = match self.sys.load_average() {
            Ok(loadavg) => (loadavg.one, loadavg.five, loadavg.fifteen),
            Err(x) => {eprintln!("Error getting load data: {}", x); (f32::MIN, f32::MIN, f32::MIN)}
        };

        // CPU Temp (only the first temperature probe if there are multiple)
        let cpu_temp = match self.sys.cpu_temp() {
            Ok(cpu_temp) if cpu_temp > -100.0 => cpu_temp,
            Ok(cpu_temp) => match read_to_string("/sys/class/thermal/thermal_zone1/temp") {
                Ok(a) => a.trim().parse::<f32>().expect("This file always only has numbers written in") / 1000.0,
                Err(_) => {eprintln!("Error while getting CPU temp data: thermal_zone0 is reading < -100 and there is no other thermal zone."); cpu_temp},
            },
            Err(x) => {eprintln!("Error getting temperature data: {}", x); f32::MIN}
        };

        // Disk Space
        let disks =  {
            let mut list = Vec::new();
            for disk in &self.disks {
                list.push(DiskData::new(disk.name().to_str().unwrap().to_string(), disk.total_space() - disk.available_space(), disk.total_space()));
            }
            list
        };

        let data = DataToSend::new(date, uptime, networks, sock_stats, mem_used, mem_max, mem_dets, swap_used, swap_max, swap_dets, cpu_data, load_data, cpu_temp, disks);
        return data;
    }
}

