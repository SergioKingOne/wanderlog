use actix_web::body::EitherBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage};
use futures::future::{ok, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::future::Future;
use std::pin::Pin;

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService { service })
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(token) = auth_header.to_str() {
                if let Ok(claims) = validate_token(token) {
                    // Add Cognito user ID to request extensions
                    req.extensions_mut().insert(claims.sub);
                }
            }
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res.map_into_left_body())
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iss: String,
    #[serde(rename = "cognito:username")]
    cognito_username: String,
    // Add other claims as needed
}

fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    // Remove "Bearer " prefix if present
    let token = token.trim_start_matches("Bearer ").trim();

    // Get the Cognito user pool ID from environment variables
    let user_pool_id = env::var("COGNITO_USER_POOL_ID").expect("COGNITO_USER_POOL_ID must be set");
    let aws_region = env::var("AWS_REGION").expect("AWS_REGION must be set");

    // Construct the JWT validation URL
    let jwks_url = format!(
        "https://cognito-idp.{}.amazonaws.com/{}/.well-known/jwks.json",
        aws_region, user_pool_id
    );

    // TODO: In production, we should cache this key and refresh periodically
    // For now, we'll use a simple synchronous request
    let jwks = reqwest::blocking::get(&jwks_url)
        .expect("Failed to fetch JWKS")
        .json::<serde_json::Value>()
        .expect("Failed to parse JWKS");

    // Get the first key from JWKS (you might want to match kid in production)
    let key = jwks["keys"][0]["n"]
        .as_str()
        .expect("Failed to get key from JWKS");

    let decoding_key =
        DecodingKey::from_rsa_components(key, &jwks["keys"][0]["e"].as_str().unwrap())?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&[format!(
        "https://cognito-idp.{}.amazonaws.com/{}",
        aws_region, user_pool_id
    )]);

    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;

    Ok(token_data.claims)
}
