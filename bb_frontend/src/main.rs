mod components;
mod pages;

use crate::{components::navbar::Navbar, pages::home::Home};
use yew::prelude::*;
use yew_router::{Routable, Router};

#[derive(Clone, Routable)]
pub enum Route {
    #[at("/")]
    Home,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home /> }
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Navbar />
            <div class="container mx-auto text-green-800 text-xl">
                <Router<Route> render={Router::render(switch)} />
            </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
