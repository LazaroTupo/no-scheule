use crate::domain::models::User;
use supabase_rs::SupabaseClient;

pub struct SupabaseAuthService {
    client: SupabaseClient,
}

impl SupabaseAuthService {
    pub fn new(client: SupabaseClient) -> Self {
        Self { client }
    }

    pub async fn authenticate(&self, token: &str) -> Result<User, String> {
        let response = self
            .client
            .auth()
            .user(token)
            .await
            .map_err(|e| e.to_string())?;

        if response.status().is_success() {
            let user: User = response.json().await.map_err(|e| e.to_string())?;
            Ok(user)
        } else {
            Err("Autenticación fallida".into())
        }
    }
}
