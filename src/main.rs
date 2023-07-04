use std::collections::HashMap;

#[derive(Debug)]
enum BidOrAsk{
    Bid,
    Ask,
}
#[derive(Debug)]
struct OrderBook{
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

impl OrderBook {
    fn new()-> OrderBook{
        OrderBook{
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    fn add_order(&mut self, price:f64, order: Order){
        match order.bid_or_ask{
            BidOrAsk::Bid => {
                let price = Price::new(price);
                
                match(self.bids.get_mut(&price)){
                    Some(limit) => {
                        limit.add_order(order);
                    },
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.bids.insert(price, limit);
                    }
                }
            }
            BidOrAsk::Ask => {

            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Price{
    intergral: u64,
    fractional: u64,
    scalar: u64,
}

impl Price{
    fn new(price: f64)->Price{
        let scalar = 100000;
        let intergral = price as u64;
        let fractional = ((price%1.0)*scalar as f64) as u64;
        Price{
            intergral,
            fractional,
            scalar,
        }
    }
}

#[derive(Debug)]
struct Limit{
    price: Price,
    orders: Vec<Order>,
}

impl Limit{
    fn new(price: Price) -> Limit{
        Limit{
            price,
            orders: Vec::new(),
        }
    }

    fn add_order(&mut self, order: Order){
        self.orders.push(order);
    }
}

#[derive(Debug)]
struct Order{
    size: f64,
    bid_or_ask: BidOrAsk,
}

impl Order{
    fn new(size: f64, bid_or_ask: BidOrAsk) -> Order{
        Order{
            size,
            bid_or_ask,
        }
    }
}


fn main() {

    let buy_order_from_alice = Order::new(5.5, BidOrAsk::Bid);
    //let sell_order = Order::new(3.5, BidOrAsk::Ask);
    let buy_order_from_bob = Order::new(2.45, BidOrAsk::Bid);
    let mut orderbook = OrderBook::new();


    orderbook.add_order(4.4, buy_order_from_alice);
    orderbook.add_order(4.4, buy_order_from_bob);

    println!("{:?}", orderbook);
}