pub enum TransactionSide {
    Buy,
    Sell,
}

pub struct Transaction {
    pub id: usize,

    pub tick: usize,
    pub agent_id: usize,
    pub currency_id: usize,

    pub side: TransactionSide,
    pub price: f64,
    pub quantity: f64,
}