/// Определяет направление торговой операции.
pub enum TransactionSide {
    buy,
    sell,
}

/// Представляет совершённую агентом операцию покупки или продажи валюты.
pub struct Transaction {
    pub id: usize,
    pub tick: usize,
    pub agent_id: usize,
    pub currency_id: usize,
    pub side: TransactionSide,
    pub price: f64,
    pub quantity: f64,
}