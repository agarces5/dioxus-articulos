use serde::{Deserialize, Serialize};

use crate::domain::precio::Precio;

#[derive(PartialEq, Debug, Default, Serialize, Clone, Deserialize)]
/// Lista de Precios para cada Caja
/// cajtpv: String
/// precios: Vec<Precio>, importe asociado a cada tarifa
pub struct Detalle {
    cajtpv: String,
    precios: Vec<Precio>,
}

impl Detalle {
    /// Creates a new [`Detalles`].
    pub fn new(cajtpv: &str, precios: &[Precio]) -> Self {
        Self {
            cajtpv: cajtpv.to_owned(),
            precios: precios.to_owned(),
        }
    }
    /// Returns a reference to the cajtpv of this [`Detalles`].
    pub fn cajtpv(&self) -> &str {
        &self.cajtpv
    }
    /// Sets the cajtpv of this [`Detalle`].
    pub fn set_cajtpv(&mut self, cajtpv: &str) {
        self.cajtpv = cajtpv.to_owned()
    }
    /// Returns a reference to the precios of this [`Detalles`].
    pub fn precios(&self) -> &[Precio] {
        &self.precios
    }
    pub fn precios_mut(&mut self) -> &mut Vec<Precio> {
        &mut self.precios
    }
    /// Sets the precios of this [`Detalle`].
    pub fn set_precios(&mut self, precios: &[Precio]) {
        self.precios = precios.to_owned()
    }
}
