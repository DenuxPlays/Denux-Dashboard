use leptos::*;
use leptos_meta::provide_meta_context;

use crate::auth::get_user;

#[component]
pub fn ProfilePage(cx: Scope) -> impl IntoView {
    let user = create_resource(cx, move || {}, move |_| get_user(cx));

    provide_meta_context(cx);
    view! {cx,
        <Transition fallback= move || view! {cx, <span>"Loading..."</span>}>
        {move || {
                user.read(cx).map(|user| match user {
                    Err(e) => view! {cx,
                        <span>{format!("Login error: {}", e.to_string())}</span>
                    }.into_view(cx),
                    Ok(None) => view! {cx,
                        <p>"Not logged in!"</p>
                    }.into_view(cx),
                    Ok(Some(user)) => view! {cx,
                        <div class="center">
                            <p>{format!("Logged in as: {}", user.email)}</p>
                        </div>
                    }.into_view(cx)
                })
            }
        }
        </Transition>
    }
}
