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
		<link rel="preconnect" href="https://rsms.me/"/>
		<link rel="stylesheet" href="https://rsms.me/inter/inter.css"/>

		// sets the document title
		<Title text="Denux"/>

		// content for this welcome page
		<Router>
			<main>
				<Routes>
					<Route path="" view=HomePage/>
					<Route path="/*any" view=NotFound/>
				</Routes>
			</main>
		</Router>
	}
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
	view! {
			<div id="navbar">
				<a href="https://github.com/DenuxPlays/Denux-Dashboard" target="_blank">
					<img src="assets/github-logo.svg" alt="Github Project"/>
				</a>
			</div>
			<div class="centered" id="main">
				<p><b>"Welcome to nothing"</b></p>
			</div>
	}
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
	// set an HTTP status code 404
	// this is feature gated because it can only be done during
	// initial server-side rendering
	// if you navigate to the 404 page subsequently, the status
	// code will not be set because there is not a new HTTP request
	// to the server
	#[cfg(feature = "ssr")]
	{
		// this can be done inline because it's synchronous
		// if it were async, we'd use a server function
		let resp = expect_context::<leptos_actix::ResponseOptions>();
		resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
	}

	view! {
		<h1>"Not Found"</h1>
	}
}
