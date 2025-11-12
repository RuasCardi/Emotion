use crate::mood::Mood;

pub struct SystemEffects;

impl SystemEffects {
    pub fn apply(mood: &Mood) {
        match mood {
            Mood::Calmo => println!("Sistema em modo calmo: cores suaves, respostas lentas, som ambiente relaxante."),
            Mood::Animado => println!("Sistema animado: cores vibrantes, respostas rápidas, sons energéticos."),
            Mood::Estressado => println!("Sistema estressado: cores intensas, alertas, sons agudos, velocidade aumentada."),
        }
        // Aqui você pode integrar com APIs do sistema para alterar temas, sons, etc.
    }
}
