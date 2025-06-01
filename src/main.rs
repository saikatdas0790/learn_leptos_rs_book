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
        <ProgressBar progress=count />
        <ProgressBar progress=double_count />
    }
}

/// Shows progress towards a goal.
#[component]
fn ProgressBar(
    /// The maximum value of the progress bar.
    #[prop(default = 100)]
    max: u16,
    /// How much progress should be displayed.
    progress: impl Fn() -> i32 + Send + 'static,
) -> impl IntoView {
    view! { <progress max=max value=progress style:display="block" style:margin-top="1rem"></progress> }
}
