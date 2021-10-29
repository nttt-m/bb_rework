use crate::Route;
use std::sync::Arc;
use yew::{classes, function_component, html, use_state, Callback};
use yew_router::{attach_route_listener, current_route, prelude::*};

fn is_active(nav: &str) -> &'static str {
    let route: Option<Route> = yew_router::current_route();
    if if let Some(current) = route {
        match current {
            Route::Home => nav == "home",
            Route::Employees => nav == "employees",
        }
    } else {
        false
    } {
        "active"
    } else {
        ""
    }
}

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class={classes!("navbar", "navbar-dark", "bg-dark", "navbar-expand-lg", "d-flex")}>
            <div class={classes!("container-fluid")}>
                <Link<Route> classes={classes!("navbar-brand")} route={Route::Home}>
                     // <img src={"/static/logo.svg"} alt="Blorebank Logo" class={classes!("h-12", "d-inline-block", "align-text-center")} style="filter: invert(100%) sepia(0%) saturate(0%) hue-rotate(23deg) brightness(105%) contrast(102%);" />
                     <i class={classes!("bi", "bi-bank", "p-2")}></i>
                     {"Blorebank HRMS"}
                </Link<Route>>
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNavAltMarkup" aria-controls="navbarNavAltMarkup" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class={classes!("collapse", "navbar-collapse")} id="navbarNavAltMarkup">
                    <div class={classes!("navbar-nav")}>
                        <Link<Route> classes={classes!("nav-link")} route={Route::Home}>{"Home"}</Link<Route>>
                        <Link<Route> classes={classes!("nav-link")} route={Route::Employees}>{"Employees"}</Link<Route>>
                    </div>
                </div>
            </div>
            <div class={classes!("container-fluid", "justify-content-end")}>
                <div class={classes!("navbar-nav")}>
                    <button classes={classes!("btn", "btn-secondary", "dropdown-toggle")}><i class={classes!("bi", "bi-person-circle", "p-2")}></i>{"Account"}</button>
                    <ul class="dropdown-menu" aria-labelledby="dropdownMenuButton1">
                        <li><a class="dropdown-item" href="#">{"Action"}</a></li>
                        <li><a class="dropdown-item" href="#">{"Another action"}</a></li>
                        <li><a class="dropdown-item" href="#">{"Something else here"}</a></li>
                    </ul>
                </div>
            </div>
        </nav>
    }
}
