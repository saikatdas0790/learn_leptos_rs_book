use leptos::prelude::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    leptos::mount::mount_to_body(|| {
        view! {
            <p>"Hello, world!"</p>
        }
    });
}
