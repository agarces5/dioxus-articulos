use dioxus::prelude::*;

use crate::{
    domain::articulo::Articulo, infrastructure::ui::components::articulo_card::ArticuloCard,
};

#[component]
pub fn ArticuloList(cx: Scope) -> Element {
    let articulo = Articulo::new(1, "Fanta", "Refrescos", &[]);
    cx.render(rsx! {
        ArticuloCard { articulo: articulo }
    })
}
