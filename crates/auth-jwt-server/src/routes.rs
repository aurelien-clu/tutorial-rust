use crate::auth;
use axum::Json;

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
    match auth::authorize(payload) {
        Ok(auth_body) => Ok(Json(auth_body)),
        Err(err) => Err(err),
    }
}
