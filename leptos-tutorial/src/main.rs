use leptos::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (x, set_x) = create_signal(0);
    let (y, set_y) = create_signal(0);
    let double_count = move || count() * 2;

    view! {
        <div
            style="position:absolute"
            style:left=move || format!("{}px", x() + 100)
            style:top=move || format!("{}px", y() + 100)
            style:background-color=move || format!("rgb({}, {}, 100)", x(), y())
            style=("--columns", x)
        >
            "Moves when coordinates change"
        </div>

        <progress max="50" value=double_count></progress>
        <p>"Double Count:" {double_count}</p>

        <button on:click=move |_| set_count.update(|n| *n += 1) class:red=move || count() % 2 == 1>
            "Click me: "
            {count}
        </button>
    }
}
