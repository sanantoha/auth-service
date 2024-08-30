use mongodb::bson::document::ValueAccessError;
use proto::auth_server::Auth;
use log::{error, info, warn};
use std::sync::Arc;
use crate::error::Error;

pub mod proto {
    tonic::include_proto!("auth"); // auth is a package in auth.proto file

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("auth_descriptor");
}

use crate::{jwt::generate_jwt, user_repository::UserRepository};
const VALID_SECONDS: usize = 3600; // 1h


#[derive(Debug)]
pub struct AuthService {
    user_repository: Arc<UserRepository>,
    secret: String
}

impl AuthService {
    pub fn new(repository: Arc<UserRepository>, secret: String) -> Self {
        AuthService {
            user_repository: repository,
            secret
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

        let token = generate_jwt(VALID_SECONDS, &self.secret)
            .map_err(|e| {
                error!("Error generate_jwt {}", e);
                tonic::Status::internal("internal server error")
        })?;
        info!("token is generated: {:?}", token);

        let response = proto::LoginResponse { token };

        return Ok(tonic::Response::new(response));
    }

    async fn register(&self, request: tonic::Request<proto::RegisterRequest>) -> Result<tonic::Response<proto::RegisterResponse>, tonic::Status> {
        let req = request.get_ref();
        info!("received user register request email: {}", req.email);

        let user_id = self.user_repository.register_user(&req.email, &req.password).await
            .map_err(|e| {
                error!("Error executing user_repository.register_user function: {}", e);
                tonic::Status::internal("internal server error")        
        })?;

        let response = proto::RegisterResponse { user_id };

        return Ok(tonic::Response::new(response));
    }

    async fn is_admin(&self, request: tonic::Request<proto::IsAdminRequest>) -> Result<tonic::Response<proto::IsAdminRespons>, tonic::Status> {
        let req = request.get_ref();
        info!("received user is_admin request user_id: {}", req.user_id);

        let is_admin = self.user_repository.is_admin(&req.user_id).await
                .map_err(|e| {
                    match e {
                        Error::UserNotFound(_) => {
                            warn!("user user_id={} not found in user database", req.user_id);
                            tonic::Status::not_found("user not found")
                        },
                        Error::MongoUserId(_) => {
                            warn!("user_id={} is invalid", req.user_id);
                            tonic::Status::invalid_argument(format!("user_id={} is invalid", req.user_id))
                        },
                        Error::MongoValueAccess(ValueAccessError::NotPresent) => {
                            warn!("user_id={} does not have is_admin property", req.user_id);
                            tonic::Status::invalid_argument(format!("user_id={} does not have is_admin property", req.user_id))
                        },
                        Error::MongoValueAccess(ValueAccessError::UnexpectedType) => {
                            warn!("is_admin has wrong type for user_id={}", req.user_id);
                            tonic::Status::internal("internal server error")
                        },                             
                        _ => {
                            error!("undefined error for requet IsAdminRequest with user_id={}", req.user_id);
                            tonic::Status::internal("internal server error")
                        }                            
                    }
                })?;
        
        let resposne = proto::IsAdminRespons { is_admin };

        return Ok(tonic::Response::new(resposne));
    }
}