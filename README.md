# Rust Trading Engine

This is a simple order-matching trading engine built in the Rust programming language. The engine supports the creation of multiple markets (trading pairs) and allows for the placement of buy and sell orders.

## Overview

The trading engine consists of the following main components:

- **OrderBook**: Contains all the buy (Bid) and sell (Ask) orders in a market. Orders are divided into 'Bid' and 'Ask' orders, and are stored in separate `HashMap` collections, each associated with a specific price level.
- **Order**: Represents a buy (Bid) or sell (Ask) order with a specific size (amount).
- **Limit**: Groups all orders at a particular price level. Includes functions to fill orders and compute the total volume of orders at that limit level.
- **MatchingEngine**: The core engine that executes trading operations. It maintains a collection of `OrderBooks`, one for each trading pair. Supports functions like adding a new market (a new trading pair), and placing limit orders.
- **TradingPair**: Represents a pair of currencies that can be traded. The 'base' is the currency being bought or sold, and the 'quote' is the currency used as a reference for price.

The engine operates by constantly checking for matchable orders (i.e., a sell order and a buy order at the same price). When a match is found, the orders are "filled" and removed from the `OrderBook`. The engine uses Limit Orders to ensure trades are only executed at the limit price or better.

## Usage

To run the trading engine, clone the repository and use Rust's `cargo run` command:

```bash
$ git clone https://github.com/omerhalid/rust-trading-engine.git
$ cd rust-trading-engine
$ cargo run
