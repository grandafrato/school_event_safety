mod components;

use crate::event::{calculate_score, Event};
use components::*;
use leptos::*;
use leptos_meta::{Body, Title};
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (event, set_event) = create_signal(cx, Event::default());
    provide_context(cx, event);
    provide_context(cx, set_event);

    view! { cx,
        <Title formatter=|text| format!("Planning for Safety {}", text)/>
        <Body class="bg-orange-200"/>
        <main>
            <Router fallback=move |cx| view! { cx, <NotFound/> }.into_view(cx)>
                <Routes>
                    <Route path="/" view=move |cx| {
                        view! { cx, <IndexView/> }
                    }/>
                    <Route path="/game" view=move |cx| {
                        view! { cx, <GameView/> }
                    }/>
                    <Route path="/result" view=move |cx| {
                        view! { cx, <ResultView/> }
                    }/>
                </Routes>
            </Router>
            <footer class="flex justify-between w-full px-2 pb-0.5 fixed
                           bottom-0 left-0 opacity-75 text-xs">
                <p>"Copyright © 2023 Lachlan Wilger."</p>
                <a class="underline hover:decoration-dashed"
                    href="https://github.com/grandafrato/school_event_safety">
                    "Source on Github"
                </a>
            </footer>
        </main>
    }
}

#[component]
fn IndexView(cx: Scope) -> impl IntoView {
    // Clears out the state if you return to the index page from the score
    // tabulation.
    use_context::<WriteSignal<Event>>(cx)
        .expect("event to be in scope")
        .set(Event::default());

    view! { cx,
        <Title text=""/>
        <ExpandingJumbotron>
            <SiteHeader>"Building an Event, Safely"</SiteHeader>
            <Article>
                <h2>"How The Simulation Works"</h2>
                <h3>"Goal"</h3>
                <p>"Your goal is to build an event for your school that is both
                    fun and safe, keeping in mind what you've learned about school
                    safety."</p>
                <h3>"Task Instructions"</h3>
                <p>"You are responsible for a new  event at school, but so far
                    it's pretty bland. In order to make it more exciting, you
                    have decided to more activities or \"event features\" to
                    the event. However, you must tow the line between a fun
                    event, or safety violation extravaganza."</p>
                <p>"You need to add features to the event to increase the fun
                    level, but each feature comes with its own risks. To
                    counteract the risk, you need to add various countermeasures
                    to ensure a safe, fun time for everyone."</p>
            </Article>
            <LinkButton href="/game">"Start Event"</LinkButton>
        </ExpandingJumbotron>
    }
}

#[component]
fn GameView(cx: Scope) -> impl IntoView {
    let (event_name, set_event_name) = create_signal(cx, String::from(""));
    let title = move || {
        let event_name = event_name.get();
        if event_name.is_empty() {
            String::from("- Set Event Name")
        } else {
            format!("- Event: {}", event_name)
        }
    };

    let event = use_context::<ReadSignal<Event>>(cx).expect("There should be an event in scope.");

    let (event_feature_index, set_event_feature_index) = create_signal(cx, 0);

    let event_feature = move || event.get().select_feature(event_feature_index.get());

    view! { cx,
        <Title text=title/>
        <ExpandingJumbotron>
            <Show
                when=move || event_name.get() != ""
                fallback=move |cx| view! { cx,
                    <InitializingForm set_event_name=set_event_name/>
                }
            >
                {if let Some(event_feature) = event_feature() {
                    view! { cx,
                        <SiteHeader>
                            "Planning Event: " {event_name.get()}
                        </SiteHeader>
                        <EventFeatureInformation event_feature=event_feature.clone()/>
                        <SelectCounterFeature feature_index={event_feature_index.get()} event_feature=event_feature/>
                        <hr class="my-3 opacity-50 border-dashed"/>
                        <NextStepButton set_index=set_event_feature_index/>
                    }.into_view(cx)
                } else {
                    view! { cx, <Redirect path="/result"/>}.into_view(cx)
                }}
            </Show>
        </ExpandingJumbotron>
    }
}

#[component]
fn ResultView(cx: Scope) -> impl IntoView {
    let event = use_context::<ReadSignal<Event>>(cx).expect("event to be in scope");

    view! { cx,
        <ExpandingJumbotron>
            <SiteHeader>"Score: " {calculate_score(&event.get())}</SiteHeader>
            <LinkButton href="/">"Go Home"</LinkButton>
        </ExpandingJumbotron>
    }
}

#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="- Page Not Found"/>
        <ExpandingJumbotron>
            <SiteHeader>"Page Not Found"</SiteHeader>
            <LinkButton href="/">"Go Home"</LinkButton>
        </ExpandingJumbotron>
    }
}
