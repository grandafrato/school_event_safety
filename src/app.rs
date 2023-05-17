mod components;

use crate::event::Event;
use components::*;
use leptos::*;
use leptos_meta::{Body, Title};
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (_event, set_event) = create_signal(cx, Event::default());
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
                </Routes>
            </Router>
            <footer class="flex justify-between w-full px-2 pb-0.5 mt-auto fixed
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
        if event_name == "" {
            String::from("- Set Event Name")
        } else {
            format!("- Event: {}", event_name)
        }
    };
    view! { cx,
        <Title text=title/>
        <ExpandingJumbotron>
            <Show
                when=move || event_name.get() != ""
                fallback=move |cx| view! { cx,
                    <InitializingForm set_event_name=set_event_name/>
                }
            >
                <SiteHeader>
                    {format!("Planning Event: {}", event_name.get())}
                </SiteHeader>
            </Show>
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
