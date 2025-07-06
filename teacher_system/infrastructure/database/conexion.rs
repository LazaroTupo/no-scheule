use anyhow::Result;
use once_cell::sync::OnceCell;
use sea_orm::{Database, DatabaseConnection};

static DB_CONN: OnceCell<DatabaseConnection> = OnceCell::new();

pub async fn init_connection(database_url: &str) -> Result<()> {
    let conn = Database::connect(database_url).await?;
    DB_CONN
        .set(conn)
        .map_err(|_| anyhow::anyhow!("Conexión ya inicializada"))?;
    Ok(())
}

pub fn get_conn() -> &'static DatabaseConnection {
    DB_CONN.get().expect("La conexión no ha sido inicializada")
}
