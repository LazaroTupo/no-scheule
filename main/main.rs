use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger};
use dotenv::dotenv;
use std::env;
use teacher_system::infrastructure::api_restful::routes::app_routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    println!("🚀 Server running on http://localhost:{port}");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .service(app_routes())
    })
    .bind(("127.0.0.1", port.parse().unwrap()))
    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?
    .run()
    .await
    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}
