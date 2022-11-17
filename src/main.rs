use yew::prelude::*;
use yew_router::prelude::*;
use bounce::helmet::{Helmet, HelmetBridge};
use bounce::BounceRoot;

#[derive(Clone, Routable, PartialEq, Eq, Debug)]
pub enum Route {
    #[at("/")]
    Home,

    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
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
            <Helmet>
                <meta charset="utf-8" />
                <link rel="stylesheet" href="css/main.css" />
            </Helmet>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </BounceRoot>
    };
}

#[function_component(Home)]
fn home() -> Html {
    return html! {
        <>
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
            </Helmet>
            <p>{"URL not found"}</p>
            <div class={"home-button"}>
                <button><Link<Route> to={Route::Home}>{ "click here to go home" }</Link<Route>></button>
            </div>
        </>
    };
}

fn main() {
    yew::start_app::<App>();
}
