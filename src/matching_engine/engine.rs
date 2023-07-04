use std::collections::HashMap;
use super::orderbook::OrderBook;

//BTCUSD
//BTC => Base
//USD => Quote

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TraidingPair{
    base: String,
    quote: String,
}

impl TraidingPair{
    pub fn new(base: String, quote: String) -> TraidingPair{
        TraidingPair{
            base,
            quote,
        }
    }
}

pub struct MathchingEngine {
    orderbooks: HashMap<TraidingPair, OrderBook>,
}

impl MathchingEngine{
    pub fn new() -> MathchingEngine{
        MathchingEngine{
            orderbooks: HashMap::new(),
        }
       }
    
    pub fn add_new_market(&mut self, pair: TraidingPair){
        self.orderbooks.insert(pair, OrderBook::new());
    }
}