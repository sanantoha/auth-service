use proto::auth_server::Auth;
use log::{error, info, warn};
use std::sync::Arc;

pub mod proto {
    tonic::include_proto!("auth"); // auth is a package in calculator.proto file

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("auth_descriptor");
}

use crate::{jwt::generate_jwt, repository::UserRepository};
const VALID_SECONDS: usize = 3600; // 1h


#[derive(Debug)]
pub struct AuthService {
    user_repository: Arc<UserRepository>
}

impl AuthService {
    pub fn new(repository: Arc<UserRepository>) -> Self {
        AuthService {
            user_repository: repository
        }
    }
}

#[tonic::async_trait]
impl Auth for AuthService {

    async fn login(&self, request: tonic::Request<proto::LoginRequest>) -> Result<tonic::Response<proto::LoginResponse>, tonic::Status> {        
        let req = request.get_ref();
        info!("received user login request email: {}, app_id: {}", req.email, req.app_id);

        let is_valid = self.user_repository.is_valid_user(&req.email, &req.password).await
            .map_err(|e| {
                error!("Error executing user_repository.is_valid_user function: {}", e);
                tonic::Status::internal("internal server error")        
        })?;

        if !is_valid {
            let msg = format!("user {} not found or wrong password", req.email);
            warn!("{}", msg);
            return Err(tonic::Status::not_found(msg));
        }

        let token = generate_jwt(VALID_SECONDS)
            .map_err(|e| {
                error!("Error generate_jwt {}", e);
                tonic::Status::internal("internal server error")
        })?;
        info!("token is generated: {:?}", token);

        let response = proto::LoginResponse { token };

        return Ok(tonic::Response::new(response));                
    }
}