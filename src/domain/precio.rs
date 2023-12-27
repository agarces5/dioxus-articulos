use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
/// Importe asociado a cada tarifa
/// tipotarifa: String
/// precio: f64
pub struct Precio {
    tipotarifa: String,
    precio: f64,
}

impl Precio {
    /// Creates a new [`Precio`].
    pub fn new(tipotarifa: String, precio: f64) -> Self {
        Self { tipotarifa, precio }
    }
    /// Returns a reference to the tipotarifa of this [`Precio`].
    pub fn tipotarifa(&self) -> &str {
        &self.tipotarifa
    }
    /// Sets the tipotarifa of this [`Precio`].
    pub fn set_tipotarifa(&mut self, tipotarifa: &str) {
        self.tipotarifa = tipotarifa.to_owned()
    }
    /// Returns the precio of this [`Precio`].
    pub fn precio(&self) -> f64 {
        self.precio
    }
    /// Sets the precio of this [`Precio`].
    pub fn set_precio(&mut self, precio: f64) {
        self.precio = precio
    }
}
