use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ArticuloDTO {
    pub articulo: f64,
    pub nombre: String,
    pub familia: String,
    pub cajtpv: Option<String>,
    pub tipotarifa: Option<String>,
    pub precio: Option<f64>,
}
use crate::domain::articulo::Articulo;
pub trait ArticuloDTOToArticulo<T> {
    fn to_articulo(self) -> Vec<Articulo>;
}

impl ArticuloDTOToArticulo<Vec<ArticuloDTO>> for Vec<ArticuloDTO> {
    fn to_articulo(self) -> Vec<Articulo> {
        use crate::domain::{detalle::Detalle, precio::Precio};
        use std::collections::HashMap;
        // Creamos un mapa para almacenar los datos en el nuevo formato
        let mut map_de_articulos: HashMap<u32, Articulo> = HashMap::new();

        // Recorremos cada elemento de la lista que nos llega
        for articulo_dto in self {
            let id = articulo_dto.articulo as u32;
            let nombre = articulo_dto.nombre.clone();
            let familia = articulo_dto.familia.clone();
            let cajtpv = articulo_dto.cajtpv.clone().unwrap_or_default();
            let tipotarifa = articulo_dto.tipotarifa.clone();
            let precio = articulo_dto.precio;

            // Para cada articulo, verificamos si ya existe y si no lo creamos
            let articulo = map_de_articulos
                .entry(id)
                .or_insert_with(|| Articulo::new(id, &nombre, &familia, &[]));

            // Verificamos si ya hay una cajtpv
            if let Some(details) = articulo
                .detalles_mut()
                .iter_mut()
                .find(|detail| detail.cajtpv() == cajtpv)
            {
                // Agregamos un nuevo precio y tarifa
                if let (Some(tipotarifa), Some(precio)) = (tipotarifa, precio) {
                    let precios = details.precios_mut();
                    precios.push(Precio::new(tipotarifa, precio / 10000.0))
                }
            } else if let (Some(tipotarifa), Some(precio)) = (tipotarifa, precio) {
                // Creamos un nuevo detalle
                let new_detail =
                    Detalle::new(&cajtpv, &[Precio::new(tipotarifa, precio / 10000.0)]);
                // Agregamos el nuevo detalle al articulo
                articulo.detalles_mut().push(new_detail);
            }
        }
        let mut sorted_list: Vec<Articulo> = map_de_articulos.into_values().collect();
        sorted_list.sort_by_key(|clave| clave.id());
        sorted_list
    }
}
