use leptos::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <main class="bg-orange-200 flex flex-col min-h-screen">
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
            <footer class="fixed mt-auto bottom-0 left-0 opacity-75 text-xs">
                <p>"Copyright © 2023 Lachlan Wilger."</p>
            </footer>
        </main>
    }
}

#[component]
fn IndexView(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="md:rounded-lg bg-[#845533] py-6 px-[10%] md:w-2/3 md:mx-auto
    r                   md:mt-5 text-white min-h-screen md:min-h-fit">
            <h1 class="text-center text-2xl font-bold mb-3">
                "Building an Event, Safely"
            </h1>
            <A href="/game" class="bg-[#ffac44] text-black px-3 py-2.5 mx-auto
                                   mt-3 mb-6 rounded-md w-32 block text-center
                                   hover:bg-orange-400">
                "Build Event"
            </A>
            <article class="prose prose-invert">
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
            </article>
        </div>
    }
}

#[component]
fn GameView(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"Game, it is."</h1>
    }
}
