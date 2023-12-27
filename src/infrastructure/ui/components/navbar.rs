use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::infrastructure::ui::routes::Route;

#[component]
pub fn Navbar(cx: Scope) -> Element {
    cx.render(rsx! {
        nav { class: "flex flex-row justify-between justify-self-center p-4 w-4/5",
            div { class: "card",
                Link { to: Route::Home {}, "Base de datos" }
            }
            div { class: "card",
                Link { to: Route::Login {}, "Login" }
            }
        }
    })
}
