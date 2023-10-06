use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use leptos::*;
    use sqlx::SqlitePool;
    use crate::auth::*;

    pub fn get_pool() -> Result<SqlitePool, ServerFnError> {
        use_context::<SqlitePool>()
            .ok_or("Pool missing.")
            .map_err(|e| ServerFnError::ServerError(e.to_string()))
    }

    pub fn auth() -> Result<AuthSession, ServerFnError> {
        use_context::<AuthSession>()
            .ok_or("Auth session missing.")
            .map_err(|e| ServerFnError::ServerError(e.to_string()))
    }
}
}
