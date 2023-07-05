use std::collections::HashMap;
use super::orderbook::{OrderBook, Order};

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

    pub fn to_string(&self) -> String{
        format!("{}/{}", self.base, self.quote)
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
        self.orderbooks.insert(pair.clone(), OrderBook::new());
        println!("opening new orderbook for market {:?}", pair.to_string());
    }

    pub fn place_limit_order(&mut self, pair: TraidingPair, price: f64, order: Order)-> Result<(), String>{

        match self.orderbooks.get_mut(&pair){
            Some(orderbook) => {
                orderbook.add_order(price, order);

                println!("Placed limit order at price leve {}", price);
                Ok(())
            }
            None => {
                Err(format!("No orderbook for market {:?}", pair.to_string()))
            }
        }
    }
}

//13.40