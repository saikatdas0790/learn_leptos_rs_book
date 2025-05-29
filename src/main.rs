use leptos::prelude::*;

fn main() {
    let _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;
    let html = "<p>This HTML will be injected.</p>";

    view! {
        <button
            on:click=move |_| {
                *set_count.write() += 10;
            }
            style="position: absolute"
            style:left=move || { format!("{}px", count.get() + 100) }
            style:background-color=move || format!("rgb({}, {}, 100)", count.get(), 100)
            style:max-width="400px"
            style=("--columns", move || count.get().to_string())
        >
            "Click me: "
            {count}
        </button>

        <br />
        <progress max="50" value=double_count></progress>
        <p>"Double count: " {double_count}</p>
        <div inner_html=html />
    }
}
