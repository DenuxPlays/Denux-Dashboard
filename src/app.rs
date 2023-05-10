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
                <a href="https://github.com/DenuxPlays" target="_blank">
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
        view! { cx,
        <div class="centered">
            <label for="uname"><b>"Username"</b></label>
            <input type="text" placeholder="Enter Username" name="uname" required />    

            <label for="pasw"><b>"Password"</b></label>
            <input type="password" placeholder="Enter Password" name="pasw" required />

            <button type="submit">"Login"</button>
      </div>
    }
}
