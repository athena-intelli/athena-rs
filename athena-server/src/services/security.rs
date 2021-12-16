use tonic::{Request, Response, Status};
use athena_api::pb::security_service::security_service_server::SecurityService;
use athena_api::pb::security_service::{ChangePasswordRequest, LoginResponse, UsernameAndPasswordToken};

#[derive(Default)]
pub struct SecurityServiceImpl;

#[tonic::async_trait]
impl SecurityService for SecurityServiceImpl {
    async fn login(&self, request: Request<UsernameAndPasswordToken>) -> Result<Response<LoginResponse>, Status> {
        todo!()
    }

    async fn logout(&self, request: Request<UsernameAndPasswordToken>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn change_password(&self, request: Request<ChangePasswordRequest>) -> Result<Response<()>, Status> {
        todo!()
    }
}