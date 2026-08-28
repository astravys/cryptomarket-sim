use crate::simulation::behavior::Behavior;
use crate::simulation::currency::Currency;
use crate::simulation::transaction::Transaction;

/// Представляет позицию агента в конкретной валюте.
pub struct Holding {
    pub currency_id: usize,
    pub purchase_price: f64,
    pub quantity: f64,
}

impl Holding {
    pub fn new(
        currency_id: usize,
        purchase_price: f64,
        quantity: f64,
    ) -> Self {
        Self {
            currency_id,
            purchase_price,
            quantity,
        }
    }
}


/// Представляет участника рынка, принимающего торговые решения
/// на основании своего состояния и поведенческих характеристик.
pub struct Agent {
    pub id: usize,
    pub money: f64,
    pub portfolio: Vec<Holding>,
    pub trade_history: Vec<Transaction>,
    pub behavior: Behavior,
}

impl Agent {
    pub fn new(
        id: usize,
        money: f64,
        behavior: Behavior,
    ) -> Self {
        Self {
            id,
            money,
            portfolio: Vec::new(),
            trade_history: Vec::new(),
            behavior,
        }
    }
}

