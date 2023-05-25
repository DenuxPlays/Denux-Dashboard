use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {

    use leptos::ServerFnError;
    use sqlx::{SqliteConnection, Connection};

    pub async fn get_conn() -> Result<SqliteConnection, ServerFnError> {
        SqliteConnection::connect("sqlite:dashboard.db").await.map_err(|e| ServerFnError::ServerError(e.to_string()))
    }
}
}
