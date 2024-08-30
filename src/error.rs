//! Main Crate Error


#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),
    
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    SystemTime(#[from] std::time::SystemTimeError),

    #[error(transparent)]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error(transparent)]
    MongoQuery(#[from] mongodb::error::Error),

    #[error("Can not extract key for user {0}")]
    MongoKey(String),

    #[error(transparent)]
    MongoValueAccess(#[from] mongodb::bson::document::ValueAccessError),

    #[error(transparent)]
    MongoUserId(#[from] mongodb::bson::oid::Error),

    #[error("User user_id={0} not found in database")]
    UserNotFound(String),

    #[error(transparent)]
    AddParse(#[from] std::net::AddrParseError),

    #[error("Can not parse variable: {input}")]
    Var {
        input: &'static str,
        #[source] 
        source: std::env::VarError,
    },

    #[error("Password hash error: {0}")]
    PasswordHash(String),
}