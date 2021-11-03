use crate::components::{modal::EmployeeModal, table::TableView};
use crate::Route;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::push_route_with_query;

#[derive(Deserialize, PartialEq, Clone, Serialize, Debug)]
pub enum EmployeeView {
    Active,
    Pending,
    Disabled,
}

#[derive(Deserialize, Serialize)]
pub struct EmployeeQuery {
    // Table view
    pub view: Option<EmployeeView>,
    // Employee modal by id
    pub focus: Option<i32>,
}

#[derive(Properties, PartialEq)]
pub struct EmployeeProps {
    pub view: Option<EmployeeView>,
    pub focus: Option<i32>,
}

#[function_component(Employees)]
pub fn employees(props: &EmployeeProps) -> Html {
    let modal_ref = NodeRef::default();
    // Setup active tab states
    let all = use_state(|| "");
    let ial = all.clone();
    let active = use_state(|| "");
    let iac = active.clone();
    let pending = use_state(|| "");
    let ipe = pending.clone();
    let disabled = use_state(|| "");
    let idi = disabled.clone();
    let pr = (props.view.clone(), props.focus);
    use_effect_with_deps(
        move |p| {
            let (view, _focus) = p;
            ial.set("");
            iac.set("");
            ipe.set("");
            idi.set("");
            if let Some(v) = view {
                match v {
                    EmployeeView::Active => iac.set("active"),
                    EmployeeView::Pending => ipe.set("active"),
                    EmployeeView::Disabled => idi.set("active"),
                }
            } else {
                ial.set("active")
            };
            // TODO: Open modal here
            // if let Some(_id) = focus {}
            || {}
        },
        pr,
    );
    html! {
        <>
            <div>
                <h1>{"Blorebank Employees"}</h1>
            </div>
            <ul class="nav nav-tabs">
                <li class="nav-item">
                    <a class={classes!("nav-link", *all)} onclick={Callback::from(move |_| {push_route_with_query(Route::Employees, EmployeeQuery { view: None, focus: None}).unwrap();})}>{"All"}</a>
                </li>
                <li class="nav-item">
                    <a class={classes!("nav-link", *active)} onclick={Callback::from(move |_| {push_route_with_query(Route::Employees, EmployeeQuery { view: Some(EmployeeView::Active), focus: None}).unwrap();})}>{"Active"}</a>
                </li>
                <li class="nav-item">
                    <a class={classes!("nav-link", *pending)} onclick={Callback::from(move |_| {push_route_with_query(Route::Employees, EmployeeQuery { view: Some(EmployeeView::Pending), focus: None}).unwrap();})}>{"Pending"}</a>
                </li>
                <li class="nav-item">
                    <a class={classes!("nav-link", *disabled)} onclick={Callback::from(move |_| {push_route_with_query(Route::Employees, EmployeeQuery { view: Some(EmployeeView::Disabled), focus: None}).unwrap();})}>{"Disabled"}</a>
                </li>
            </ul>
            <TableView />
            <button type="button" class="btn btn-primary" data-bs-toggle="modal" data-bs-target="#employeeModal">
                {"Launch demo modal"}
            </button>
            /*<button type="button" class="btn btn-primary" onclick={Callback::from(move |_| {crate::js_bind::modal::get_modal_by_id("employeeModal").toggle();})}>
                {"Launch demo2 modal"}
            </button>*/
            <div class="modal fade" id="employeeModal" tabindex="-1" aria-labelledby="employeeModalLabel" aria-hidden="true" style="display: none;" ref={modal_ref.clone()}>
                <EmployeeModal />
            </div>
        </>
    }
}
