/// Представляет состояние криптовалютного рынка и управляет ходом симуляции.
pub struct Market {
    pub currencies: Vec<Currency>,
    pub agents: Vec<Agent>,
    pub transactions: Vec<Transaction>,

    pub tick: usize,
    pub spread: f64,

    pub next_currency_id: usize,
    pub next_agent_id: usize,
    pub next_transaction_id: usize,
}

impl Market {
    pub fn new(spread: f64) -> Self {
        Self {
            currencies: Vec::new(),
            agents: Vec::new(),
            transactions: Vec::new(),

            tick: 0,
            spread,

            next_currency_id: 1,
            next_agent_id: 1,
            next_transaction_id: 1,
        }
    }

    pub fn new_currency(&mut self, price: f64, fomo: f64) -> &Currency {
        let currency = Currency::new(
            self.next_currency_id,
            price,
            fomo,
        );

        self.currencies.push(currency);
        self.next_currency_id += 1;

        self.currencies.last().unwrap()
    }

    pub fn new_agent(&mut self, money: f64, behavior: Behavior) -> &Agent {
        let agent = Agent::new(
            self.next_agent_id,
            money,
            behavior,
        );

        self.agents.push(agent);
        self.next_agent_id += 1;

        self.agents.last().unwrap()
    }

    pub fn add_transaction(&mut self, mut transaction: Transaction) {
        transaction.id = self.next_transaction_id;
        self.transactions.push(transaction);
        self.next_transaction_id += 1;
    }
    
}