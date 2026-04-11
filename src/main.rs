use std::fs;
use std::thread;
use std::time::Duration;
struct Process {
    pid: u32,
    name: String,
    ram_kb: u64,
    state: String,
}

fn main() {
    let cpu = get_cpu_usage();
    println!("==============================");
    println!("      INFRA-SCAN v0.1         ");
    println!("==============================");
    println!("CPU: {:.1}%\n", cpu);
    println!("Disk  | {}", get_disk_stats());
    println!("Net   | {}", get_net_stats());

    let mut processes: Vec<Process> = fs::read_dir("/proc")
        .unwrap()
        .filter_map(|e| {
            let e = e.ok()?;
            let pid = e.file_name().to_string_lossy().parse::<u32>().ok()?;
            parse_process(pid)
        })
        .collect();

    processes.sort_by(|a, b| b.ram_kb.cmp(&a.ram_kb));

    println!("=== TOP 5 MEMORY HOGS ===");
    for p in processes.iter().take(5) {
        println!("  {:>20} | PID: {:>6} | RAM: {} kB", p.name, p.pid, p.ram_kb);
    }

    println!("\n=== ZOMBIES ===");
    let zombies: Vec<&Process> = processes.iter()
        .filter(|p| p.state.starts_with('Z'))
        .collect();
    if zombies.is_empty() {
        println!("  None.");
    } else {
        for p in zombies {
            println!("  ZOMBIE: {} (PID: {})", p.name, p.pid);
        }
    }
}
fn get_field(contents: &str, field: &str) -> Option<String> {
    for line in contents.lines() {
        if line.starts_with(field) {
            return Some(line.split_whitespace().skip(1).collect::<Vec<_>>().join(" "));
        }
    }


    None

}

fn parse_process(pid: u32) -> Option<Process> {
    let status_path = format!("/proc/{}/status", pid);
    let contents = fs::read_to_string(status_path).ok()?;
    
    let name = get_field(&contents, "Name")?;
    let state = get_field(&contents, "State")?;
    let ram_str = get_field(&contents, "VmRSS")?;
    let ram_kb = ram_str.split_whitespace()
        .next()?
        .parse::<u64>().ok()?;

    Some(Process { pid, name, ram_kb, state })
}
fn read_cpu_stat() -> (u64, u64) {
    let contents = fs::read_to_string("/proc/stat").unwrap();
    let first_line = contents.lines().next().unwrap();
    let nums: Vec<u64> = first_line
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    let idle = nums[3] + nums[4]; // idle + iowait
    let total: u64 = nums.iter().sum();
    (idle, total)
}
fn get_disk_stats() -> String {
    let contents = fs::read_to_string("/proc/diskstats").unwrap_or_default();
    for line in contents.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() > 9 && fields[2] == "nvme0n1" {
            let reads: u64 = fields[3].parse().unwrap_or(0);
            let writes: u64 = fields[7].parse().unwrap_or(0);
            return format!("Reads: {}  Writes: {}", reads, writes);
        }
    }
    "No disk data".to_string()
}

fn get_net_stats() -> String {
    let contents = fs::read_to_string("/proc/net/dev").unwrap_or_default();
    for line in contents.lines() {
        let line = line.trim();
        if line.starts_with("wlp3s0") {
            let fields: Vec<&str> = line.split_whitespace().collect();
            let rx_bytes: u64 = fields[1].parse().unwrap_or(0);
            let tx_bytes: u64 = fields[9].parse().unwrap_or(0);
            return format!("RX: {} KB  TX: {} KB", rx_bytes / 1024, tx_bytes / 1024);
        }
    }
    "No network data".to_string()
}
fn get_cpu_usage() -> f64 {
    let (idle1, total1) = read_cpu_stat();
    thread::sleep(Duration::from_millis(500));
    let (idle2, total2) = read_cpu_stat();

    let idle_delta = idle2 - idle1;
    let total_delta = total2 - total1;

    (1.0 - idle_delta as f64 / total_delta as f64) * 100.0
}

