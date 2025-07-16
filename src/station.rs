use std::{thread, time::Duration};

use systemstat::{saturating_sub_bytes, Platform, System};

pub struct Station {
    sys: System,
    last_check: usize,
}

impl Station {
    pub fn new() -> Station {
        Station { sys: System::new(), last_check: 0 }
    }

    pub fn get_data(&mut self) {
        match self.sys.networks() {
            Ok(netifs) => {
                println!("\nNetworks:");
                for netif in netifs.values() {
                    println!("{} ({:?})", netif.name, netif.addrs);
                }
            }
            Err(x) => println!("\nNetworks: error: {}", x)
        }

        match self.sys.memory() {
            Ok(mem) => println!("\nMemory: {} used / {} ({} bytes) total ({:?})", saturating_sub_bytes(mem.total, mem.free), mem.total, mem.total.as_u64(), mem.platform_memory),
            Err(x) => println!("\nMemory: error: {}", x)
        }

        match self.sys.swap() {
            Ok(swap) => println!("\nSwap: {} used / {} ({} bytes) total ({:?})", saturating_sub_bytes(swap.total, swap.free), swap.total, swap.total.as_u64(), swap.platform_swap),
            Err(x) => println!("\nSwap: error: {}", x)
        }

        match self.sys.load_average() {
            Ok(loadavg) => println!("\nLoad average: {} {} {}", loadavg.one, loadavg.five, loadavg.fifteen),
            Err(x) => println!("\nLoad average: error: {}", x)
        }

        match self.sys.uptime() {
            Ok(uptime) => println!("\nUptime: {:?}", uptime),
            Err(x) => println!("\nUptime: error: {}", x)
        }

        match self.sys.cpu_load_aggregate() {
            Ok(cpu)=> {
                println!("\nMeasuring CPU load...");
                thread::sleep(Duration::from_secs(1));
                let cpu = cpu.done().unwrap();
                println!("CPU load: {}% user, {}% nice, {}% system, {}% intr, {}% idle ",
                    cpu.user * 100.0, cpu.nice * 100.0, cpu.system * 100.0, cpu.interrupt * 100.0, cpu.idle * 100.0);
            },
            Err(x) => println!("\nCPU load: error: {}", x)
        }

        match self.sys.cpu_temp() {
            Ok(cpu_temp) => println!("\nCPU temp: {}", cpu_temp),
            Err(x) => println!("\nCPU temp: {}", x)
        }

        match self.sys.socket_stats() {
            Ok(stats) => println!("\nSystem socket statistics: {:?}", stats),
            Err(x) => println!("\nSystem socket statistics: error: {}", x)
        }
    }
}

