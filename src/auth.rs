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
    use sqlx::SqlitePool;
    use crate::utilities::get_conn;

    impl User {
        pub async fn get(id: i64) -> Option<Self> {
            let mut conn = get_conn().await.ok()?;
            let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE id = ?")
                .bind(id)
                .fetch_one(&mut conn)
                .await
                .ok()?;

            Some(sqluser.into_user())
        }

        pub async fn get_from_email(email: String) -> Option<Self> {
            let mut conn = get_conn().await.ok()?;
            let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE email = ?")
               .bind(email)
               .fetch_one(&mut conn)
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
    
}
}

#[server(Login, "/api")]
pub async fn login(
    cx: Scope,
    email: String,
    password: String,
    ) -> Result<(), ServerFnError> {
    log::info!("Email {}", email);
   let user: User = User::get_from_email(email)
       .await
       .ok_or("User does not exist.")
       .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

   if password.eq(&user.password) {
       leptos_actix::redirect(cx, "/user/start");
       Ok(())
   } else {
        Err(ServerFnError::ServerError("Password does not match.".to_string()))
   }
}
