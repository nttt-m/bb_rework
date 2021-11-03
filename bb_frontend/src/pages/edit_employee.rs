use web_sys::{Element, HtmlFormElement};
use yew::prelude::*;

fn submit(event: FocusEvent, node_ref: NodeRef) {
    let form = event.target();
    if form.is_some() {
        let form = node_ref.cast::<HtmlFormElement>().unwrap();
        if !form.check_validity() {
            event.prevent_default();
            event.stop_propagation();
        };
        node_ref
            .cast::<Element>()
            .unwrap()
            .class_list()
            .add_1("was-validated")
            .unwrap();
    }
}

#[function_component(EditEmployee)]
pub fn edit_employee() -> Html {
    let node_ref = NodeRef::default();
    html! {
        <form class={classes!("needs-validation")} novalidate=true onsubmit={Callback::from(move |e| submit(e, node_ref.clone()))} ref={node_ref.clone()}>
            <div class={classes!("row")}>
                <div class={classes!("col")}>
                    <label for="firstName" class={classes!("form-label")}>{"First Name"}</label>
                    <input type="text" class={classes!("form-control")} id="firstName" required=true value="" />
                    <div class="invalid-feedback">
                        {"Employee must have a first name."}
                    </div>
                </div>
                <div class={classes!("col")}>
                    <label for="lastName" class={classes!("form-label")}>{"Last Name"}</label>
                    <input type="text" class={classes!("form-control")} id="lastName" required=true value="" />
                    <div class="invalid-feedback">
                        {"Employee must have a last name."}
                    </div>
                </div>
            </div>
            <div class={classes!("row")}>
                <div class={classes!("col")}>
                    <label for="dob" class={classes!("form-label")}>{"Date of Birth"}</label>
                    <input type="date" class={classes!("form-control")} id="dob" required=true value="" />
                    <div class="invalid-feedback">
                        {"Employee must have a date of birth."}
                    </div>
                </div>
                <div class={classes!("col")}>
                    <label for="position" class={classes!("form-label")}>{"Position"}</label>
                    <input type="text" class={classes!("form-control")} id="position" required=true value="" />
                    <div class="invalid-feedback">
                        {"Employee must have a position."}
                    </div>
                </div>
            </div>
            <div>
                <button class="btn btn-primary" type="submit">{"Submit form"}</button>
            </div>
        </form>
    }
}
