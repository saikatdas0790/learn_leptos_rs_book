use leptos::prelude::*;

fn main() {
    let _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <button on:click=move |_| {
            *set_count.write() += 1;
        }>"Click me: " {count}</button>
        <p>"Double count: " {move || count.get() * 2}</p>
        <p>
            <strong>"Reactive: "</strong>
            {move || count.get()}
        </p>
        <p>
            <strong>"Reactive shorthand: "</strong>
            {count}
        </p>
        <p>
            <strong>"Not reactive: "</strong>
            {count.get()}
        </p>
    }
}
