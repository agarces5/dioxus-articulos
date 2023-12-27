use dioxus::prelude::*;

use crate::domain::articulo::Articulo;

// #[derive(Props)]
// struct ArticuloCardProps<'a> {
//     articulo: &'a Articulo,
// }

// #[component]
// pub fn ArticuloCard<'a>(cx: Scope<'a, ArticuloCardProps<'a>>) -> Element {
//     cx.render(rsx! {
//         h2 { "ArticuloCard" }
//     })
// }
#[component]
pub fn ArticuloCard(cx: Scope, articulo: Articulo) -> Element {
    let id = articulo.id();
    let nombre = articulo.nombre();
    let familia = articulo.familia();
    let detalles = articulo.detalles();
    cx.render(rsx! {
        h2 { "ArticuloCard" }
        h3 {
            "ID: {id}"
        }
        h3 {
            "NOMBRE: {nombre}"
        }
        h3 {
            "FAMILIA: {familia}"
        }
        h3 {
            "DETALLES: {detalles:?}"
        }
    })
}
