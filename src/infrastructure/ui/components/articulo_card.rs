use dioxus::prelude::*;

use crate::domain::articulo::Articulo;

#[component]
pub fn ArticuloCard<'a>(cx: Scope, articulo: &'a Articulo) -> Element {
    let id = articulo.id();
    let nombre = articulo.nombre();
    let familia = articulo.familia();
    let detalles = articulo.detalles();
    let cajas: Vec<&str> = detalles.iter().map(|detalle| detalle.cajtpv()).collect();
    cx.render(rsx! {
        tr {
            class: "odd:bg-gray-800 even:bg-gray-900 border hover:bg-gray-600",
            td {
                class: "border-x border-white px-6 py-4",
                "{id}"
            }            
            th {
                scope: "row",
                class: "border-x border-white px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white",
                "{nombre}"
            }
            td {
                class: "border-x border-white px-6 py-4",
                "{familia}"
            }            
            td {
                class: "border-x border-white px-0 py-0",
                table {
                    class: "w-full h-8 py-0",
                    tr {
                        class: "hover:bg-gray-600 py-0",
                        cajas.iter().map(|caja| rsx! {
                            td{
                                class: "border-x border-white px-6 py-4",
                                "{caja}"
                            }
                        } )
                    }
                }
            }
            td {
                class: "border-x border-white px-6 py-4",
                a {
                    href: "",
                    class: "font-medium text-blue-400 hover:underline",
                    "Edit"
                }
            }
        }
    })
}
