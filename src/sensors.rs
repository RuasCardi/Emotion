pub struct TemperatureSensor;
pub struct SoundSensor;
pub struct KeyboardMonitor;
pub struct MouseMonitor;

impl TemperatureSensor {
    pub fn new() -> Self { TemperatureSensor }
    pub fn read(&self) -> f32 {
        // Leitura real da temperatura no Linux
        match std::fs::read_to_string("/sys/class/thermal/thermal_zone0/temp") {
            Ok(content) => {
                if let Ok(raw) = content.trim().parse::<f32>() {
                    // O valor está em milicelsius
                    raw / 1000.0
                } else {
                    30.0 // fallback
                }
            }
            Err(_) => 30.0 // fallback
        }
    }
}

impl SoundSensor {
    pub fn new() -> Self { SoundSensor }
    pub fn read(&self) -> f32 {
        // Captura de som ambiente usando cpal
        use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
        use std::sync::{Arc, Mutex};
        
        let host = cpal::default_host();
        let device = match host.default_input_device() {
            Some(d) => d,
            None => return 0.0,
        };
        let config = match device.default_input_config() {
            Ok(c) => c,
            Err(_) => return 0.0,
        };
        
        let volume = Arc::new(Mutex::new(0.0f32));
        let samples = Arc::new(Mutex::new(0usize));
        let volume_clone = volume.clone();
        let samples_clone = samples.clone();
        
        let stream = device.build_input_stream(
            &config.into(),
            move |data: &[f32], _| {
                let mut vol = volume_clone.lock().unwrap();
                let mut samp = samples_clone.lock().unwrap();
                for &sample in data {
                    *vol += sample.abs();
                    *samp += 1;
                }
            },
            |_| {},
            None
        );
        
        if let Ok(s) = stream {
            let _ = s.play();
            std::thread::sleep(std::time::Duration::from_millis(100));
            drop(s);
        }
        
        let vol = *volume.lock().unwrap();
        let samp = *samples.lock().unwrap();
        
        if samp > 0 {
            vol / samp as f32
        } else {
            0.0
        }
    }
}

impl KeyboardMonitor {
    pub fn new() -> Self { KeyboardMonitor }
    pub fn read(&self) -> f32 {
        use device_query::{DeviceQuery, DeviceState};
        let device_state = DeviceState::new();
        let keys = device_state.get_keys();
        // Simulação: retorna quantidade de teclas pressionadas
        keys.len() as f32
    }
}

impl MouseMonitor {
    pub fn new() -> Self { MouseMonitor }
    pub fn read(&self) -> f32 {
        use device_query::{DeviceQuery, DeviceState};
        let device_state = DeviceState::new();
        let mouse = device_state.get_mouse();
        // Retorna quantidade de botões pressionados
        let pressed = mouse.button_pressed.iter().filter(|&&b| b).count();
        pressed as f32
    }
}
