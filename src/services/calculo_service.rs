use crate::config::Settings;
use crate::models::DesgloseCosto; 

pub struct CalculoService;

impl CalculoService {
    /// Calcular el costo total de un viaje
    pub fn calcular_costo_viaje(
        distancia_km: f64,
        duracion_minutos: i32,
        settings: &Settings,
    ) -> DesgloseCosto {
        // 1. Tarifa base
        let tarifa_base = settings.tarifa_base;
        
        // 2. Costo por distancia
        let costo_por_km = settings.costo_por_km;
        let costo_distancia = distancia_km * costo_por_km;
        
        // 3. Costo por tiempo
        let costo_por_minuto = settings.costo_por_minuto;
        let costo_tiempo = duracion_minutos as f64 * costo_por_minuto;
        
        // 4. Subtotal
        let subtotal = tarifa_base + costo_distancia + costo_tiempo;
        
        // 5. Impuestos (0% por ahora)
        let impuestos = 0.0;
        
        // 6. Descuentos (0 por ahora)
        let descuentos = 0.0;
        
        // 7. Total final
        let total_final = subtotal + impuestos - descuentos;
        
        tracing::info!(
            "💰 Cálculo de costo: Base={:.2} + Distancia({:.2}km x {:.2})={:.2} + Tiempo({}min x {:.2})={:.2} = TOTAL: {:.2} Bs",
            tarifa_base,
            distancia_km,
            costo_por_km,
            costo_distancia,
            duracion_minutos,
            costo_por_minuto,
            costo_tiempo,
            total_final
        );
        
        DesgloseCosto {
            tarifa_base,
            distancia_km,
            costo_por_km,
            duracion_minutos,
            costo_por_minuto,
            subtotal,
            impuestos,
            descuentos,
            total_final,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calcular_costo_viaje() {
        let settings = Settings {
            tarifa_base: 10.0,
            costo_por_km: 2.0,
            costo_por_minuto: 0.5,
            ..Default::default()
        };
        
        let desglose = CalculoService::calcular_costo_viaje(5.0, 15, &settings);
        
        assert_eq!(desglose.tarifa_base, 10.0);
        assert_eq!(desglose.costo_por_km, 2.0);
        assert_eq!(desglose.duracion_minutos, 15);
        assert_eq!(desglose.subtotal, 27.5); // 10 + (5*2) + (15*0.5)
        assert_eq!(desglose.total_final, 27.5);
    }
}