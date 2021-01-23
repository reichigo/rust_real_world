use tonic::Response;
use use_case::IInsertUserUseCase;

use crate::grpc::generated::user_proto::user_server::User;


pub struct UserHandler<UR: domain::repositories::IUserRepository>{
    user_repository: UR
}

impl<UR: domain::repositories::IUserRepository> UserHandler<UR> {
    pub fn new(user_repository: UR) -> Self{
        Self{
            user_repository
        }
    }
}

#[tonic::async_trait]
impl<UR: 'static +  domain::repositories::IUserRepository + Sync + Send> User for UserHandler<UR> {
    async fn insert(
            &self,
            request: tonic::Request<crate::grpc::generated::user_proto::InsertUserRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
        
            use_case::InsertUserUseCase::new(&self.user_repository)
            .execult(&request.get_ref().name)
            .await
            .map_err(|e| cross_cutting::utils::CastCodeStatus::cast_to_tonic_status(e.status_code, e.message))
            .map(|_| Response::new(()))

      
    }
}