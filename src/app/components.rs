use leptos::*;
use leptos_router::{AProps, A};

#[component]
pub fn ExpandingJumbotron(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="md:rounded-lg bg-[#845533] py-6 px-[10%] md:w-2/3 md:mx-auto
                    md:mt-5 text-white min-h-screen md:min-h-fit">
            {children(cx)}
        </div>
    }
}

#[component]
pub fn LinkButton(cx: Scope, href: &'static str, children: Children) -> impl IntoView {
    view! { cx,
        <A href=href class="bg-[#ffac44] text-black px-3 py-2.5 mx-auto mt-3
                            mb-6 rounded-md w-32 block text-center
                            hover:bg-orange-400">
            {children(cx)}
        </A>
    }
}

#[component]
pub fn SiteHeader(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <h1 class="text-center text-2xl font-bold mb-3">
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
