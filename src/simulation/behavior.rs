/// Описывает устойчивые поведенческие характеристики агента,
/// влияющие на оценку торговых решений.
pub struct Behavior {
    pub patience: f64,       // Склонность ждать и удерживать позицию
    pub impulsiveness: f64,  // Склонность быстро принимать решения
    pub greed: f64,         // Склонность отдавать предпочтение потенциальной прибыли
    pub fear: f64,           // Склонность избегать потенциальных убытков
    pub sociality: f64,      // Склонность ориентироваться на поведение других участников
}

impl Behavior {
    pub fn new(
        patience: f64,
        impulsiveness: f64,
        greed: f64,
        fear: f64,
        sociality: f64,
    ) -> Self {
        Self {
            patience,
            impulsiveness,
            greed,
            fear,
            sociality,
        }
    }
}