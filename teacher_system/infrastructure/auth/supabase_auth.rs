use crate::domain::models::user::User;
use std::sync::Arc;
use supabase_rs::SupabaseClient;

pub struct SupabaseAuthService {
    client: Arc<SupabaseClient>,
}

impl SupabaseAuthService {
    pub fn new(client: Arc<SupabaseClient>) -> Self {
        Self { client }
    }

    pub async fn authenticate(&self, token: &str) -> Result<User, String> {
        println!("Autenticando usuario con token: {}", token);
        Err("Función no implementada".into())
    }
}
