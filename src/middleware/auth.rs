use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::LocalBoxFuture;
use std::future::{ready, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32,           // id_usuario
    pub email: String,
    pub rol: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Clone)]
pub struct AuthUser {
    pub id_usuario: i32,
    pub email: String,
    pub rol: String,
}

pub struct AuthMiddleware {
    pub jwt_secret: String,
    pub required_roles: Vec<String>,
}

impl AuthMiddleware {
    pub fn new(jwt_secret: String, required_roles: Vec<String>) -> Self {
        Self {
            jwt_secret,
            required_roles,
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service,
            jwt_secret: self.jwt_secret.clone(),
            required_roles: self.required_roles.clone(),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
    jwt_secret: String,
    required_roles: Vec<String>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Extraer token del header Authorization
        let auth_header = req.headers().get("Authorization");
        
        let token = match auth_header {
            Some(value) => {
                match value.to_str() {
                    Ok(v) if v.starts_with("Bearer ") => v[7..].to_string(),
                    _ => {
                        return Box::pin(async move {
                            Err(AppError::Unauthorized("Formato de token inválido".to_string()).into())
                        });
                    }
                }
            }
            None => {
                return Box::pin(async move {
                    Err(AppError::Unauthorized("Token no proporcionado".to_string()).into())
                });
            }
        };
        
        // Validar JWT
        let validation = Validation::new(Algorithm::HS256);
        let jwt_secret = self.jwt_secret.clone();
        let required_roles = self.required_roles.clone();
        
        let decode_result = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &validation,
        );
        
        match decode_result {
            Ok(token_data) => {
                let claims = token_data.claims;
                let user_rol = claims.rol.clone(); // ✅ CLONAR ANTES
                
                // Verificar rol si es necesario
                if !required_roles.is_empty() && !required_roles.contains(&user_rol) {
                    //  Usar move
                    return Box::pin(async move {
                        Err(AppError::Forbidden(format!(
                            "Rol '{}' no tiene permisos para esta operación",
                            user_rol
                        ))
                        .into())
                    });
                }
                
                // Insertar usuario en extensions
                let auth_user = AuthUser {
                    id_usuario: claims.sub,
                    email: claims.email.clone(),
                    rol: user_rol.clone(),
                };
                
                req.extensions_mut().insert(auth_user);
                
                tracing::info!(
                    "✅ Usuario autenticado: {} (ID: {}, Rol: {})",
                    claims.email,
                    claims.sub,
                    user_rol
                );
                
                let fut = self.service.call(req);
                Box::pin(async move {
                    fut.await
                })
            }
            Err(err) => {
                let error_msg = err.to_string(); // ✅ CONVERTIR A STRING ANTES
                //  Usar move
                Box::pin(async move {
                    Err(AppError::Unauthorized(format!("Token inválido: {}", error_msg)).into())
                })
            }
        }
    }
}

// Helper para extraer usuario autenticado
pub fn get_auth_user(req: &ServiceRequest) -> Result<AuthUser, AppError> {
    req.extensions()
        .get::<AuthUser>()
        .cloned()
        .ok_or_else(|| AppError::Unauthorized("Usuario no autenticado".to_string()))
}