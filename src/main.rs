use yew::prelude::*;
use yew_router::prelude::*;

#[path = "sites/html_util.rs"]
mod html_util;

#[derive(Clone, Routable, PartialEq, Debug)]
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

#[function_component(Main)]
fn app() -> Html {
    return html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
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
            <p>{"URL not found"}</p>
                <div class={"home-button"}>
                    <button><Link<Route> to={Route::Home}>{ "click here to go home" }</Link<Route>></button>
                </div>
        </>
    };
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
