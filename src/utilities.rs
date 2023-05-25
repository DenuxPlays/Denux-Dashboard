use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use leptos::*;
    use sqlx::SqlitePool;

    pub fn get_pool(cx: Scope) -> Result<SqlitePool, ServerFnError> {
        use_context::<SqlitePool>(cx)
            .ok_or("Pool missing.")
            .map_err(|e| ServerFnError::ServerError(e.to_string()))
    }
}
}
