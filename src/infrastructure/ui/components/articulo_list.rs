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
            class: "w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400",
            thead {
                class: "text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400",
                tr {
                    // th {
                    //     scope: "col",
                    //     class: "p-4",
                    //     div {
                    //         class: "flex items-center",
                    //         input {
                    //             id: "checkbox-all-search",
                    //             r#type: "checkbox",
                    //             class: "w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                    //         }
                    //         label {
                    //             r#for: "checkbox-all-search",
                    //             class: "sr-only",
                    //             "checkbox"
                    //         }
                    //     }
                    // }
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
