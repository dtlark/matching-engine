extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;
use std::fs::File;
use std::io::Read;


#[derive(Serialize, Deserialize)]
struct Order {
    firm_id: u32,
    symbol: String,
    quantity: u32,
    side: bool,
    price: f32
}

fn main() {

    let mut file = File::open("./src/orders.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let json: Vec<Order> = serde_json::from_str(&data).unwrap();

    for datum in json.iter() {
        println!("{:#?}", datum.firm_id);
    }

}