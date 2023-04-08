use crate::auth;
use axum::Json;

pub async fn me(claims: auth::Claims) -> Result<Json<auth::Claims>, auth::AuthError> {
    Ok(Json(claims))
}
