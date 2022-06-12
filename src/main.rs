extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;
use std::fs::File;
use std::io::Read;
use std::collections::VecDeque;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Order {
    otype: char,
    order_id: u32,
    firm_id: u32,
    symbol: String,
    quantity: u32,
    side: bool,
    price: f32
}

pub struct Stock {
    buys: VecDeque<(u32, f32)>,
    sells: VecDeque<(u32, f32)>,
    firms: HashMap<u32, bool>,
}

pub fn new_order(order: &Order, stock_orders: &HashMap<String, Stock>) { //pub fn new_order(firmId: u32, symbol: &String, side: bool, price: f32) 
    if !stock_orders.contains_key(&order.symbol) {
        println!("In new order");
    }
}

pub fn modify_order(firm_id: u32, symbol: &String, price: f32) {
}

pub fn cancel_order(firm_id: u32, symbol: &String) {
}

fn main() {

    let mut file = File::open("./src/orders.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let orders: Vec<Order> = serde_json::from_str(&data).unwrap();

    let stock_orders: HashMap<String, Stock> = HashMap::new();
    let firm_info: HashMap<String, (u32, u32, u32)> = HashMap::new();

    for order in orders.iter() {
        match order.otype {
            'N'=>new_order(order, &stock_orders),
            'M'=>modify_order(order.firm_id, &order.symbol, order.price),
            'C'=>cancel_order(order.firm_id, &order.symbol),
            _=>println!("Invalid order type"),
        }
        println!("{:#?}", order.otype);
    }
}