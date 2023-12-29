use dioxus::prelude::*;

use crate::infrastructure::ui::{
    components::{articulo_card::ArticuloCard, search_bar::SearchBar},
    hooks::use_articulo::use_articulo,
};

#[component]
pub fn ArticuloList(cx: Scope) -> Element {
    let articulos = use_articulo(cx);
    cx.render(rsx! {
        SearchBar {}
        table {
            class: "w-full mx-auto text-sm text-center rtl:text-center text-gray-400",
            thead {
                class: "text-xs text-gray-400 uppercase bg-gray-700",
                tr {
                    th {
                        scope: "col",
                        class: "px-6 py-3",
                        "id"
                    }
                    th {
                        scope: "col",
                        class: "px-6 py-3",
                        "nombre"
                    }
                    th {
                        scope: "col",
                        class: "px-6 py-3",
                        "familia"
                    }
                    th {
                        scope: "col",
                        class: "px-6 py-3",
                        "detalles"
                    }
                    th {
                        scope: "col",
                        class: "px-6 py-3",
                        "acciones"
                    }
                }
            }
            tbody {
                articulos.iter().map(|art| rsx! {ArticuloCard { articulo: art }})
            }
        }
    })
}
