mod matching_engine;
use matching_engine::orderbook::{OrderBook, Order, BidOrAsk};
use matching_engine::engine::{MathchingEngine, TraidingPair};
use rust_decimal_macros::dec;

fn main() {

    let buy_order_from_alice = Order::new(5.5, BidOrAsk::Bid);
    //let sell_order = Order::new(3.5, BidOrAsk::Ask);
    let buy_order_from_bob = Order::new(2.45, BidOrAsk::Bid);
    let mut orderbook = OrderBook::new();


    orderbook.add_limit_order(dec!(4.4), buy_order_from_alice);
    orderbook.add_limit_order(dec!(4.4), buy_order_from_bob);

    let sell_order = Order::new(6.5, BidOrAsk::Ask);
    orderbook.add_limit_order(dec!(20.0), sell_order);

    let mut engine = MathchingEngine::new();

    let pair = TraidingPair::new("BTC".to_string(), "USD".to_string());

    engine.add_new_market(pair.clone());
    
    let buy_order = Order::new(6.5, BidOrAsk::Bid);
    //let eth_pair = TraidingPair::new("ETH".to_string(), "USD".to_string());
    engine.place_limit_order(pair,dec!(10.000), buy_order).unwrap();

}
