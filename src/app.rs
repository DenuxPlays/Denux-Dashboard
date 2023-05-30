use crate::page::login::LoginPage;
use crate::auth::get_user;
use crate::user::start_page::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/denux_dashboard.css"/>

        // sets the document title
        <Title text="Denux"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="login" view=|cx| view! {cx, <LoginPage/> }/>
                    <Route path="user/start" view=|cx| view! {cx, <StartPage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let user = create_resource(cx, move || {}, move |_| get_user(cx));
    provide_meta_context(cx);

    view! { cx,
            <div id="navbar">
                <a href="https://github.com/DenuxPlays/Denux-Dashboard" target="_blank">
                    <img src="/github-logo.svg" alt="Github Project"/>
                </a>
                <Transition fallback=move || view! {cx, <span class="navbar-right">"Loading..."</span>}>
                    {move || {
                        user.read(cx).map(|user| match user {
                            Err(e) => view! {cx,
                                <a class="navbar-right" href="/login">"Login"</a>
                                <span id="navbar-right">{format!("Login error: {}", e.to_string())}</span>
                            }.into_view(cx),
                            Ok(None) => view! {cx,
                                <a class="navbar-right" href="/login">"Login"</a>
                            }.into_view(cx),
                            Ok(Some(user)) => view! {cx,
                                <a class="navbar-right" href="/user/start">{format!("{}", user.email)}</a>
                        }.into_view(cx)
                        })
                        }
                 }
                </Transition>
            </div>
            <div class="centered" id="main">
                <p><b>"Welcome to nothing"</b></p>
            </div>
    }
}
