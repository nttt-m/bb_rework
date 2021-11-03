use crate::components::auth_login::Login;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AuthProps {
    pub login: bool,
}

#[function_component(Auth)]
pub fn auth(props: &AuthProps) -> Html {
    if props.login {
        html! {
            <Login />
        }
    } else {
        html! {
            <p>{"Register"}</p>
        }
    }
}
