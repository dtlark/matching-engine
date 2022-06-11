extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;
use std::fs::File;
use std::io::Read;
use std::collections::VecDeque;
use std::collections::HashMap;


#[derive(Serialize, Deserialize)]
struct Order {
    otype: char,
    firm_id: u32,
    symbol: String,
    quantity: u32,
    side: bool,
    price: f32
}

struct Matcher {
    buys: VecDeque<u32>,
    sells: VecDeque<u32>,
    firms: HashMap<u32, bool>,
}

impl Default for Matcher {
    fn default() -> Matcher {
        Matcher {
            buys: VecDeque::new(),
            sells: VecDeque::new(),
            firms: HashMap::new()
        }
    }
}

impl Matcher {
    pub fn new_order(firmId: u32, symbol: &String, side: bool, price: f32) {

    }
    pub fn modify_order(firmId: u32, symbol: &String, price: f32) {

    }
    pub fn cancel_order(firmId: u32, symbol: &String) {

    }
}


fn main() {

    let mut file = File::open("./src/orders.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let orders: Vec<Order> = serde_json::from_str(&data).unwrap();

    for order in orders.iter() {
        match order.otype {
            'N'=>Matcher::new_order(order.firm_id, &order.symbol, order.side, order.price),
            'M'=>Matcher::modify_order(order.firm_id, &order.symbol, order.price),
            'C'=>Matcher::cancel_order(order.firm_id, &order.symbol),
            _=>println!("Invalid order type"),
        }
        println!("{:#?}", order.otype);
    }
}