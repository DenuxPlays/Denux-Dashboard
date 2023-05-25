use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {

    use leptos::{Scope, ServerFnError};
    use sqlx::{SqliteConnection, Connection};
    use actix_web::web::Data;
    //use leptos_actix::DataResponse::Data;

    pub async fn get_conn() -> Result<SqliteConnection, ServerFnError> {
        SqliteConnection::connect("sqlite:dashboard.db").await.map_err(|e| ServerFnError::ServerError(e.to_string()))
    }
}
}
