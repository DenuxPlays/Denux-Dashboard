use crate::auth::Login;
use leptos::*;
use leptos_router::*;

#[component]
pub fn LoginPage(cx: Scope) -> impl IntoView {
    let login_action = create_server_action::<Login>(cx);
    view! { cx,
        <div class="centered" id="login-form">
            <ActionForm action=login_action>
                <label>
                    "E-Mail"
                    <input type="text" name="email" placeholder="Enter E-Mail"/>
                </label>
                <label>
                    "Password"
                    <input type="password" name="password" placeholder="Enter Password"/>
                </label>
                <input id="login" type="submit" value="Login"/>
                <input id="cancel" type="button" onclick="window.location.href='/';" value="Cancel" />
            </ActionForm>
      </div>
    }
}
