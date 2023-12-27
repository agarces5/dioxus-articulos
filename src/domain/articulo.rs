use serde::{Deserialize, Serialize};

use crate::domain::detalle::Detalle;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
/// id: u32
/// nombre: String
/// familia: String
/// detalles: Vec<Detail>, lista de cajas y precios
pub struct Articulo {
    id: u32,
    nombre: String,
    familia: String,
    detalles: Vec<Detalle>,
}

impl Articulo {
    /**
    Creates a new [`Articulo`].
    */
    pub fn new(id: u32, nombre: &str, familia: &str, detalles: &[Detalle]) -> Self {
        Self {
            id,
            nombre: nombre.to_owned(),
            familia: familia.to_owned(),
            detalles: detalles.to_owned(),
        }
    }
    /// Returns the id of this [`Articulo`].
    pub fn id(&self) -> u32 {
        self.id
    }
    /// Sets the id of this [`Articulo`].
    pub fn set_id(&mut self, id: u32) {
        self.id = id
    }
    /// Returns a reference to the nombre of this [`Articulo`].
    pub fn nombre(&self) -> &str {
        &self.nombre
    }
    /// Sets the nombre of this [`Articulo`].
    pub fn set_nombre(&mut self, nombre: &str) {
        self.nombre = nombre.to_owned()
    }
    /// Returns a reference to the familia of this [`Articulo`].
    pub fn familia(&self) -> &str {
        &self.familia
    }
    /// Sets the familia of this [`Articulo`].
    pub fn set_familia(&mut self, familia: &str) {
        self.familia = familia.to_owned()
    }
    /// Returns a reference to the detalles of this [`Articulo`].
    pub fn detalles(&self) -> &[Detalle] {
        &self.detalles
    }
    /// Sets the detalles of this [`Articulo`].
    pub fn set_detalles(&mut self, detalles: &[Detalle]) {
        self.detalles = detalles.to_owned()
    }
}
