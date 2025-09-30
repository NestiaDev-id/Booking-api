use redis::Commands;
use std::env;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct Query {
    title: String,
    description: String,
}

#[derive(serde::Serialize)]
struct Response {
    status: String,
    todo: HashMap<String, String>,
}

pub fn main() {
    let url = env::var("REDIS_URL").expect("REDIS_URL not set");
    let client = redis::Client::open(url).unwrap();
    let mut con = client.get_connection().unwrap();

    let query: Query = serde_qs::from_str(&std::env::var("QUERY_STRING").unwrap()).unwrap();

    let id = chrono::Utc::now().timestamp_millis() as u64;
    let todo = serde_json::json!({
        "id": id,
        "title": query.title,
        "description": query.description
    });

    let _: () = con.rpush("todos", todo.to_string()).unwrap();

    let mut resp = HashMap::new();
    resp.insert("id".to_string(), id.to_string());
    resp.insert("title".to_string(), query.title);
    resp.insert("description".to_string(), query.description);

    println!("{}", serde_json::to_string(&Response {
        status: "added".to_string(),
        todo: resp,
    }).unwrap());
}
