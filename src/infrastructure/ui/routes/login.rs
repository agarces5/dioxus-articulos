use dioxus::prelude::*;

use crate::infrastructure::ui::components::navbar::Navbar;

#[component]
pub fn Login(cx: Scope) -> Element {
    cx.render(rsx! {
        Navbar {}
        h1 { "Login" }
        form {
            // prevent_default: "onsubmit",
            fieldset { input { r#type: "text", placeholder: "Usuario" } }
            fieldset { input { r#type: "password", placeholder: "Contraseña" } }
            button {
                r#type: "submit",
                disabled: false,
                onsubmit: move |event| { log::info!("Clicked! Event: {event:?}") },
                "Entrar"
            }
        }
    })
}
