fn main() {
    let username: String = std::env::var("USER").expect("Unknown");
    let hostname: String = std::fs::read_to_string("/proc/sys/kernel/hostname").expect("Unknown").trim().to_string();

    let os = get_os();

    let version = get_version();

    let shell: String = std::env::var("SHELL").unwrap_or("Unknown".to_string());

    let terminal: String = std::env::var("TERM_PROGRAM").unwrap_or("Unknown".to_string());

    let desktop: String = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or("Unknown".to_string());

    let (days, hours, minutes, seconds) = get_uptime();

    let cpu = get_cpu();

    let (total_mem, total_used, percent_used) = get_memory();

    let (used_bytes, total_bytes, total_bytes_percent) = get_storage();


    println!("
⣇⣿⠘⣿⣿⣿⡿⡿⣟⣟⢟⢟⢝⠵⡝⣿⡿⢂⣼⣿⣷⣌⠩⡫⡻⣝⠹⢿⣿⣷  | {username}@{hostname}
⡆⣿⣆⠱⣝⡵⣝⢅⠙⣿⢕⢕⢕⢕⢝⣥⢒⠅⣿⣿⣿⡿⣳⣌⠪⡪⣡⢑⢝⣇  | 
⡆⣿⣿⣦⠹⣳⣳⣕⢅⠈⢗⢕⢕⢕⢕⢕⢈⢆⠟⠋⠉⠁⠉⠉⠁⠈⠼⢐⢕⢽  | {os}
⡗⢰⣶⣶⣦⣝⢝⢕⢕⠅⡆⢕⢕⢕⢕⢕⣴⠏⣠⡶⠛⡉⡉⡛⢶⣦⡀⠐⣕⢕  | Linux {version}
⡝⡄⢻⢟⣿⣿⣷⣕⣕⣅⣿⣔⣕⣵⣵⣿⣿⢠⣿⢠⣮⡈⣌⠨⠅⠹⣷⡀⢱⢕  | {shell}
⡝⡵⠟⠈⢀⣀⣀⡀⠉⢿⣿⣿⣿⣿⣿⣿⣿⣼⣿⢈⡋⠴⢿⡟⣡⡇⣿⡇⡀⢕  | {terminal}
⡝⠁⣠⣾⠟⡉⡉⡉⠻⣦⣻⣿⣿⣿⣿⣿⣿⣿⣿⣧⠸⣿⣦⣥⣿⡇⡿⣰⢗⢄  | {desktop}
⠁⢰⣿⡏⣴⣌⠈⣌⠡⠈⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣬⣉⣉⣁⣄⢖⢕⢕⢕  | {days:} Day(s) {hours} Hour(s) {minutes} Minute(s) {seconds:.2} Second(s)
⡀⢻⣿⡇⢙⠁⠴⢿⡟⣡⡆⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣵⣵⣿  | {cpu}
⡻⣄⣻⣿⣌⠘⢿⣷⣥⣿⠇⣿⣿⣿⣿⣿⣿⠛⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿  | {total_used:.5} GiB / {total_mem:.5} GiB ({percent_used:.5}%)
⣷⢄⠻⣿⣟⠿⠦⠍⠉⣡⣾⣿⣿⣿⣿⣿⣿⢸⣿⣦⠙⣿⣿⣿⣿⣿⣿⣿⣿⠟  | {used_bytes:.4} TiB / {total_bytes:.4} TiB ({total_bytes_percent:.5}%)
⡕⡑⣑⣈⣻⢗⢟⢞⢝⣻⣿⣿⣿⣿⣿⣿⣿⠸⣿⠿⠃⣿⣿⣿⣿⣿⣿⡿⠁⣠  |
⡝⡵⡈⢟⢕⢕⢕⢕⣵⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣶⣿⣿⣿⣿⣿⠿⠋⣀⣈⠙  |
⡝⡵⡕⡀⠑⠳⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⠛⢉⡠⡲⡫⡪⡪⡣  |
");
}

fn get_version() -> String {
    let version_content: String = std::fs::read_to_string("/proc/version").expect("Unknown");
    let mut version_parts = version_content.split_whitespace();
    let version = version_parts.nth(2).unwrap_or("Unknown").to_string();

    version
}

fn get_os() -> String {
    let os_release_content: String = std::fs::read_to_string("/etc/os-release").expect("Unknown");
    let mut os= "Unknown".to_string();
    for line in os_release_content.lines() {
        if line.starts_with("PRETTY_NAME=") {
          if let Some((_, value)) = line.split_once('=') {
              let trimmed_value: &str = value.trim_matches('"');
              os = trimmed_value.to_string();
              break;
          }
        }
    }
    os
}

fn get_uptime() -> (String, String, String, String) {
    let uptime_content: String = std::fs::read_to_string("/proc/uptime").unwrap_or("Unknown".to_string());
    let mut uptime_parts = uptime_content.split_whitespace();
    let uptime_string_seconds = uptime_parts.next().unwrap_or("0");

    let mut uptime_seconds: f32 = uptime_string_seconds.parse::<f32>().unwrap_or(0.0);

    const DAY_SECONDS: f32 = 24.0 * 60.0 * 60.0;
    let days: f32 = (uptime_seconds / DAY_SECONDS).floor();
    uptime_seconds = uptime_seconds - (days * DAY_SECONDS);

    const HOUR_SECONDS: f32 = 60.0 * 60.0;
    let hours: f32 = (uptime_seconds / HOUR_SECONDS).floor();
    uptime_seconds = uptime_seconds - (hours * HOUR_SECONDS);

    const MINUTE_SECONDS: f32 = 60.0;
    let minutes: f32 = (uptime_seconds / MINUTE_SECONDS).floor();
    uptime_seconds = uptime_seconds - (minutes * MINUTE_SECONDS);

    let seconds: f32 = uptime_seconds;

    return (days.to_string(), hours.to_string(), minutes.to_string(), seconds.to_string());
}

fn get_cpu() -> String {
    let cpuinfo_content = std::fs::read_to_string("/proc/cpuinfo").unwrap_or("Unknown".to_string());
    let mut cpu = "Unknown".to_string();

    for line in cpuinfo_content.lines() {
        if line.starts_with("model name") {
            if let Some((_, value)) = line.split_once(':') {
              cpu = value.trim().to_string();
            }
        }
    }

    cpu
}

fn get_memory() -> (String, String, String) {
    let mut total_mem: f32 = 0.0;
    let mut available_mem: f32 = 0.0;

    let meminfo_content = std::fs::read_to_string("/proc/meminfo").unwrap_or("Unknown".to_string());
    for line in meminfo_content.lines(){
        if line.starts_with("MemTotal") {
            if let Some((_, value)) = line.split_once(':') {
                let trimmed_value = value.trim();

                let mut value_parts = trimmed_value.split_whitespace();
                total_mem = value_parts.next().unwrap_or("0.0").parse::<f32>().unwrap_or(0.0);
            }
        }

        if line.starts_with("MemAvailable") {
            if let Some((_, value)) = line.split_once(':') {
                let trimmed_value = value.trim();

                let mut value_parts = trimmed_value.split_whitespace();
                available_mem = value_parts.next().unwrap_or("0.0").parse::<f32>().unwrap_or(0.0);
            }
        }
    }

    let gigabyte_kilabytes: f32 = 1024.0 * 1024.0;
    total_mem = total_mem / gigabyte_kilabytes;
    let total_used: f32 = total_mem - (available_mem / gigabyte_kilabytes);

    let percent_used: f32 = (total_used / total_mem) * 100.0;

    return (total_mem.to_string(), total_used.to_string(), percent_used.to_string());
}

fn get_storage() -> (String, String, String) {
    const BYTES: f64 = 1024.0;

    if let Ok(stats) = nix::sys::statvfs::statvfs("/") {
        let mut total_bytes: f64 = (stats.blocks() as f64 ) * (stats.block_size() as f64);
        let free_bytes: f64 = (stats.blocks_free() as f64 ) * (stats.block_size() as f64);
        let mut used_bytes: f64 = total_bytes - free_bytes;

        total_bytes = total_bytes / BYTES.powf(4.0);
        used_bytes = used_bytes / BYTES.powf(4.0);

        let used_bytes_percent = (used_bytes as f64 / total_bytes as f64) * 100.0;

        return (used_bytes.to_string(), total_bytes.to_string(), used_bytes_percent.to_string());
    } else {
        return ("0".to_string(), "0".to_string(), "0".to_string());
    }
}
