use crate::{components::navbar_acc::NavbarAcc, Route};
use yew::utils::window;
use yew::{
    classes, function_component, html, use_context, use_effect_with_deps, use_state, Callback,
    UseStateHandle,
};
use yew_router::prelude::*;
use yew_router::{attach_route_listener, RouteListener};

fn get_current_pathname() -> String {
    window().location().pathname().unwrap()
}

fn reset_active(states: Vec<UseStateHandle<&'static str>>) {
    states.iter().for_each(|s| s.set(""));
}

fn create_route_listener(cb: Callback<Option<Route>>) -> RouteListener {
    attach_route_listener(cb)
}

fn i_route(expect: Route) -> &'static str {
    if let Some(current) = Route::recognize(&get_current_pathname()) {
        if current == expect {
            "active"
        } else {
            ""
        }
    } else {
        ""
    }
}

fn mod_route(
    home: UseStateHandle<&'static str>,
    employees: UseStateHandle<&'static str>,
) -> Callback<Option<Route>> {
    Callback::from(move |_: Option<Route>| {
        reset_active(vec![home.clone(), employees.clone()]);
        if let Some(current) = Route::recognize(&get_current_pathname()) {
            match current {
                Route::Home => home.clone().set("active"),
                Route::Employees => employees.clone().set("active"),
                _ => {}
            }
        };
    })
}

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let ctx = use_context::<crate::GlobalContext>().expect("no ctx found");
    // Setup active indicator states
    let home = use_state(|| i_route(Route::Home));
    let employees = use_state(|| i_route(Route::Employees));
    let eh = home.clone();
    let ee = employees.clone();
    // Create route listener for indicator
    let listener = move || create_route_listener(mod_route(eh, ee));
    // Really weird logic to make it work
    let value_handle = use_state(listener.clone());
    use_effect_with_deps(
        move |_| {
            value_handle.set(listener());
            || {}
        },
        use_state(|| {}),
    );

    // Actual visuals
    html! {
        <nav class={classes!("navbar", "navbar-dark", "bg-dark", "navbar-expand-lg", "d-flex")}>
            <div class={classes!("container-fluid")}>
                <Link<Route> classes={classes!("navbar-brand")} route={Route::Home}>
                     <i class={classes!("bi", "bi-bank", "p-2")}></i>
                     {"Blorebank HRMS"}
                </Link<Route>>
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNavAltMarkup" aria-controls="navbarNavAltMarkup" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class={classes!("collapse", "navbar-collapse", "container-fluid", "row")} id="navbarNavAltMarkup">
                    <div class={classes!("col")}>
                        <div class={classes!("navbar-nav")}>
                            <Link<Route> classes={classes!("nav-link", *home)} route={Route::Home}>{"Home"}</Link<Route>>
                            <Link<Route> classes={classes!("nav-link", *employees, if ctx.logged {""} else {"disabled"})} route={Route::Employees}>{"Employees"}</Link<Route>>
                        </div>
                    </div>
                    <NavbarAcc />
                </div>
            </div>
        </nav>
    }
}
