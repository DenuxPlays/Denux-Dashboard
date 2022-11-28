use yew::prelude::*;
use yew_router::prelude::*;
use bounce::helmet::{Helmet, HelmetBridge};
use bounce::BounceRoot;

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Home,

    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    return match routes {
        Route::Home => html! { <Home /> },
        Route::NotFound => html! { <NotFound /> },
    };
}

#[function_component(App)]
fn app() -> Html {
    return html! {
        <BounceRoot>
            <HelmetBridge default_title="Denux"/>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </BounceRoot>
    };
}

#[function_component(Home)]
fn home() -> Html {
    return html! {
        <>
        <Helmet>
            <link rel="stylesheet" href="assets/css/main.css" />
            <link rel="icon" href="assets/img/ServerLogo.jpg" />
        </Helmet>
            <body>
                <p>{"Nothing to see"}</p>
            </body>
        </>
    };
}

#[function_component(NotFound)]
pub fn not_found() -> Html {
    return html! {
        <>
            <Helmet>
                <title>{"Page not found"}</title>
                <link rel="stylesheet" href="assets/css/sites/404.css" />
                <link rel="icon" href="assets/img/ServerLogo.jpg" />
            </Helmet>
            <p>{"URL not found"}</p>
            <button><Link<Route> to={Route::Home}>{ "click here to go home" }</Link<Route>></button>
        </>
    };
}

fn main() {
    yew::Renderer::<App>::new().render();
}