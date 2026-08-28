use crate::simulation::price_point::PricePoint;

/// Представляет криптовалюту, доступную для торговли на рынке.
pub struct Currency {
    pub id: usize,
    pub price: f64, // Текущая рыночная цена
    pub fomo: f64, // Постоянный информационный фактор валюты в диапазоне [-1; 1]
    pub volume: f64, // Текущий объем торгов валютой за последние n тиков
    pub holders: usize, // Количество агентов, владеющих валютой
    pub price_history: Vec<PricePoint>, // История изменения цены валюты
}

impl Currency {
    pub fn new(
        id: usize,
        price: f64,
        fomo: f64,
    ) -> Self {
        Self {
            id,
            price,
            fomo,
            volume: 0.0,
            holders: 0,
            price_history: Vec::new(),
        }
    }

    pub fn update_price(&mut self, price: f64, tick: usize) {
        self.price = price;
        self.price_history.push(PricePoint::new(tick, price))
    }

    pub fn update_volume(&mut self, volume: f64) {
        self.volume = volume;
    }

    pub fn add_holder(&mut self) {
        self.holders += 1;
    }

    pub fn remove_holder(&mut self) {
        self.holders = self.holders.saturating_sub(1);
    }

    /// При коллапсе валюты fomo меняет знак
    pub fn trigger_fomo_collapse(&mut self) {
        let current_fomo = self.fomo;
        self.fomo = -current.fomo;
        todo!();
        
    }
}