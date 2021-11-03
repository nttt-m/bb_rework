use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <p>{"404 Page Not Found!"}</p>
    }
}
