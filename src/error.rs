use std::fmt;
use std::error::Error;




#[derive(Deserialize)]
#[serde(untagged)]
pub enum ApiResult<T> {
    Ok(T),
    Err(ApiError)
}

/*
impl<T> Into<Result<T, ApiError>> for ApiResult<T> {
    fn into(self) -> Result<T, ApiError> {
        match self {
            ApiResult::Ok(x) => Ok(x),
            ApiResult::Err(x) => Err(x)
        }
    }
}


impl<T> From<ApiResult<T>> for Result<T, ApiError> {
    fn from(r: ApiResult<T>) -> Self {
        match r {
            ApiResult::Ok(x) => Ok(x),
            ApiResult::Err(x) => Err(x)
        }
    }
}
*/

impl<T> ApiResult<T> {
    pub fn as_result(self) -> Result<T, ApiError> {
        match self {
            ApiResult::Ok(x) => Ok(x),
            ApiResult::Err(x) => Err(x)
        }
    }
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
pub enum TwilioErr {
    Api(ApiError),
    Http(reqwest::Error)
}


impl Error for TwilioErr {
    fn description(&self) -> &str {
        match self {
            TwilioErr::Api(cause) => cause.description(),
            TwilioErr::Http(cause) => cause.description(),
        }
    }

    fn source(&self) -> Option<&(Error + 'static)> {
        match self {
            TwilioErr::Api(cause) => Some(cause),
            TwilioErr::Http(cause) => Some(cause),
        }
    }
}

impl fmt::Display for TwilioErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}
