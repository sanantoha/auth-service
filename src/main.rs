use crate::auth::proto::auth_server::AuthServer;
use repository::UserRepository;
use tonic::transport::Server;
use crate::error::Error;
use mongodb::Client;
use std::sync::Arc;
use std::env;

mod auth;
mod jwt;
mod error;
mod repository;

const AUTH_SERVICE_PORT_NAME: &str = "AUTH_PORT";

#[tokio::main]
async fn main() -> Result<(), Error> {

    env_logger::init();

    let port = env::var(AUTH_SERVICE_PORT_NAME)
        .map_err(|e| Error::Var { input: AUTH_SERVICE_PORT_NAME, source: e })?;

    let addr = format!("[::1]:{}", port).parse()?;

    let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    let user_repository = UserRepository::new(Arc::new(client));
    let auth_service = auth::AuthService::new(Arc::new(user_repository));



    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(auth::proto::FILE_DESCRIPTOR_SET)
        .build().map_err(|e| Error::Generic(e.to_string()))?;

    Server::builder()
        .add_service(reflection_service)
        .add_service(AuthServer::new(auth_service))
        .serve(addr)
        .await
        .map_err(|e| Error::Generic(e.to_string()))?;

    Ok(())
}
