use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use std::fs;
use std::io;

struct AppState {
    path: Mutex<String>,
    current_file: Mutex<String>,
}

async fn index(data: web::Data<AppState>) -> String {
    let path = data.path.lock().unwrap();
    // let paths = fs::read_dir(path.to_string()).unwrap();
    // for pp in paths {
    //     println!("Name: {}", pp.unwrap().path().display())
    // }

    format!("Request number: @ {path}") // <- response with count
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Awaiting Path (Empty string indicates current directory):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    println!("Looking at: {}",input);

    let counter = web::Data::new(AppState {
        current_file: Mutex::new("".to_string()),
        path: Mutex::new(input),
    });

    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .app_data(counter.clone()) // <- register the created data
            .route("/", web::get().to(index))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
