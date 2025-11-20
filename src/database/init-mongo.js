// Cambiar a la base de datos
db = db.getSiblingDB('smart_ride_pagos');

// Crear usuario específico
db.createUser({
  user: 'usuario',
  pwd: 'usuario123',
  roles: [
    {
      role: 'readWrite',
      db: 'smart_ride_pagos'
    }
  ]
});

// Crear colecciones con validación
db.createCollection('pagos', {
  validator: {
    $jsonSchema: {
      bsonType: 'object',
      required: ['id_pago', 'id_viaje', 'id_pasajero', 'id_conductor', 'desglose_costo', 'metodo_pago', 'estado_pago'],
      properties: {
        id_pago: {
          bsonType: 'int',
          description: 'ID único del pago'
        },
        id_viaje: {
          bsonType: 'int',
          description: 'ID del viaje'
        },
        id_pasajero: {
          bsonType: 'int',
          description: 'ID del pasajero'
        },
        id_conductor: {
          bsonType: 'int',
          description: 'ID del conductor'
        },
        desglose_costo: {
          bsonType: 'object',
          required: ['tarifa_base', 'distancia_km', 'costo_por_km', 'duracion_minutos', 'subtotal', 'total_final'],
          properties: {
            tarifa_base: { bsonType: 'double' },
            distancia_km: { bsonType: 'double' },
            costo_por_km: { bsonType: 'double' },
            duracion_minutos: { bsonType: 'int' },
            costo_por_minuto: { bsonType: 'double' },
            subtotal: { bsonType: 'double' },
            impuestos: { bsonType: 'double' },
            descuentos: { bsonType: 'double' },
            total_final: { bsonType: 'double' }
          }
        },
        metodo_pago: {
          enum: ['efectivo', 'tarjeta', 'wallet'],
          description: 'Método de pago'
        },
        tipo_pago: {
          enum: ['real', 'simulado'],
          description: 'Tipo de pago'
        },
        estado_pago: {
          enum: ['pendiente', 'completado', 'fallido', 'reembolsado'],
          description: 'Estado del pago'
        }
      }
    }
  }
});

db.createCollection('facturas', {
  validator: {
    $jsonSchema: {
      bsonType: 'object',
      required: ['id_factura', 'id_pago', 'id_viaje', 'numero_factura', 'costo_total'],
      properties: {
        id_factura: { bsonType: 'int' },
        id_pago: { bsonType: 'int' },
        id_viaje: { bsonType: 'int' },
        id_pasajero: { bsonType: 'int' },
        id_conductor: { bsonType: 'int' },
        numero_factura: { bsonType: 'string' },
        costo_total: { bsonType: 'double' },
        estado_factura: {
          enum: ['emitida', 'anulada']
        }
      }
    }
  }
});

db.createCollection('transacciones_fallidas');

// Crear colección de contadores para IDs secuenciales
db.createCollection('counters');
db.counters.insertMany([
  { _id: 'pago_id', seq: 0 },
  { _id: 'factura_id', seq: 0 },
  { _id: 'transaccion_id', seq: 0 }
]);

// Crear índices
db.pagos.createIndex({ "id_pago": 1 }, { unique: true });
db.pagos.createIndex({ "id_viaje": 1 }, { unique: true });
db.pagos.createIndex({ "id_pasajero": 1 });
db.pagos.createIndex({ "id_conductor": 1 });
db.pagos.createIndex({ "estado_pago": 1 });
db.pagos.createIndex({ "fecha_pago": -1 });

db.facturas.createIndex({ "id_factura": 1 }, { unique: true });
db.facturas.createIndex({ "id_pago": 1 }, { unique: true });
db.facturas.createIndex({ "numero_factura": 1 }, { unique: true });
db.facturas.createIndex({ "id_pasajero": 1 });
db.facturas.createIndex({ "fecha_emision": -1 });

db.transacciones_fallidas.createIndex({ "id_viaje": 1 });
db.transacciones_fallidas.createIndex({ "fecha_intento": -1 });

print("✅ Base de datos 'smart_ride_pagos' inicializada correctamente");
print("✅ Colecciones creadas: pagos, facturas, transacciones_fallidas, counters");
print("✅ Índices creados");