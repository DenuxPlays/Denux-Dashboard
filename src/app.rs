use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::auth::Login;

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
                    <Route path="user/start" view=|cx| view! {cx, <UserHomepage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
            <div id="navbar">
                <a href="https://github.com/DenuxPlays/Denux-Dashboard" target="_blank">
                    <img src="github-logo.svg" alt="Github Profile"/>
                </a>
                <div id="navbar-right">
                    <a href="/login">"Login"</a>
                </div>
            </div>
            <div class="centered" id="main">
                <p><b>"Welcome to nothing"</b></p>
            </div>
    }
}

#[component]
fn LoginPage(cx: Scope) -> impl IntoView {
    let login_action = create_server_action::<Login>(cx);
    view! { cx,
        <div class="centered">
            <ActionForm action=login_action>
                <label>
                    "E-Mail"
                    <input type="text" name="email"/>
                </label>
                <label>
                    "Password"
                    <input type="password" name="password"/>
                </label>
                <input type="submit" value="Login"/>
            </ActionForm>
      </div>
    }
}

#[component]
fn UserHomepage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Denux - Logged in!" />
        <div class="centered">
            <p><b>"Logged in!"</b></p>
        </div>
    }
}
