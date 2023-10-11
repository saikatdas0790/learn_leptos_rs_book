use leptos::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;

    view! {
        <button on:click=move |_| set_count.update(|n| *n += 1)>"Click me: " {count}</button>
        <ProgressBar progress=count/>
        <ProgressBar progress=Signal::derive(double_count)/>
        <StaticList/>
        <DynamicList initial_length=5/>
    }
}

/// Shows progress toward a goal.
#[component]
fn ProgressBar(
    /// The maximum value of the progress bar.
    #[prop(default = 100)]
    max: u16,
    /// How much progress should be displayed.
    #[prop(into)]
    progress: Signal<i32>,
) -> impl IntoView {
    view! { <progress max=max value=progress></progress> }
}

#[component]
fn StaticList() -> impl IntoView {
    let length = 5;
    let counters = (1..=length).map(create_signal);

    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! {
                <li>
                    <button on:click=move |_| set_count.update(|n| *n += 1)>{count}</button>
                </li>
            }
        })
        .collect_view();

    view! { <ul>{counter_buttons}</ul> }
}

#[component]
fn DynamicList(initial_length: usize) -> impl IntoView {
    let mut next_counter_id = initial_length;

    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();

    let (counters, set_counters) = create_signal(initial_counters);

    let add_counter = move |_| {
        let sig = create_signal(next_counter_id + 1);
        set_counters.update(move |counters| counters.push((next_counter_id, sig)));
        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>"Add Counter"</button>
            <ul>
                <For
                    each=counters
                    key=|counter| counter.0
                    children=move |(id, (count, set_count))| {
                        view! {
                            <li>
                                <button on:click=move |_| {
                                    set_count.update(|n| *n += 1)
                                }>{count}</button>
                                <button on:click=move |_| {
                                    set_counters
                                        .update(|counters| {
                                            counters.retain(|(counter_id, _)| counter_id != &id)
                                        });
                                }>

                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />

            </ul>
        </div>
    }
}
