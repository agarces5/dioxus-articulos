use dioxus::prelude::*;

#[component]
pub fn Modal(cx: Scope) -> Element {
    let modal_open = use_state(cx, || false);
    cx.render(rsx! {
        // button {
        //     r#type: "button",
        //     class: "text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800",
        //     onclick: move |_| modal_open.set(true),
        //     "Click me!"
        // }
        dialog {
            open: "{modal_open}",
            div {
                id: "modal",
                class: "modal",
                p {
                    class: "text-xl",
                    "Tu modal to reshulon",
                }
                button {
                    r#type: "button",
                    class: "focus:outline-none text-white bg-red-700 hover:bg-red-800 focus:ring-4 focus:ring-red-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-red-600 dark:hover:bg-red-700 dark:focus:ring-red-900",
                    onclick: move |_| modal_open.set(false),
                    "CLOSE!"
                }
            }
        }
    })
}
