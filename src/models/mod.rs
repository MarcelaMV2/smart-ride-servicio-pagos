pub mod pago;
pub mod factura;

pub use pago::{
    Pago,
    MetodoPago,
    EstadoPago,
    DesgloseCosto,
    CrearPagoRequest,
    ActualizarPagoRequest,
};

pub use factura::{
    Factura,
    DatosPasajero,
    DetalleViaje,
    EstadoFactura,
    CrearFacturaRequest,
    ActualizarFacturaRequest,
};
