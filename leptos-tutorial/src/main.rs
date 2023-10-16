use leptos::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let is_odd = move || value() & 1 == 1;
    let odd_text = move || if is_odd() { Some("How odd!") } else { None };

    view! {
        <h1>Control Flow</h1>
        <button on:click=move |_| set_value.update(|n| *n += 1)>"+1"</button>
        <p>"Value is: " {value}</p>

        <hr/>
    }
}
