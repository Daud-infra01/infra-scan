use std::fs;

fn main() {
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
                    println!("PID: {} | Name: {} | RAM: {}", name_str, n, r);
                }
            }
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
