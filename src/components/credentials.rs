/*use leptos::{component, create_signal, Action, IntoView, Scope, Signal, SignalGet};

#[component]
pub fn LoginForm(
    cx: Scope,
    button_label: &'static str,
    action: Action<(String, String), ()>,
    error: Signal<Option<String>>,
    disabled: Signal<bool>,
) -> impl IntoView {
    let (password, set_password) = create_signal(cx, String::new());
    let (email, set_email) = create_signal(cx, String::new());

    let loggin_action = move || action.dispatch((email.get(), password.get()));
}*/
