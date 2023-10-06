use leptos::*;
use leptos_meta::provide_meta_context;

use crate::auth::get_user;

#[component]
pub fn ProfilePage() -> impl IntoView {
    let user = create_resource(move || {}, move |_| get_user());

    provide_meta_context();
    view! {
        <Transition fallback= move || view! {<span>"Loading..."</span>}>
        {move || {
                user.get().map(|user| match user {
                    Err(e) => view! {
                        <span>{format!("Login error: {}", e.to_string())}</span>
                    },
                    Ok(None) => view! {
                        <span><p>"Not logged in!"</p></span>
                    },
                    Ok(Some(user)) => view! {
                        <span>
                        <div class="center">
                            <p>{format!("Logged in as: {}", user.email)}</p>
                        </div>
                        </span>
                    }
                })
            }
        }
        </Transition>
    }
}
