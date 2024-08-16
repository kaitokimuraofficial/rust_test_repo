use actix_web::{get, web, App, HttpServer, Responder};
use std::sync::Mutex;

struct AppState {
    app_name: String,
}

async fn index() -> impl Responder {
    "Hello world!"
}

async fn about() -> impl Responder {
    "Hello about!"
}

#[get("/")]
async fn greeting(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

async fn count_up(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("Request number: {counter}")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .app_data(counter.clone())
            .route("/count", web::get().to(count_up))
            .service(greeting)
            .service(web::scope("/app").route("/index.html", web::get().to(index)))
            .service(web::scope("/api").route("/about.html", web::get().to(about)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
