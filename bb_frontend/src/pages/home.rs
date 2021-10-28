use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct HomeProps;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = HomeProps;

    fn create(ctx: &Context<Self>) -> Self {
        Home
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"Blorebank Secure HRMS"}</h1>
            </div>
        }
    }
}
