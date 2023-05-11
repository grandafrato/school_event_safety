use crate::event::Event;
use leptos::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <Routes>
                <Route path="/" view=move |cx| {
                    view! { cx, <IndexView/>}
                }/>
                <Route path="/game" view=move |cx| {
                    view! { cx, <GameView/>}
                }/>
            </Routes>
        </Router>
        <footer>
            <p>"Copyright (c) 2023 Lachlan Wilger."</p>
        </footer>
    }
}

#[component]
fn IndexView(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"Building an Event, Safely"</h1>
        <A href="/game">"Start Event"</A>
    }
}

#[component]
fn GameView(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"Game, it is."</h1>
    }
}
