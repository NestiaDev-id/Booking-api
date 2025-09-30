use redis::Commands;
use std::env;

#[derive(serde::Serialize, serde::Deserialize)]
struct Todo {
    id: u64,
    title: String,
    description: String,
}

pub fn main() {
    let url = env::var("REDIS_URL").expect("REDIS_URL not set");
    let client = redis::Client::open(url).unwrap();
    let mut con = client.get_connection().unwrap();

    let todos: Vec<String> = con.lrange("todos", 0, -1).unwrap_or_default();
    let parsed: Vec<Todo> = todos
        .into_iter()
        .filter_map(|s| serde_json::from_str(&s).ok())
        .collect();

    println!("{}", serde_json::to_string(&parsed).unwrap());
}
