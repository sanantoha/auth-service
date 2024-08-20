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

    #[error(transparent)]
    AddParse(#[from] std::net::AddrParseError),

    // #[error(transparent)]
    // Var(#[from] std::error::Va),

    #[error("Can not parse variable: {input}")]
    Var {
        input: &'static str,
        #[source] 
        source: std::env::VarError,
    },
}