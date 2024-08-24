use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, Header, EncodingKey};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::error::Error;

pub const SECRET_NAME: &str = "AUTH_SECRET";

#[warn(private_interfaces)]
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize, // Expiration time (as UTC timestamp)
}

pub fn generate_jwt(valid_seconds: usize, secret: &str) -> Result<String, Error> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs() as usize + valid_seconds;

    let claims = Claims {
        sub: "user@example.com".to_string(),
        company: "My Company".to_string(),
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}
// not sure if it needed.
// pub fn validate_jwt(token: &str) -> Result<TokenData<Claims>, Error> {    
//     let secret = env::var(SECRET_NAME)
//         .map_err(|e| Error::Var { input: SECRET_NAME, source: e })?;

//     let token_data = decode::<Claims>(
//         token,
//         &DecodingKey::from_secret(secret.as_ref()),
//         &Validation::default(),
//     )?;

//     Ok(token_data)
// }