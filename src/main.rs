mod sensors;
mod mood;
mod system_effects;
mod gui;
mod typing_data;
mod typing_ai;

use eframe::egui;
use sensors::{TemperatureSensor, SoundSensor, KeyboardMonitor, MouseMonitor};
use mood::Mood;

fn main() {
    let temp_sensor = TemperatureSensor::new();
    let sound_sensor = SoundSensor::new();
    let keyboard_monitor = KeyboardMonitor::new();
    let mouse_monitor = MouseMonitor::new();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([100.0, 100.0])
            .with_position([10.0, 10.0])
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(true)
            .with_taskbar(false),
        ..Default::default()
    };
    let mood_state = std::sync::Arc::new(std::sync::Mutex::new(Mood::Calmo));
    let mood_state_bg = mood_state.clone();

    let mut typing_logger = typing_data::TypingLogger::new();
    let mut typing_ai = typing_ai::TypingAI::new();
    std::thread::spawn(move || {
        loop {
            let temperature = temp_sensor.read();
            let sound_level = sound_sensor.read();
            let key_pressure = keyboard_monitor.read();
            let click_interval = mouse_monitor.read();
            typing_logger.log();
            let ai_mood = typing_ai.analyze();
            println!("Temp: {:.1}°C | Som: {:.3} | Teclas: {} | Mouse: {} | IA: {}", 
                     temperature, sound_level, key_pressure, click_interval, ai_mood);
            let mood = Mood::from_signals(temperature, sound_level, key_pressure, click_interval);
            let mut mood_lock = mood_state_bg.lock().unwrap();
            *mood_lock = mood;
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    });

    let mood_state_gui = mood_state.clone();
    eframe::run_native(
        "Signal Skin",
        native_options,
        Box::new(move |_cc| {
            Box::new(gui::MoodApp {
                mood: mood_state_gui.clone(),
            })
        })
    );
}
