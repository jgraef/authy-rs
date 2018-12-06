use phonenumber::PhoneNumber;
use reqwest::header::HeaderName;
use reqwest::RequestBuilder;

use crate::error::{ApiResult, AuthyErr};


pub struct Client {
    http: reqwest::Client,
    api_key: String
}

pub enum Via {
    SMS,
    Call
}

impl Into<&str> for Via {
    fn into(self) -> &'static str {
        match self {
            Via::SMS => "sms",
            Via::Call => "call"
        }
    }
}


#[derive(Serialize, Debug)]
struct VerifyRequest<'a> {
    phone_number: &'a str,
    country_code: &'a str,
    via: &'a str,
    code_length: u8,
    locale: &'a str
}

#[derive(Deserialize, Debug)]
pub struct VerifyResponse {
    carrier: String,
    is_cellphone: bool,
    message: String,
    seconds_to_expire: u32,
    uuid: String,
    success: bool
}

#[derive(Serialize, Debug)]
pub struct CheckRequest<'a> {
    phone_number: &'a str,
    country_code: &'a str,
    verification_code: u32
}

#[derive(Deserialize, Debug)]
pub struct CheckResponse {
    message: String,
    success: bool
}

impl Into<bool> for CheckResponse {
    fn into(self) -> bool {
        self.success
    }
}

#[derive(Serialize, Debug)]
pub struct StatusRequest<'a> {
    uuid: &'a str
}

enum Status {
    Expired,
    Verified,
    Pending,
    Unknown
}

impl<'a> From<&'a str> for Status {
    fn from(value: &'a str) -> Status {
        match value {
            "expired" => Status::Expired,
            "verified" => Status::Verified,
            "pending" => Status::Pending,
            _ => Status::Unknown
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct StatusResponse {
    status: String,
    seconds_to_expire: u32,
    success: bool,
    message: String
}



impl Client {
    pub fn new(api_key: &str) -> Client {
        Client {
            http: reqwest::Client::new(),
            api_key: api_key.into()
        }
    }

    fn query<T, S>(&self, rb: RequestBuilder, data: &T) -> Result<S, AuthyErr>
        where T: serde::Serialize,
        for<'de> S: serde::Deserialize<'de>
    {
        let x_authy_api_key: HeaderName = HeaderName::from_static("x-authy-api-key");

        rb.header(x_authy_api_key, self.api_key.clone())
            .form(data)
            .send()
            .map_err(AuthyErr::Http)
            .and_then(|mut r: reqwest::Response| {
                r.json()
                    .map_err(AuthyErr::Http)
                    .and_then(|r: ApiResult<S>| match r {
                            ApiResult::Ok(x) => Ok(x),
                            ApiResult::Err(x) => Err(x)
                        }.map_err(AuthyErr::Api))
            })
    }

    pub fn verify(&self, number: &PhoneNumber, via: Via, code_length: u8, locale: &str) -> Result<VerifyResponse, AuthyErr> {
        self.query::<VerifyRequest, VerifyResponse>(
            self.http.post("https://api.authy.com/protected/json/phones/verification/start"),
            &VerifyRequest {
                    phone_number: &number.national().to_string(),
                    country_code: &number.code().value().to_string(),
                    via: via.into(),
                    code_length,
                    locale,
                })
    }

    pub fn check(&self, number: &PhoneNumber, verification_code: u32) -> Result<CheckResponse, AuthyErr> {
        self.query::<CheckRequest, CheckResponse>(
            self.http.get("https://api.authy.com/protected/json/phones/verification/check"),
            &CheckRequest {
                    phone_number: &number.national().to_string(),
                    country_code: &number.code().value().to_string(),
                    verification_code
                })
    }

    pub fn status(&self, uuid: &str) -> Result<StatusResponse, AuthyErr> {
        self.query::<StatusRequest, StatusResponse>(
            self.http.get("https://api.authy.com/protected/json/phones/verification/status"),
            &StatusRequest { uuid }
        )
    }
}


#[cfg(test)]
mod tests {
    use crate::client::{Client, Via};
    use phonenumber::PhoneNumber;

    fn setup() -> (Client, PhoneNumber) {
        dotenv::dotenv()
            .expect("No .env file found");
        (Client::new(&dotenv::var("VERIFY_API_KEY")
            .expect("No verify API key in .env file")),
         phonenumber::parse(None, dotenv::var("VERIFY_TEST_PHONE_NUMBER")
             .expect("No verify test phone number in .env file"))
             .expect("Can't parse test phone number"))
    }


    #[test]
    fn test_verify() {
        let (client, test_phone_number) = setup();
        println!("Response:\n{:#?}", client.verify(&test_phone_number, Via::SMS, 6, "de")
            .expect("verify error"));
    }

    #[test]
    fn test_check() {
        let verification_code: Option<u32> = None;
        let verification_code = verification_code.expect("You must set the verification code here to get a positive result");

        let (client, test_phone_number) = setup();
        println!("Response:\n{:#?}", client.check(&test_phone_number, verification_code)
            .expect("check error"));
    }

    #[test]
    fn test_status() {
        let uuid: Option<&str> = None;
        let uuid = uuid.expect("You must set the UUID for a verification request here to get a positive result");

        let (client, _) = setup();
        println!("Response:\n{:#?}", client.status(uuid));
    }
}