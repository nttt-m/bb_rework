use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct EmployeeProps;

pub struct Employees;

impl Component for Employees {
    type Message = ();
    type Properties = EmployeeProps;

    fn create(ctx: &Context<Self>) -> Self {
        Employees
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"Blorebank Employees"}</h1>
            </div>
        }
    }
}
