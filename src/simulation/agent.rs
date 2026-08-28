use crate::simulation::behavior::Behavior;

pub struct Agent {
    pub id: usize,

    pub money: f64,
    pub portfolio: Vec<Currency>,

    pub trade_history: Vec<Transaction>,

    pub behavior: Behavior,
}

impl Agent {
    pub fn new(id: usize, money: f64, risk: f64, patience: f64, impulsiveness: f64, greed: f64, fear: f64, sociality: f64) -> Self {
        Agent {
            id,
            money,
            portfolio: Vec::new(),
            trade_history: Vec::new(),
            bevahior: Behavior::new(risk, patience, impulsiveness, greed, fear, sociality)
        }
    }
}