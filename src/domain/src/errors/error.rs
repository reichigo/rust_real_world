pub struct Error {
    pub message: String,
    pub status_code: StatusCode,
}

#[derive(Eq, PartialEq, Debug)]
pub enum StatusCode {
    NotFound,
    Unauthorized,
    InternalServerError,
    Conflict,
    BadRequest,
}

impl Error {
    pub fn not_found(error: String) -> Self {
        Self {
            message: error,
            status_code: StatusCode::NotFound,
        }
    }

    pub fn unauthorized(error: String) -> Self {
        Self {
            message: error,
            status_code: StatusCode::Unauthorized,
        }
    }

    pub fn internal_server_error(error: String) -> Self {
        Self {
            message: error,
            status_code: StatusCode::InternalServerError,
        }
    }

    pub fn conflict(error: String) -> Self {
        Self {
            message: error,
            status_code: StatusCode::Conflict,
        }
    }

    pub fn bad_request(error: String) -> Self {
        Self {
            message: error,
            status_code: StatusCode::BadRequest,
        }
    }
}

impl StatusCode {
    pub fn get_code(status_code: StatusCode) -> i32 {
        match status_code {
            StatusCode::NotFound => 404,
            StatusCode::Unauthorized => 401,
            StatusCode::InternalServerError => 500,
            StatusCode::Conflict => 409,
            StatusCode::BadRequest => 400,
        }
    }
}

