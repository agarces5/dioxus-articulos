use dioxus::prelude::*;

use crate::domain::articulo::Articulo;

#[component]
pub fn ArticuloCard(cx: Scope, articulo: Articulo) -> Element {
    let id = articulo.id();
    let nombre = articulo.nombre();
    let familia = articulo.familia();
    let detalles = articulo.detalles();
    cx.render(rsx! {
        tr {
            class: "bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600",
            td {
                class: "w-4 p-4",
                div {
                    class: "flex items-center",
                    input {
                        id: "checkbox-table-search-1",
                        r#type: "checkbox",
                        class: "w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600",
                    }
                    label {
                        r#for: "checkbox-table-search-1",
                        class: "sr-only",
                        "checkbox"
                    }
                }
            }
            td {
                class: "px-6 py-4",
                "{id}"
            }            
            th {
                scope: "row",
                class: "px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white",
                "{nombre}"
            }
            td {
                class: "px-6 py-4",
                "{familia}"
            }            
            td {
                class: "px-6 py-4",
                "{detalles:?}"
            }
            td {
                class: "px-6 py-4",
                a {
                    href: "#",
                    class: "font-medium text-blue-600 dark:text-blue-500 hover:underline",
                    "Edit"
                }
            }
        }
    })
}
