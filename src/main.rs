use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::sync::Mutex;

#[derive(Clone)]
struct Todo {
    id: u64,
    title: String,
    description: String,
}

struct AppState {
    todos: Mutex<Vec<Todo>>,
}

#[derive(Deserialize)]
struct AddParams {
    title: String,
    description: String,
}

// GET /todos → list semua todo
async fn get_todos(data: web::Data<AppState>) -> impl Responder {
    let todos = data.todos.lock().unwrap();
    let list: Vec<_> = todos.iter()
        .map(|t| format!("{}: {} - {}", t.id, t.title, t.description))
        .collect();
    HttpResponse::Ok().body(list.join("\n"))
}

// GET /todos/add?title=...&description=...
async fn add_todo(params: web::Query<AddParams>, data: web::Data<AppState>) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();
    let todo = Todo {
        id: todos.len() as u64 + 1,
        title: params.title.clone(),
        description: params.description.clone(),
    };
    todos.push(todo);
    HttpResponse::Ok().body("Todo added!")
}

// GET /todos/delete/{id}
async fn delete_todo(path: web::Path<u64>, data: web::Data<AppState>) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();
    let id = path.into_inner();
    todos.retain(|t| t.id != id);
    HttpResponse::Ok().body(format!("Todo {} deleted", id))
}

async fn not_found() -> impl Responder {
    HttpResponse::NotFound()
        .content_type("application/json")
        .body(r#"{"error": "Endpoint not found"}"#)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        todos: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/todos", web::get().to(get_todos))
            .route("/todos/add", web::get().to(add_todo))
            .route("/todos/delete/{id}", web::get().to(delete_todo))
            .default_service(web::route().to(not_found))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
