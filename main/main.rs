use dotenv::dotenv;
use teacher_system::infrastructure::database::conexion;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    // Configuración inicial
    let db_url = env::var("POSTGRES_URL")?;
    conexion::init(&db_url).await?;

    // Aquí iría la lógica de la aplicación (API, CLI, etc.)
    print!("Aplicación de gestión de profesores iniciada correctamente.\n");
    Ok(())
}
