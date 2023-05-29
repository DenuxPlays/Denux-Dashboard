use crate::auth::{get_user, Logout};
use leptos::*;
use leptos_meta::{provide_meta_context, Title};
use leptos_router::*;

#[component]
pub fn StartPage(cx: Scope) -> impl IntoView {
    let logout_action = create_server_action::<Logout>(cx);
    let user = create_resource(cx, move || {}, move |_| get_user(cx));
    provide_meta_context(cx);

    view! { cx,
        <Title text="Denux - Logged in!" />

        <Transition fallback=move || view! {cx, <span>"Loading..."</span>}>
        {move || {
            user.read(cx).map(|user| match user {
                Err(e) => view! {cx,
                    <p>"Signup"</p>
                    <p>"Login"</p>
                    <span>{format!("Login error: {}", e.to_string())}</span>
                }.into_view(cx),
                Ok(None) => view! {cx,
                    <p>"Not logged in!"</p>
                }.into_view(cx),
                Ok(Some(user)) => view! {cx,
                    <div id="navbar">
                        <div class="navbar-right">
                            <ActionForm action=logout_action>
                                <input type="submit" value="Logout"/>
                            </ActionForm>
                        </div>
                    </div>
                    <div class="centered">
                        <p><b>{format!("Logged in as: {} ({})", user.email, user.id)}</b></p>
                    </div>
                }.into_view(cx)
            })
            }
        }
        </Transition>
    }
}
