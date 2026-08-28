pub struct Currency {
    pub id: usize,

    pub price: f64,
    
    pub fomo: f64,

    pub volume: f64,

    pub price_history: Vec<PricePoint>,
}

impl Currency {
    pub fn new(id: usize, price: f64, fomo: f64) -> Self {
        Currency {
            id,
            price,
            fomo,
            volume: 0.0,
            price_history: Vec::new(),
        }
    }
}