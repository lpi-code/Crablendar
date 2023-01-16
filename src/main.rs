use actix_web::http::header::HeaderValue;
use actix_web::{web, App, HttpServer, HttpResponse};
use tokio::sync::RwLock;
mod scheduler;
mod calendar;

use std::sync::Arc;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Crablendar server");
    // Make sure exit on Ctrl + C
    ctrlc::set_handler(move || {
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");
    let calendar_storage = Arc::new(RwLock::new("Initial calendar".to_string())); 
    // Start the web server in the main thread
    let lock1 = web::Data::new(calendar_storage.clone());
    let server = HttpServer::new(move || {
        App::new()
            .app_data(lock1.clone())
            .route("/", web::get().to(index))
            
    })
    .bind(("0.0.0.0", 8080)).unwrap_or_else(|_| panic!("Can not bind to port 8080"))
    .run();
    // Start the background task in a separate thread
    let lock2 = calendar_storage.clone();
    let result = tokio::join!(
        server,
        scheduler::scheduler(lock2)
    );
    result.0 // Return the result of the web server

    // Wait for the web server to finish
    // Wait for the background task to finish
    //bj.await?;

    
}

async fn index(
    calendar_storage: web::Data<Arc<RwLock<String>>>,
) -> impl actix_web::Responder {
    println!("Request received");
    let calendar = calendar_storage.read().await;
    println!("Calendar lock aquired");
    let inner_calendar = calendar.clone();
    HttpResponse::Ok()
        .content_type("text/ics")
        // Add Content disposition header
        .append_header((
            "Content-Disposition",
            HeaderValue::from_static("attachment; filename=\"calendar.ics\""),
        ))
        .body(inner_calendar)
}

