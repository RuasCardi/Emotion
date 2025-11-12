use eframe::egui;
use crate::mood::Mood;
use std::sync::{Arc, Mutex};

pub struct MoodApp {
    pub mood: Arc<Mutex<Mood>>,
}

impl eframe::App for MoodApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mood = self.mood.lock().unwrap().clone();
        
        // Define cor do emoji por humor
        let emoji_color = match mood {
            Mood::Calmo => egui::Color32::from_rgb(255, 215, 0),      // Amarelo
            Mood::Animado => egui::Color32::from_rgb(50, 205, 50),    // Verde
            Mood::Estressado => egui::Color32::from_rgb(220, 20, 60), // Vermelho
        };
        
        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                let emoji = match mood {
                    Mood::Calmo => "😌",
                    Mood::Animado => "😃",
                    Mood::Estressado => "😡",
                };
                
                // Desenha o emoji
                ui.centered_and_justified(|ui| {
                    let response = ui.label(egui::RichText::new(emoji)
                        .size(70.0)
                        .color(emoji_color));
                    
                    // Detecta arrasto e move a janela
                    if response.dragged() {
                        if let Some(pos) = ctx.input(|i| i.pointer.interact_pos()) {
                            ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                        }
                    }
                });
            });
        
        ctx.request_repaint();
    }
}
