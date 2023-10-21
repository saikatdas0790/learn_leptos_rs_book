use leptos::*;
use web_sys::MouseEvent;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (items, set_items) = create_signal(vec![0, 1, 2]);

    let render_prop = move || {
        let len = move || items.with(Vec::len);
        view! { <p>"Length: " {len}</p> }
    };

    view! {
        <TakesChildern render_prop>
            <p>"Here's a child."</p>
            <p>"Here's another child."</p>
        </TakesChildern>
        <hr/>
        <WrapsChildren>
            <p>"Here's a child."</p>
            <p>"Here's another child."</p>
        </WrapsChildren>
    }
}

#[component]
pub fn TakesChildern<F, IV>(render_prop: F, children: Children) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <h1>
            <code>"<TakesChildren />"</code>
        </h1>
        <h2>"Render Prop"</h2>
        {render_prop()}
        <hr/>
        <h2>"Children"</h2>
        {children()}
    }
}

#[component]
pub fn WrapsChildren(children: Children) -> impl IntoView {
    let children = children()
        .nodes
        .into_iter()
        .map(|child| view! { <li>{child}</li> })
        .collect::<Vec<_>>();

    view! {
        <h1>
            <code>"<WrapsChildren/>"</code>
        </h1>
        <ul>{children}</ul>
    }
}
