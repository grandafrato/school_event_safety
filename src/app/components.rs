use crate::event::{CounterMeasure, Event, EventFeature};
use leptos::*;
use leptos::{ev::SubmitEvent, html::Input};
use leptos_router::*;

#[component]
pub fn ExpandingJumbotron(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="md:rounded-lg bg-[#845533] py-6 px-[10%] md:w-2/3 md:mx-auto
                    md:mt-5 text-white min-h-screen md:min-h-fit drop-shadow-lg">
            {children(cx)}
        </div>
    }
}

#[component]
pub fn LinkButton(cx: Scope, href: &'static str, children: Children) -> impl IntoView {
    view! { cx,
        <A href=href class="bg-[#ffac44] text-black px-3 py-2.5 mx-auto my-3
                            rounded-md w-32 block text-center
                            hover:bg-orange-400 drop-shadow-md
                            hover:drop-shadow-sm">
            {children(cx)}
        </A>
    }
}

#[component]
pub fn SiteHeader(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <h1 class="text-center capitalize text-2xl font-bold mb-6">
            {children(cx)}
        </h1>
    }
}

#[component]
pub fn Article(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <article class="prose prose-invert">
            {children(cx)}
        </article>
    }
}

#[component]
pub fn InitializingForm(cx: Scope, set_event_name: WriteSignal<String>) -> impl IntoView {
    let set_event =
        use_context::<WriteSignal<Event>>(cx).expect("There should be an event in scope.");
    let input_element: NodeRef<Input> = create_node_ref(cx);
    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading
        ev.prevent_default();

        let value = input_element.get().expect("<input> to exist").value();

        set_event_name.set(value.clone());
        set_event.update(|event| event.set_name(value));
    };

    view! { cx,
        <SiteHeader>"Set Event Name"</SiteHeader>
        <form class="bg-orange-400 flex rounded-lg p-3 justify-center space-x-4
                     max-w-md mx-auto"
            on:submit=on_submit>
            <input class="text-black rounded-md" type="text"
                node_ref=input_element/>
            <button class="bg-[#845533] rounded-md px-3 w-32 py-2.5
                           drop-shadow-md hover:drop-shadow-sm
                           hover:bg-orange-600"
                type="submit" value="Submit">
                "Enter"
            </button>
        </form>
    }
}

#[component]
pub fn EventFeatureInformation(cx: Scope, event_feature: EventFeature) -> impl IntoView {
    view! { cx,
        <div class="bg-orange-500 p-3 rounded-md text-black">
            <h2 class="text-2xl text-center">"Feature: " {event_feature.name}</h2>
            <h3 class="text-xl">"Added Counter Measures:"</h3>
            <ul class="divide-y divide-solid divide-current">
                <For each=move || event_feature.selected_counter_measures.clone()
                    key=|cm| cm.name view=move |cx, opt: CounterMeasure| {
                        view! { cx,
                            <li>{opt.name}</li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
pub fn SelectCounterFeature(
    cx: Scope,
    feature_index: usize,
    event_feature: EventFeature,
) -> impl IntoView {
    let event = use_context::<WriteSignal<Event>>(cx).expect("event to be in scope");

    let update_event = move |option| {
        event.update(|ev| ev.toggle_selected_counter_measure_option(feature_index, option));
    };

    view! { cx,
        <h2 class="text-2xl my-3 text-bold text-center">"Countermeasure Toggles"</h2>
        <div class="flex flex-row flex-wrap justify-center gap-5 my-3">
            <For each=move || event_feature.options.clone()
                key=|opt| opt.name view=move |cx, opt: CounterMeasure| {
                    view! { cx,
                        <button class="bg-[#ffac44] text-black px-3 py-2.5
                                           rounded-md w-32 block text-center
                                           hover:bg-orange-400 drop-shadow-md
                                           hover:drop-shadow-sm"
                            on:click=move |_| update_event(opt)>
                            {opt.name}
                        </button>
                    }
                }
            />
        </div>
    }
}

#[component]
pub fn NextStepButton(cx: Scope, set_index: WriteSignal<usize>) -> impl IntoView {
    let update_index = move |_| set_index.update(|index| *index += 1);

    view! { cx,
        <button class="bg-[#ffac44] text-black py-2.5 mx-auto rounded-md
                       w-32 block text-center hover:bg-orange-400 drop-shadow-md
                       hover:drop-shadow-sm flex flex-row justify-center"
            on:click=update_index>
            <p>"Next"</p>
        </button>
    }
}
