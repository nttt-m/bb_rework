use yew::{classes, function_component, html, Callback};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let home = Callback::from(|_| yew_router::push_route(crate::Route::Home));
    html! {
        <nav class={classes!("bg-gray-800")}>
            <div class={classes!("h-16", "flex", "flex-row", "flex-shrink-0", "justify-between")}>
                <div class={classes!("justify-right", "flex", "flex-row")}>
                    <img src={"/static/logo.svg"} alt="Blorebank Logo" class={classes!("h-16", "w-auto", "hover:cursor-pointer")} style="filter: invert(100%) sepia(0%) saturate(0%) hue-rotate(23deg) brightness(105%) contrast(102%);" onclick={&home} />
                    <button>{"Home"}</button>
                    <button>{"View Employees"}</button>
                </div>
                <div class={classes!("justify-left")}>
                    <img src={"/static/avatar.svg"} alt="Profile Avatar" class={classes!("h-16", "w-auto", "hover:cursor-pointer")} style="filter: invert(100%) sepia(0%) saturate(0%) hue-rotate(23deg) brightness(105%) contrast(102%);" onclick={&home} />
                </div>
                // <button>{"Profile"}</button>
                // <button>{"Login"}</button>
                // <button>{"Register"}</button>
                // <button>{"Logout"}</button>
            </div>
        </nav>
    }
}
