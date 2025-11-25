fn main() {
    let username: String = std::env::var("USER").expect("Unknown");
    let hostname: String = std::fs::read_to_string("/proc/sys/kernel/hostname").expect("Unknown");
    println!("{}@{}", username, hostname.trim());

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
    println!("{}", os);

    let version_content: String = std::fs::read_to_string("/proc/version").expect("Unknown");
    let mut version_parts = version_content.split_whitespace();
    let version: &str = version_parts.nth(2).unwrap_or("Unknown");
    println!("Linux {}", version);

    let shell: String = std::env::var("SHELL").unwrap_or("Unknown".to_string());
    println!("{}", shell);

    let terminal: String = std::env::var("TERM_PROGRAM").unwrap_or("Unknown".to_string());
    println!("{}", terminal);

    let desktop: String = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or("Unknown".to_string());
    println!("{}", desktop);

    let (days, hours, minutes, seconds) = find_uptime();

    println!("{} Day(s) {} Hour(s) {} Minute(s) {} Second(s)", days, hours, minutes, seconds);
}

fn find_uptime() -> (String, String, String, String) {
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
