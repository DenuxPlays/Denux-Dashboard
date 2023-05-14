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
    let (password, set_password) = create_signal(cx, String::new());
    let (email, set_email) = create_signal(cx, String::new());
    
    let mut logged_in: bool = false;
    let login_action = move |_| login(email.get(), password.get(), &mut logged_in);

    view! { cx,
        <div class="centered">
            <label for="uname"><b>"E-Mail"</b></label>
            <input type="text" placeholder="Enter your E-Mail adress." name="email" required 
                on:keyup=move |ev: ev::KeyboardEvent| {
                    let val = event_target_value(&ev);
                    set_email.update(|v| *v = val);
                }
                on:change=move |ev| {
                    let val = event_target_value(&ev);
                    set_email.update(|v| *v = val);
                }
            />
            <label for="passw"><b>"Password"</b></label>
            <input type="password" placeholder="Enter Password" name="password" required 
                on:keyup=move |ev: ev::KeyboardEvent| {
                    let val = event_target_value(&ev);
                    set_password.update(|v| *v = val);
                }
                on:change=move |ev| {
                    let val = event_target_value(&ev);
                    set_password.update(|v| *v = val);
                }
 
            />

            <button on:click=login_action>
            "Login"
            </button>
      </div>
    }
}

fn login(email: String, password: String, result: &mut bool) {
    log::info!("Login button pressed");
    if email.eq_ignore_ascii_case("test@du-hs.dev") {
       if password.eq("123HS") {
            *result = true;
            return;
       }
    }
    *result = false;
}
