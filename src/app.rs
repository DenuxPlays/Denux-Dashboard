use crate::auth::get_user;
use crate::page::login::LoginPage;
use crate::user::profile::*;
use crate::user::start_page::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/denux_dashboard.css"/>

        // sets the document title
        <Title text="Denux"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="login" view=LoginPage/>
                    <Route path="user/start" view=StartPage/>
                    <Route path="user/profile" view=ProfilePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let user = create_resource(move || {}, move |_| get_user());
    provide_meta_context();

    view! {
            <div id="navbar">
                <a href="https://github.com/DenuxPlays/Denux-Dashboard" target="_blank">
                    <img src="/github-logo.svg" alt="Github Project"/>
                </a>
                <Transition fallback=move || view! {<span class="navbar-right">"Loading..."</span>}>
                    {move || {
                        user.get().map(|user| match user {
                            Err(e) => view! {
                                <a class="navbar-right" href="/login">"Login"</a>
                                <span id="navbar-right">{format!("Login error: {}", e.to_string())}</span>
                            }.into_view(),
                            Ok(None) => view! {
                                <a class="navbar-right" href="/login">"Login"</a>
                            }.into_view(),
                            Ok(Some(user)) => view! {
                                <a class="navbar-right" href="/user/profile">{format!("{}", user.email)}</a>
                        }.into_view()
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
