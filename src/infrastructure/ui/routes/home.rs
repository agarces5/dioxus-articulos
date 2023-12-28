use dioxus::prelude::*;

use crate::infrastructure::ui::components::{
    articulo_list::ArticuloList, modal::Modal, navbar::Navbar,
};

#[component]
pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        Navbar {}
        ArticuloList {}
        Modal {}
    })
}
