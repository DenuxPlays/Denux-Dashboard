use cfg_if::cfg_if;

pub mod app;
pub mod auth;
pub mod components;
pub mod error_template;
pub mod errors;
pub mod fallback;
pub mod fileserv;
pub mod state;
pub mod user;
pub mod utilities;
pub mod page;

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
      use app::*;
      use leptos::*;

      // initializes logging using the `log` crate
      _ = console_log::init_with_level(log::Level::Debug);
      console_error_panic_hook::set_once();

      leptos::mount_to_body(move |cx| {
          view! { cx, <App/> }
      });
    }
}
}
