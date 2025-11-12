use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct TypingAI {
    pub last_mood: String,
}

impl TypingAI {
    pub fn new() -> Self {
        TypingAI { last_mood: "Calmo".to_string() }
    }

    pub fn analyze(&mut self) -> String {
        let file = match File::open("typing_log.csv") {
            Ok(f) => f,
            Err(_) => return self.last_mood.clone(),
        };
        let reader = BufReader::new(file);
        let mut intervals = Vec::new();
        let mut key_counts = Vec::new();
        for line in reader.lines().flatten().rev().take(30) {
            let parts: Vec<&str> = line.split(';').collect();
            if parts.len() >= 3 {
                if let Ok(interval) = parts[1].parse::<u128>() {
                    intervals.push(interval);
                }
                if let Ok(count) = parts[2].parse::<usize>() {
                    key_counts.push(count);
                }
            }
        }
        let avg_interval = if intervals.len() > 0 {
            intervals.iter().sum::<u128>() as f32 / intervals.len() as f32
        } else { 0.0 };
        let avg_keys = if key_counts.len() > 0 {
            key_counts.iter().sum::<usize>() as f32 / key_counts.len() as f32
        } else { 0.0 };
        // Lógica simples: digitação rápida e muitas teclas = estressado
        let mood = if avg_interval < 120.0 && avg_keys > 2.0 {
            "Estressado"
        } else if avg_interval < 300.0 {
            "Animado"
        } else {
            "Calmo"
        };
        self.last_mood = mood.to_string();
        mood.to_string()
    }
}
