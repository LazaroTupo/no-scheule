use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web};
use dotenv::dotenv;
use teacher_system::infrastructure::api_restful::config::boostrap;
use teacher_system::infrastructure::api_restful::routes::app_routes;

#[tokio::main]
async fn main() -> Result<(), String> {
    dotenv().ok();
    env_logger::init();

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    // Initialize app state
    let app_state = boostrap::bootstrap_server().await?;

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
            .app_data(web::Data::new(app_state.clone()))
            .service(app_routes())
    })
    .bind(("127.0.0.1", port.parse().unwrap()))
    .map_err(|e| e.to_string())?
    .run()
    .await
    .map_err(|e| e.to_string())
}
