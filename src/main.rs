use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
#[derive(Debug)]
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
    }
}

#[function_component(Main)]
fn app() -> Html {
    return html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

#[function_component(Home)]
fn home() -> Html {
    return html! {
        <>
            <head>
                <title>{"Denux"}</title>
            </head>
            <body>
                <p>{"Nothing to see"}</p>
            </body>
        </>
    }
}

#[function_component(NotFound)]
pub fn not_found() -> Html {
    return html! {
        <>
            <head>
                <title>{"URL not found"}</title>
            </head>
            <body>
                <p>{"URL not found"}</p>
                <Link<Route> to={Route::Home}>{ "click here to go home" }</Link<Route>>
            </body>
        </>
    }
}

fn main() {
    yew::start_app::<Main>();
}