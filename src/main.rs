#![allow(non_camel_case_types)]
mod components;
mod pages;

mod prelude {
    pub use crate::components::*;
    pub use crate::pages::*;
    pub use gloo_timers::callback::Timeout;
    pub use reqwasm::http::Request;
    pub use std::collections::HashMap;
    pub use std::ops::Deref;
    pub use wasm_bindgen::{JsCast, JsValue};
    pub use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlSelectElement, HtmlSpanElement};
    pub use yew::prelude::*;
    pub use yew_router::prelude::*;
}

use prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Content,
}

fn match_route(route: Route) -> Html {
    match route {
        Route::Content => html!(<Content/>),
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <>
            <BrowserRouter>
                <Switch<Route> render={match_route} />
            </BrowserRouter>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
