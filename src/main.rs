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
    buys: VecDeque<u32>,
    sells: VecDeque<u32>,

}

impl Stock {
    fn new() -> Stock {
        Stock {
            buys: VecDeque::new(),
            sells: VecDeque::new(),  
        }
    }
}

pub struct Matcher {
    stock_orders: HashMap<String, Stock>,
    trade_info: HashMap<String, (u32, u32, u32)>,
    order_list: HashMap<u32, Order>,
}

impl Matcher{
    fn new() -> Matcher {
        Matcher {
            stock_orders: HashMap::new(),
            trade_info: HashMap::new(),
            order_list: HashMap::new()
        }
    }
    pub fn new_order(&mut self, order: &Order) { //pub fn new_order(firmId: u32, symbol: &String, side: bool, price: f32) 
        let order_symbol = order.symbol.to_string();
        if !self.stock_orders.contains_key(&order.symbol) {
            self.stock_orders.insert(order_symbol, Stock::new());
        }

        if order.side { //order side: 1 == buy
           // self.stock_orders[&order_symbol].buys.push_back(1);
        } else { //sell

        }

    }

    pub fn cancel_order(&mut self, order_id: u32) {
        let order_symbol = self.order_list[&order_id].symbol.to_string();
        if self.stock_orders.contains_key(&order_symbol) { //if the order symbol exists
            if self.stock_orders[&order_symbol].buys.contains(&order_id) { //if the orderid exists in buy
                //self.stock_orders[&order_symbol].buys.retain(|&x| x != order_id); //retain if element != order_id (remove order_id)
            } else if self.stock_orders[&order_symbol].sells.contains(&order_id) {  //if the orderid exists in sells
                //self.stock_orders[&order_symbol].sells.retain(|&x| x != order_id); //retain if element != order_id (remove order_id)
            }
        }
    }

    pub fn print(&self) {
        println!("Firm ID: Active | Filled | $");
        for (key, value) in &self.trade_info {
            //println!("{}: {}", key, value);
        }
    }
}

fn main() {

    let mut file = File::open("./src/orders.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let orders: Vec<Order> = serde_json::from_str(&data).unwrap();

    let mut matcher: Matcher = Matcher::new();

    for order in orders.iter() {
        match order.otype {
            'N'=>matcher.new_order(order),
            'C'=>matcher.cancel_order(order.order_id),
            _=>println!("Invalid order type"),
        }
        //println!("{:#?}", order.otype);
    }
    matcher.print();
}