use cfg_if::cfg_if;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password: String,
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    use async_trait::async_trait;
    use sqlx::SqlitePool;
    use axum_session_auth::{SessionSqlitePool, Authentication};
    pub type AuthSession = axum_session_auth::AuthSession<User, i64, SessionSqlitePool, SqlitePool>;

    impl User {
        pub async fn get(id: i64, pool: &SqlitePool) -> Option<Self> {
            let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE id = ?")
                .bind(id)
                .fetch_one(pool)
                .await
                .ok()?;

            Some(sqluser.into_user())
        }

        pub async fn get_from_email(email: String, pool: &SqlitePool) -> Option<Self> {
            let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE email = ?")
               .bind(email)
               .fetch_one(pool)
               .await
               .ok()?;

           Some(sqluser.into_user())
        }
    }

    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlUser {
        pub id: i64,
        pub email: String,
        pub password: String,
    }

    impl SqlUser {
        pub fn into_user(self) -> User {
            User {
                id: self.id,
                email: self.email,
                password: self.password,
            }
        }
    }
    
    #[async_trait]
    impl Authentication<User, i64, SqlitePool> for User {
        async fn load_user(userid: i64, pool: Option<&SqlitePool>) -> Result<User, anyhow::Error> {
            let pool = pool.unwrap();

            User::get(userid, pool)
                .await
                .ok_or_else(|| anyhow::anyhow!("Cannot get user"))
        }

        fn is_authenticated(&self) -> bool { true }

        fn is_active(&self) -> bool { true }

        fn is_anonymous(&self) -> bool { false }
    }
}
}

#[server(Login, "/api")]
pub async fn login(cx: Scope, email: String, password: String) -> Result<(), ServerFnError> {
    let pool = crate::utilities::get_pool(cx)?;
    let user: User = User::get_from_email(email, &pool)
        .await
        .ok_or("User does not exist.")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    if password.eq(&user.password) {
        leptos_axum::redirect(cx, "/user/start");
        Ok(())
    } else {
        Err(ServerFnError::ServerError(
            "Password does not match.".to_string(),
        ))
    }
}
