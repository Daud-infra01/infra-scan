use std::fs;
use std::thread;
use std::time::Duration;

fn main() {
     let cpu = get_cpu_usage();
println!("CPU Usage: {:.1}%", cpu);
    let mut processes: Vec<(u64, String, String)> = Vec::new();
let proc = fs::read_dir("/proc").unwrap();
    for entry in proc {
        let entry = entry.unwrap();
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        // Only process numeric directories (PIDs)
        if name_str.parse::<u32>().is_ok() {
            let status_path = format!("/proc/{}/status", name_str);
            if let Ok(contents) = fs::read_to_string(&status_path) {
                let proc_name = get_field(&contents, "Name");
                let vm_rss = get_field(&contents, "VmRSS");
                if let (Some(n), Some(r)) = (proc_name, vm_rss) {
if let Ok(ram_val) = r.split_whitespace().next().unwrap_or("0").parse::<u64>() {
    processes.push((ram_val, n, name_str.to_string()));
}          

  }
            }
}        }
processes.sort_by(|a, b| b.0.cmp(&a.0));
println!("\n=== TOP 5 MEMORY HOGS ===");
for (ram, name, pid) in processes.iter().take(5) {
    println!("  {:>20} | PID: {:>6} | RAM: {} kB", name, pid, ram);
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

fn get_cpu_usage() -> f64 {
    let (idle1, total1) = read_cpu_stat();
    thread::sleep(Duration::from_millis(500));
    let (idle2, total2) = read_cpu_stat();

    let idle_delta = idle2 - idle1;
    let total_delta = total2 - total1;

    (1.0 - idle_delta as f64 / total_delta as f64) * 100.0
}

