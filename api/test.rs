use std::collections::HashMap;

pub fn main() {
    let mut res = HashMap::new();
    res.insert("message", "Hello from Rust on Vercel!");
    println!("{}", serde_json::to_string(&res).unwrap());
}
