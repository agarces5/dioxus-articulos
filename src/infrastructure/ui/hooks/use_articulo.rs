use dioxus::prelude::*;

use crate::domain::articulo::Articulo;
use crate::infrastructure::models::articulo_dto::*;

pub fn use_articulo(cx: &ScopeState) -> &mut Vec<Articulo> {
    let articulos_dto = include_str!("../../../mocks/articulos_input.json");
    let articulos_dto: Vec<ArticuloDTO> =
        serde_json::from_str(articulos_dto).expect("Unable to parse articulos.json");
    let articulos: Vec<Articulo> = articulos_dto.to_articulo();
    cx.use_hook(|| articulos)
}
