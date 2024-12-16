use actix_service::Service;
use actix_web::body::EitherBody;
use actix_web::dev::{ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage};
use futures::future::{ok, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::task::{Context, Poll};

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
            if let Some(auth_header) = req.headers().get("Authorization") {
                if let Ok(token) = auth_header.to_str() {
                    if let Ok(claims) = validate_token(token).await {
                        req.extensions_mut().insert(claims.sub);
                    }
                }
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
    #[serde(rename = "cognito:username")]
    cognito_username: String,
}

async fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token = token.trim_start_matches("Bearer ").trim();

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
