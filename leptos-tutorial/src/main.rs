use leptos::{html::Input, *};
use web_sys::SubmitEvent;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <h2>"Controlled Component"</h2>
        <ControlledComponent/>
        <h2>"Uncontrolled Component"</h2>
        <UncontrolledComponent/>
    }
}

#[component]
fn ControlledComponent() -> impl IntoView {
    let (name, set_name) = create_signal("Controlled".to_string());

    view! { <input type="text" on:input=move |ev| set_name(event_target_value(&ev)) prop:value=name/> }
}

#[component]
fn UncontrolledComponent() -> impl IntoView {
    let (name, set_name) = create_signal("Uncontrolled".to_string());

    // we'll use a NodeRef to store a reference to the input element
    // this will be filled when the element is created
    let input_element: NodeRef<Input> = create_node_ref();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value = input_element().expect("<input> element not found").value();
        set_name(value);
    };

    view! {
        <form on:submit=on_submit>
            <input type="text" value=name node_ref=input_element/>
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
    }
}
