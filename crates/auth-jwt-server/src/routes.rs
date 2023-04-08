use crate::auth;

pub async fn protected(claims: auth::Claims) -> Result<String, auth::AuthError> {
    // Send the protected data to the user
    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{}",
        claims
    ))
}
