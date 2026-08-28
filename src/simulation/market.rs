pub struct Market {
    pub currencies: Vec<Currency>,
    pub agents: Vec<Agent>,
    pub transactions: Vec<Transaction>,

    pub tick: usize,

    pub price_spread: f64,
    
    pub next_currency_id: usize,
    pub next_agent_id: usize,
    pub next_transaction_id: usize,
}

impl Market {
    pub fn new_agent(&mut self, money: f64, risk_tolerance: f64, holding_tendency: f64, reaction_speed: f64) -> &Agent {
        let agent = Agent::new(self.next_agent_id, money, risk_tolerance, holding_tendency, reaction_speed);
        self.agents.push(agent);
        self.next_agent_id += 1;
        self.agents.last().unwrap()
    }

    pub fn new_currency(&mut self, price: f64, fomo: f64) -> &Currency {
        let currency = Currency::new(self.next_currency_id, price, fomo);
        self.currencies.push(currency);
        self.next_currency_id += 1;
        self.currencies.last().unwrap()
    }

    pub fn new_transaction(&mut self) {
        todo!()
    }
    
}