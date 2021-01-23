use tonic::Status;

pub struct CastCodeStatus;

impl CastCodeStatus {
    pub fn cast_to_tonic_status(
        status_code: domain::errors::StatusCode,
        message: String,
    ) -> Status {
        match status_code {
            domain::errors::StatusCode::NotFound => Status::not_found(message),
            domain::errors::StatusCode::Unauthorized => Status::unauthenticated(message),
            domain::errors::StatusCode::InternalServerError => Status::internal(message),
            domain::errors::StatusCode::Conflict => Status::already_exists(message),
            domain::errors::StatusCode::BadRequest => Status::invalid_argument(message),
        }
    }
}
