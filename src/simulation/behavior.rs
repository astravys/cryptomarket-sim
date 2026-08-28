pub struct Behavior {
    pub patience: f64, // Склонность ждать и удерживать позицию
    pub impulsiveness: f64, // Насколько быстро агент принимает решение
    pub greed: f64, // Насколько сильно потенциальная прибыль повышает привлекательность действия
    pub fear: f64, // Насколько сильно потенциальный убыток снижает привлекательность действия
    pub sociality: f64, // Насколько сильно агент ориентируется на поведение других участников
}

impl Behavior {
    pub fn new(patience: f64, impulsiveness: f64, greed: f64, fear: f64, sociality: f64) {
        Behavior {
            patience,
            impulsiveness,
            greed,
            fear,
            sociality,
        }
    }
}