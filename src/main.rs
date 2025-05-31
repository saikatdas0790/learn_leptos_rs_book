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
        // <button on:click=move |_| {
        // *set_count.write() += 1;
        // }>"Click me: "</button>
        // <ProgressBar progress=count />
        // <ProgressBar progress=double_count />
        <span style:display="block">
            <SizeOf<usize> />
        </span>
        <span style:display="block">
            <SizeOf<String> />
        </span>
    }
}

#[component]
fn ProgressBar(
    #[prop(default = 100)] max: u16,
    progress: impl Fn() -> i32 + Send + 'static,
) -> impl IntoView {
    view! { <progress max=max value=progress style:display="block" style:margin-top="1rem"></progress> }
}

#[component]
fn SizeOf<T: Sized>(#[prop(optional)] _ty: PhantomData<T>) -> impl IntoView {
    std::mem::size_of::<T>()
}
