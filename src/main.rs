use std::marker::PhantomData;

use leptos::prelude::*;

fn main() {
    let _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || *count.read() * 2;

    view! {
        <button on:click=move |_| {
            *set_count.write() += 1;
        }>"Click me: "</button>
        <ProgressBar />
    }
}

#[component]
fn ProgressBar(
    #[prop(optional)] progress: Option<Box<dyn Fn() -> i32 + Send + Sync>>,
) -> impl IntoView {
    progress.map(|progress| {
        view! { <progress max=100 value=progress></progress> }
    })
}
