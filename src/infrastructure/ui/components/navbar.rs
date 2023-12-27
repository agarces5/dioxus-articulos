use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::infrastructure::ui::routes::Route;

#[component]
pub fn Navbar(cx: Scope) -> Element {
    cx.render(rsx! {
        nav { class: "nav",
            div { class: "card",
                Link { to: Route::Home {}, "Go to Home" }
            }
            div { class: "card",
                Link { to: Route::Login {}, "Go to Login" }
            }
        }
    })
}
