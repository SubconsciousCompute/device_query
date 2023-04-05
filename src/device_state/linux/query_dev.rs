//! Read the keyboard and mice events from /dev/input folder.

#[derive(Debug, Clone)]
/// Device state descriptor.
pub struct DeviceState {
    enabled: bool,
}

impl DeviceState {
    pub fn new() -> Self {
        Self { enabled: false }
    }
}

/// List all input devices.
pub fn list_input_devices() -> Vec<(String, String)> {
    let mut result = vec![];
    if let Ok(content) = std::fs::read_to_string("/proc/bus/input/devices") {
        let devices = content.split("\n\n");
        for device in devices {
            let mut name = "";
            let mut handlers = "";
            for line in device.split('\n') {
                if line.starts_with('N') {
                    name = line
                        .strip_prefix("N: Name=")
                        .unwrap_or("")
                        .trim_matches('"');
                } else if line.starts_with("H:") {
                    handlers = line.strip_prefix("H: Handlers=").unwrap_or("").trim();
                    break;
                }
            }
            if ! name.is_empty() {
                result.push((name.to_string(), handlers.to_string()));
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_devices() {
        let devs = list_input_devices();
        println!("{:?}", devs);
    }
}
