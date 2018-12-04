use phonenumber::{PhoneNumber, country};

use crate::error::{ApiError, ApiResult, TwilioErr};




#[derive(Deserialize, Debug)]
pub struct SubResourceUris {
    media: String
}

#[derive(Deserialize, Debug)]
pub struct SendSmsResponse {
    account_sid: String,
    api_version: String,
    body: String,
    date_created: String,
    date_sent: Option<String>,
    date_updated: Option<String>,
    direction: String,
    error_code: Option<String>,
    error_mesasge: Option<String>,
    from: String,
    messaging_service_sid: Option<String>,
    num_media: String,
    num_segments: String,
    price: Option<f32>,
    price_unit: String,
    sid: String,
    status: String,
    subresource_uris: SubResourceUris,
    to: String,
    uri: String
}



pub struct Client {
    http: reqwest::Client,
    account_sid: String,
    auth_token: String
}


impl Client {
    pub fn new(account_sid: &str, auth_token: &str) -> Client {
        Client {
            http: reqwest::Client::new(),
            account_sid: account_sid.into(),
            auth_token: auth_token.into()
        }
    }

    pub fn send_sms(&self, from: &PhoneNumber, to: &PhoneNumber, message: &str) -> Result<SendSmsResponse, TwilioErr> {
        let url = format!("https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json", self.account_sid);
        let mut result : reqwest::Result<reqwest::Response> = self.http.post(&url)
            .basic_auth(&self.account_sid, Some(&self.auth_token))
            .form(&vec![
                ("From", from.to_string()),
                ("To", to.to_string()),
                ("Body", message.to_string())
            ])
            .send();

        result.map_err(|mut e| TwilioErr::Http(e))
            .and_then(|mut r: reqwest::Response| {
                r.json()
                    .map_err(|e: reqwest::Error| TwilioErr::Http(e))
                    .and_then(|x: ApiResult<SendSmsResponse>| x.as_result()
                        .map_err(|e| TwilioErr::Api(e)))
            })
    }
}