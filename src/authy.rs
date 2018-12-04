
/*
curl -XPOST 'https://api.authy.com/protected/json/phones/verification/start' \
-H "X-Authy-API-Key: YOUR_AUTHY_API_KEY" \
-d via='sms' \
-d phone_number='5551234567' \
-d country_code=1 \
-d code_length=6 \
-d locale='en'


{
  "carrier": "AT&T Wireless",
  "is_cellphone": true,
  "message": "Text message sent to +1 987-654-3210.",
  "seconds_to_expire": 599,
  "uuid": "b8ebcd40-1234-5678-3fb5-0e5d6a065904",
  "success": true
}


curl 'https://api.authy.com/protected/json/phones/verification/status?uuid=b8ebcd40-1234-5678-3fb5-0e5d6a065904' \
-H "X-Authy-API-Key: d57d919d11e6b221c9bf6f7c882028f9"


{
  "message": "Phone Verification status.",
  "status": "verified",
  "seconds_to_expire": 474,
  "success": true
}
*/


use phonenumber::PhoneNumber;
use reqwest::header::HeaderName;
use reqwest::RequestBuilder;

use crate::error::{ApiResult, TwilioErr};



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


#[derive(Deserialize, Debug)]
pub struct VerifyResponse {
    carrier: String,
    is_cellphone: bool,
    message: String,
    seconds_to_expire: u32,
    uuid: String,
    success: bool
}

#[derive(Deserialize, Debug)]
pub struct CheckResponse {
    message: String,
    success: bool
}




impl Client {
    pub fn new(api_key: &str) -> Client {
        Client {
            http: reqwest::Client::new(),
            api_key: api_key.into()
        }
    }

    /*fn query<T>(&self, rb: RequestBuilder) -> Result<T, TwilioErr> {
        let x_authy_api_key: HeaderName = HeaderName::from_static("x-authy-api-key");
        let result : reqwest::Result<reqwest::Response> = self.http.post("https://api.authy.com/protected/json/phones/verification/start")
            .header(x_authy_api_key, self.api_key.clone())
            .send();

        result.map_err(|e| TwilioErr::Http(e))
            .and_then(|mut r: reqwest::Response| {
                r.json()
                    .map_err(|e: reqwest::Error| TwilioErr::Http(e))
                    .and_then(|x: ApiResult<T>| x.as_result()
                        .map_err(|e| TwilioErr::Api(e)))
            })
    }*/

    pub fn verify(&self, number: &PhoneNumber, via: Via, code_length: u8, locale: &str) -> Result<VerifyResponse, TwilioErr> {
        let x_authy_api_key: HeaderName = HeaderName::from_static("x-authy-api-key");
        let result : reqwest::Result<reqwest::Response> = self.http.post("https://api.authy.com/protected/json/phones/verification/start")
            .header(x_authy_api_key, self.api_key.clone())
            .form(&vec![
                ("via", {let x: &str = via.into(); x}.to_string()),
                ("phone_number", number.national().to_string()),
                ("country_code", number.code().value().to_string()),
                ("code_length", code_length.to_string()),
                ("locale", locale.to_string())
            ])
            .send();

        result.map_err(|e| TwilioErr::Http(e))
            .and_then(|mut r: reqwest::Response| {
                r.json()
                    .map_err(|e: reqwest::Error| TwilioErr::Http(e))
                    .and_then(|x: ApiResult<VerifyResponse>| x.as_result()
                        .map_err(|e| TwilioErr::Api(e)))
            })
    }

    pub fn check(&self, number: &PhoneNumber, verification_code: u32) -> Result<CheckResponse, TwilioErr> {
        let x_authy_api_key: HeaderName = HeaderName::from_static("x-authy-api-key");
        let result : reqwest::Result<reqwest::Response> = self.http.get("https://api.authy.com/protected/json/phones/verification/check")
            .header(x_authy_api_key, self.api_key.clone())
            .form(&vec![
                ("country_code", number.code().value().to_string()),
                ("phone_number", number.national().to_string()),
                ("verification_code", verification_code.to_string())
            ])
            .send();

        result.map_err(|e| TwilioErr::Http(e))
            .and_then(|mut r: reqwest::Response| {
                r.json()
                    .map_err(|e: reqwest::Error| TwilioErr::Http(e))
                    .and_then(|x: ApiResult<CheckResponse>| x.as_result()
                        .map_err(|e| TwilioErr::Api(e)))
            })
    }
}


#[cfg(test)]
mod tests {
    use crate::authy::{Client, Via};
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
        let verification_code = 218904;
        let (client, test_phone_number) = setup();
        println!("Response:\n{:#?}", client.check(&test_phone_number, verification_code)
            .expect("check error"));
    }
}