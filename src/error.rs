use std::fmt;
use std::error::Error;




#[derive(Deserialize)]
#[serde(untagged)]
pub enum ApiResult<T> {
    Ok(T),
    Err(ApiError)
}

#[derive(Deserialize, Debug)]
pub struct ApiError {
    pub code: u32,
    pub message: String,
    pub more_info: String,
    pub status: u32
}

impl Error for ApiError {
    fn description(&self) -> &str {
        self.message.as_str()
    }

    fn source(&self) -> Option<&(Error + 'static)> {
        None
    }
}

/*
impl PartialEq for ErrorResponse {
    fn eq(&self, other: &ErrorResponse) -> bool {
        self.code == other.code
    }
}
*/

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ErrorResponse #{}: {}", self.code, self.message)
    }
}

impl Into<u32> for ApiError {
    fn into(self) -> u32 {
        self.code
    }
}



#[derive(Debug)]
pub enum AuthyErr {
    Api(ApiError),
    Http(reqwest::Error)
}


impl Error for AuthyErr {
    fn description(&self) -> &str {
        match self {
            AuthyErr::Api(cause) => cause.description(),
            AuthyErr::Http(cause) => cause.description(),
        }
    }

    fn source(&self) -> Option<&(Error + 'static)> {
        match self {
            AuthyErr::Api(cause) => Some(cause),
            AuthyErr::Http(cause) => Some(cause),
        }
    }
}

impl fmt::Display for AuthyErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}
