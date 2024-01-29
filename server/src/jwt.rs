use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json, RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::types::chrono;

lazy_static! {
    static ref ENCODING_KEY: EncodingKey =
        EncodingKey::from_secret(std::env::var("SECRET").unwrap().as_bytes());
    static ref DECODING_KEY: DecodingKey =
        DecodingKey::from_secret(std::env::var("SECRET").unwrap().as_bytes());
    static ref VALIDATION: Validation = Validation::new(Algorithm::HS256);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub name: String,
    pub user_id: String,
}

impl Claims {
    pub fn new(name: String, user_id: String) -> Self {
        Self {
            exp: (chrono::Utc::now().timestamp() + 3600 * 24 * 30)
                as usize,
            name,
            user_id,
        }
    }
}

impl std::fmt::Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}\nId: {}", &self.name, &self.user_id)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        log::debug!("token: {}", bearer.token());
        // Decode the user data
        let token_data = decode(bearer.token()).map_err(|err| { log::debug!("{:#?}", err); AuthError::InvalidToken })?;

        Ok(token_data.claims)
    }
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

pub fn encode(claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
    jsonwebtoken::encode(&Header::default(), claims, &ENCODING_KEY)
}

pub fn decode(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    jsonwebtoken::decode(token, &DECODING_KEY, &VALIDATION)
}
