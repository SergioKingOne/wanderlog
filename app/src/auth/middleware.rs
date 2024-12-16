use actix_service::Service;
use actix_web::body::EitherBody;
use actix_web::dev::{ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage, HttpResponse};
use futures::future::{ok, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::rc::Rc;
use std::task::{Context, Poll};
use tracing::{error, info};

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService {
            service: Rc::new(service),
        })
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            if req.method() == actix_web::http::Method::OPTIONS {
                let res = service.call(req).await?;
                return Ok(res.map_into_left_body());
            }

            if let Some(auth_header) = req.headers().get("Authorization") {
                if let Ok(token) = auth_header.to_str() {
                    match validate_token(token).await {
                        Ok(claims) => {
                            req.extensions_mut().insert(claims.sub);
                        }
                        Err(e) => {
                            error!("Token validation failed: {}", e);
                            let response = HttpResponse::Unauthorized().json(
                                json!({ "error": "Invalid token", "details": format!("{}", e) }),
                            );
                            return Ok(ServiceResponse::new(req.into_parts().0, response)
                                .map_into_right_body());
                        }
                    }
                } else {
                    info!("Invalid Authorization header format");
                    let response = HttpResponse::Unauthorized()
                        .json(json!({ "error": "Invalid Authorization header format" }));
                    return Ok(
                        ServiceResponse::new(req.into_parts().0, response).map_into_right_body()
                    );
                }
            } else {
                info!("No Authorization header found");
                let response = HttpResponse::Unauthorized()
                    .json(json!({ "error": "No Authorization header found" }));
                return Ok(ServiceResponse::new(req.into_parts().0, response).map_into_right_body());
            }

            let res = service.call(req).await?;
            Ok(res.map_into_left_body())
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iss: String,
    aud: String,
    #[serde(rename = "cognito:username")]
    cognito_username: String,
}

async fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token = token.trim_start_matches("Bearer ").trim();

    // First, decode the token header without verification to get the kid
    let header = match jsonwebtoken::decode_header(token) {
        Ok(h) => h,
        Err(e) => {
            info!("Failed to decode token header: {}", e);
            return Err(e);
        }
    };

    let kid = header.kid.ok_or_else(|| {
        info!("No 'kid' found in token header");
        jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken)
    })?;

    let user_pool_id =
        std::env::var("COGNITO_USER_POOL_ID").expect("COGNITO_USER_POOL_ID must be set");
    let aws_region = std::env::var("AWS_REGION").expect("AWS_REGION must be set");

    let jwks_url = format!(
        "https://cognito-idp.{}.amazonaws.com/{}/.well-known/jwks.json",
        aws_region, user_pool_id
    );

    let client = reqwest::Client::new();
    let jwks = client
        .get(&jwks_url)
        .send()
        .await
        .expect("Failed to fetch JWKS")
        .json::<serde_json::Value>()
        .await
        .expect("Failed to parse JWKS");

    // Find the key with matching kid
    let key = jwks["keys"]
        .as_array()
        .and_then(|keys| keys.iter().find(|k| k["kid"].as_str() == Some(&kid)))
        .ok_or_else(|| {
            info!("Could not find matching key for kid: {}", kid);
            jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken)
        })?;

    let n = key["n"].as_str().expect("Failed to get key modulus");
    let e = key["e"].as_str().expect("Failed to get key exponent");

    let decoding_key = DecodingKey::from_rsa_components(n, e)?;

    let mut validation = Validation::new(Algorithm::RS256);
    let app_client_id = std::env::var("COGNITO_USER_POOL_CLIENT_ID")
        .expect("COGNITO_USER_POOL_CLIENT_ID must be set");

    validation.set_issuer(&[format!(
        "https://cognito-idp.{}.amazonaws.com/{}",
        aws_region, user_pool_id
    )]);
    validation.set_audience(&[app_client_id]);

    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
    info!(
        "Token validated for user: {}",
        token_data.claims.cognito_username
    );

    Ok(token_data.claims)
}
