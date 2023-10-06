use crate::auth::{get_user, Logout};
use leptos::*;
use leptos_meta::{provide_meta_context, Title};
use leptos_router::*;

#[component]
pub fn StartPage() -> impl IntoView {
    let logout_action = create_server_action::<Logout>();
    let user = create_resource(move || {}, move |_| get_user());
    provide_meta_context();

    view! {
        <Title text="Denux - Logged in!" />

        <Transition fallback=move || view! {<span>"Loading..."</span>}>
        {move || {
            user.get().map(|user| match user {
                Err(e) => view! {
                    <div>
                    <p>"Login"</p>
                    <span>{format!("Login error: {}", e.to_string())}</span>
                    </div>
                },
                Ok(None) => view! {
                    <div><p>"Not logged in!"</p></div>
                },
                Ok(Some(user)) => view! {
                    <div>
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
                    </div>
                }
            })
            }
        }
        </Transition>
    }
}
