use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct BatteryInfo {
    pub percentage: Option<u8>,
    pub status: Option<String>,
}

fn read_sysfs(path: &str) -> Option<String> {
    fs::read_to_string(Path::new(path))
        .ok()
        .map(|s| s.trim().to_string())
}

fn find_battery_device() -> Option<String> {
    let power_supply = Path::new("/sys/class/power_supply");
    if !power_supply.is_dir() {
        return None;
    }
    for entry in fs::read_dir(power_supply).ok()? {
        let entry = entry.ok()?;
        let name = entry.file_name().to_string_lossy().to_string();
        let uevent_path = entry.path().join("uevent");
        if uevent_path.exists()
            && let Ok(content) = fs::read_to_string(&uevent_path)
            && content.contains("POWER_SUPPLY_TYPE=Battery")
        {
            return Some(entry.path().to_string_lossy().to_string());
        }
        if name.starts_with("BAT") || name.starts_with("battery") {
            return Some(entry.path().to_string_lossy().to_string());
        }
    }
    None
}

pub fn get_battery_info() -> BatteryInfo {
    let device_path = match find_battery_device() {
        Some(p) => p,
        None => return BatteryInfo::default(),
    };

    let capacity =
        read_sysfs(&format!("{}/capacity", device_path)).and_then(|s| s.parse::<u8>().ok());

    let status = read_sysfs(&format!("{}/status", device_path));

    BatteryInfo {
        percentage: capacity,
        status,
    }
}
