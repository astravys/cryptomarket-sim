/// Представляет состояние цены валюты в определённый момент симуляции.
pub struct PricePoint {
    pub tick: usize,
    pub price: f64,
}

impl PricePoint {
    pub fn new(tick: usize, price: f64) -> Self {
        Self {
            tick,
            price,
        }
    }
}