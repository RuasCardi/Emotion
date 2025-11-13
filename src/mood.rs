#[derive(Debug, Clone)]
pub enum Mood {
    Calmo,
    Animado,
    Estressado,
}

impl Mood {
    pub fn from_signals(_temp: f32, sound: f32, key: f32, click: f32) -> Self {
        // key = número de teclas pressionadas
        // click = número de botões de mouse pressionados
        // sound = nível de som ambiente (0.0 a 1.0+)

        if sound <= 0.080 {
            // Som baixo = calmo (amarelo)
            Mood::Calmo
        } else if sound > 0.080 && sound <= 0.110 {
            // Som moderado = animado (verde)
            Mood::Animado
        } else if sound > 0.110 {
            // Som alto = estressado (vermelho)
            Mood::Estressado
        } else if key > 2.0 || click > 1.0 {
            // Muitas teclas ou cliques = estressado
            Mood::Estressado
        } else if key > 0.0 || click > 0.0 {
            // Atividade de teclado/mouse sem som alto = animado
            Mood::Animado
        } else {
            Mood::Calmo
        }
    }
}
