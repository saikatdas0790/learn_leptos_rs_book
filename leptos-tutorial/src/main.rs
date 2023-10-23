use gloo_timers::future::TimeoutFuture;
use leptos::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (name, set_name) = create_signal("Bill".to_string());

    let async_data = create_resource(name, |name| async move { important_api_call(name).await });

    view! {
        <input on:input=move |ev| {set_name(event_target_value(&ev))} prop:value=name/>
        <p><code>"name: "</code>{name}</p>
        <Suspense fallback=move || {view! { <p>"Loading..."</p> }}>
            <p>"Your shouting name is "</p>
            {move || async_data.get() }
        </Suspense>
    }
}

async fn important_api_call(name: String) -> String {
    TimeoutFuture::new(1_000).await;
    name.to_ascii_uppercase()
}
