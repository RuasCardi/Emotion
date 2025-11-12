#[derive(Debug, Clone)]
pub enum Mood {
    Calmo,
    Animado,
    Estressado,
}

impl Mood {
    pub fn from_signals(_temp: f32, sound: f32, key: f32, click: f32) -> Self {
        // Lógica refinada para detectar humor
        // key = número de teclas pressionadas
        // click = número de botões de mouse pressionados
        // sound = nível de som ambiente (0.0 a 1.0+)
        
        if key > 2.0 || click > 1.0 || sound > 0.501 {
            // Muitas teclas, cliques ou som muito alto = estressado (vermelho)
            Mood::Estressado
        } else if sound >= 0.130 && sound <= 0.400 {
            // Som moderado = animado (verde)
            Mood::Animado
        } else if key > 0.0 || click > 0.0 {
            // Atividade de teclado/mouse sem som alto = animado (verde)
            Mood::Animado
        } else {
            // Som ambiente baixo ou silêncio = calmo (amarelo)
            Mood::Calmo
        }
    }
}
