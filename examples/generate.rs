extern crate uuidv6_rs;

fn main() {
    println!("uuidv6: {:?}", uuidv6_rs::new_v6().to_string());
}