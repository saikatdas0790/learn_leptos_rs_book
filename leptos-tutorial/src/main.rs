use gloo_timers::future::TimeoutFuture;
use leptos::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (tab, set_tab) = create_signal(0);

    let user_data = create_resource(tab, |tab| async move {important_api_call(tab).await});

    view! {
        <div class="buttons">
            <button on:click=move |_| set_tab(0) class:selected=move || tab() == 0>
                "Tab A"
            </button>
            <button on:click=move |_| set_tab(1) class:selected=move || tab() == 1>
                "Tab B"
            </button>
            <button on:click=move |_| set_tab(2) class:selected=move || tab() == 2>
                "Tab C"
            </button>
            {move || if user_data.loading().get() { "Loading..." } else { "" }}

            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                <p>{move || user_data.get()}</p>
            </Transition>
        </div>
    }
}

async fn important_api_call(id: usize) -> String {
    TimeoutFuture::new(1_000).await;
    match id {
        0 => "Alice",
        1 => "Bob",
        2 => "Carol",
        _ => "User not found",
    }
    .to_string()
}


