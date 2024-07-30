mod app;
mod model;
mod compute;
mod utils;
mod decoders;
mod errors;

use app::*;
use leptos::*;

pub fn main() {

    if web_sys::window().is_some() {
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();
        logging::log!("csr mode - mounting to body");
        mount_to_body(|| view! { <App/> })
    } else
    {
        // this is a worker, do nothing
    }
}