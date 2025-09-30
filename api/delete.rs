use redis::Commands;
use std::env;
use serde::Deserialize;

#[derive(Deserialize)]
struct Query {
    id: u64,
}

#[derive(serde::Serialize)]
struct Response {
    status: String,
}

pub fn main() {
    let url = env::var("REDIS_URL").expect("REDIS_URL not set");
    let client = redis::Client::open(url).unwrap();
    let mut con = client.get_connection().unwrap();

    let query: Query = serde_qs::from_str(&std::env::var("QUERY_STRING").unwrap()).unwrap();

    let todos: Vec<String> = con.lrange("todos", 0, -1).unwrap_or_default();
    for t in &todos {
        if let Ok(todo) = serde_json::from_str::<serde_json::Value>(t) {
            if todo["id"].as_u64() == Some(query.id) {
                let _: () = con.lrem("todos", 1, t).unwrap();
                println!("{}", serde_json::to_string(&Response {
                    status: format!("Todo {} deleted", query.id),
                }).unwrap());
                return;
            }
        }
    }

    println!("{}", serde_json::to_string(&Response {
        status: "not found".to_string(),
    }).unwrap());
}
