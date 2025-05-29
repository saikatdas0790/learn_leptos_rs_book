use leptos::prelude::*;

fn main() {
    let _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| {
        view! {
            <p>"Hello, world!"</p>
        }
    });
}
