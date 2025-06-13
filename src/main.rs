use leptos::prelude::*;

fn main() {
    let _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let counter = RwSignal::new(0);
    let derived_counter = move || *counter.read() * 2;

    view! {
        <button on:click=move |_| *counter.write() += 1>{counter}</button>
        <ProgressBar progress=counter />
        <ProgressBar progress=derived_counter />
    }
}

#[component]
fn ProgressBar(progress: impl Fn() -> u32 + Send + 'static) -> impl IntoView {
    view! { <progress max="50" value=progress></progress> }
}
