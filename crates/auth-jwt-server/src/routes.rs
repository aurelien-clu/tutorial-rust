use crate::auth;
use axum::Json;
use jsonwebtoken::{encode, Header};

pub async fn protected(claims: auth::Claims) -> Result<String, auth::AuthError> {
    // Send the protected data to the user
    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{}",
        claims
    ))
}

pub async fn authorize(
    Json(payload): Json<auth::AuthPayload>,
) -> Result<Json<auth::AuthBody>, auth::AuthError> {
    // Check if the user sent the credentials
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(auth::AuthError::MissingCredentials);
    }
    // Here you can check the user credentials from a database
    if payload.client_id != "foo" || payload.client_secret != "bar" {
        return Err(auth::AuthError::WrongCredentials);
    }
    let claims = auth::Claims {
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        // Mandatory expiry time as UTC timestamp
        exp: 2000000000, // May 2033
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &auth::KEYS.encoding)
        .map_err(|_| auth::AuthError::TokenCreation)?;

    // Send the authorized token
    Ok(Json(auth::AuthBody::new(token)))
}
