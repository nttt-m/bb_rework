use yew::prelude::*;

#[function_component(TableView)]
pub fn table() -> Html {
    html! {
        <>
            <table class={classes!("table", "table-dark")}>
                <thead>
                    <tr>
                        <th scope="col">{"ID"}</th>
                        <th scope="col">{"Name"}</th>
                        <th scope="col">{"Position"}</th>
                    </tr>
                </thead>
            </table>
        </>
    }
}
