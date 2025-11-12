use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use device_query::{DeviceQuery, DeviceState, Keycode};

pub struct TypingLogger {
    last_time: Option<u128>,
}

impl TypingLogger {
    pub fn new() -> Self {
        TypingLogger { last_time: None }
    }

    pub fn log(&mut self) {
        let device_state = DeviceState::new();
        let keys = device_state.get_keys();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let interval = if let Some(last) = self.last_time {
            now - last
        } else {
            0
        };
        self.last_time = Some(now);
        let key_count = keys.len();
        let key_names: Vec<String> = keys.iter().map(|k| format!("{:?}", k)).collect();
        let line = format!("{};{};{};{}\n", now, interval, key_count, key_names.join(","));
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("typing_log.csv")
            .unwrap();
        let _ = file.write_all(line.as_bytes());
    }
}
