mod agents;
mod components;
mod js_bind;
mod pages;

use crate::pages::employees::EmployeeQuery;
use crate::{
    components::navbar::Navbar,
    pages::{auth::Auth, employees::Employees, home::Home, not_found::NotFound},
};
use yew::prelude::*;
use yew_router::{Routable, Router};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Clone, Debug, PartialEq)]
pub struct GlobalContext {
    pub logged: bool,
    pub id: i32,
    pub admin: bool,
}

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/employees")]
    Employees,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route, ctx: UseStateHandle<GlobalContext>) -> Html {
    match routes {
        Route::Home => {
            html! { <Home /> }
        }
        Route::Employees => {
            let query = yew_router::parse_query::<EmployeeQuery>();
            if ctx.logged {
                if let Ok(q) = query {
                    html! { <Employees view={q.view} focus={q.focus} /> }
                } else {
                    html! { <Employees view={None} focus={None} /> }
                }
            } else {
                yew_router::push_route(Route::Home);
                html! {}
            }
        }
        Route::Login => {
            html! { <Auth login=true /> }
        }
        Route::Register => {
            html! { <Auth login=false /> }
        }
        Route::NotFound => {
            html! { <NotFound /> }
        }
    }
}

fn init_context() -> GlobalContext {
    GlobalContext {
        logged: true,
        id: 12345,
        admin: false,
    }
}

#[function_component(App)]
fn app() -> Html {
    // let storage = web_sys::Storage { obj: val };
    let ctx = use_state(init_context);
    html! {
        <ContextProvider<GlobalContext> context={(*ctx).clone()}>
            <Navbar />
            <div class="container">
                <Router<Route> render={Router::render(move |r| switch(r, ctx.clone()))} />
            </div>
        </ContextProvider<GlobalContext>>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
