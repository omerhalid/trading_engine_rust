mod matching_engine;
use matching_engine::orderbook::{OrderBook, Order, BidOrAsk};
use matching_engine::engine::MathchingEngine;

fn main() {

    let buy_order_from_alice = Order::new(5.5, BidOrAsk::Bid);
    //let sell_order = Order::new(3.5, BidOrAsk::Ask);
    let buy_order_from_bob = Order::new(2.45, BidOrAsk::Bid);
    let mut orderbook = OrderBook::new();


    orderbook.add_order(4.4, buy_order_from_alice);
    orderbook.add_order(4.4, buy_order_from_bob);

    let sell_order = Order::new(6.5, BidOrAsk::Ask);
    orderbook.add_order(20.0, sell_order);

    let engine = MathchingEngine::new();

    println!("{:?}", orderbook);
}
