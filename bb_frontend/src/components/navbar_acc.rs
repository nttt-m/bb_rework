use crate::{pages::employees::EmployeeQuery, GlobalContext, Route};
use yew::prelude::*;
use yew_router::{prelude::*, push_route, push_route_with_query};

fn edit_self(ctx: GlobalContext) -> Callback<MouseEvent> {
    let mut view = None;
    // let query = yew_router::parse_query::<EmployeeQuery>();
    // if let Ok(q) = query {
    //     let EmployeeQuery { view: vw, .. } = q;
    //     view = vw
    // }
    Callback::from(move |_| {
        push_route_with_query(
            Route::Employees,
            EmployeeQuery {
                view: view.clone(),
                focus: Some(ctx.id),
            },
        )
        .unwrap();
    })
}

#[function_component(NavbarAcc)]
pub fn navbar_acc() -> Html {
    let ctx = use_context::<crate::GlobalContext>().expect("no ctx found");
    if ctx.logged {
        html! {
            <div class={classes!("col", "d-flex", "flex-row-reverse")}>
                <div class={classes!("dropdown")}>
                    <button class={classes!("btn", "btn-dark", "dropdown-toggle")} style="color: var(--bs-white);" type="button" id="loggedDropdown" data-bs-toggle="dropdown" aria-expanded="false"><i class={classes!("bi", "bi-person-circle", "p-2")}></i>{"Account"}</button>
                    <ul class={classes!("dropdown-menu", "dropdown-menu-dark", "dropdown-menu-end")} aria-labelledby="loggedDropdown">
                        <li><a class="dropdown-item" onclick={edit_self(ctx)}>{"Edit Account"}</a></li>
                        <li><a class="dropdown-item" href="#">{"Log Out"}</a></li>
                    </ul>
                </div>
            </div>
        }
    } else {
        html! {
            <div class={classes!("col", "d-flex", "flex-row-reverse")}>
                <div class={classes!("dropdown")}>
                    <button class={classes!("btn", "btn-dark")} type="button" onclick={Callback::from(|_| push_route(Route::Login))}>{"Log In"}</button>
                    <button type="button" class={classes!("btn", "btn-dark", "dropdown-toggle", "dropdown-toggle-split")} id="notLoggedDropdown" data-bs-toggle="dropdown" aria-expanded="false">
                        <span class="visually-hidden">{"Toggle Dropdown"}</span>
                    </button>
                    <ul class={classes!("dropdown-menu", "dropdown-menu-dark", "dropdown-menu-end")} aria-labelledby="notLoggedDropdown">
                        <li><Link<Route> classes={classes!("dropdown-item")} route={Route::Register}>{"Register"}</Link<Route>></li>
                    </ul>
                </div>
            </div>
        }
    }
}
