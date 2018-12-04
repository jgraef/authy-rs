use phonenumber::PhoneNumber;

use crate::error::{ApiResult, TwilioErr};




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
        let result : reqwest::Result<reqwest::Response> = self.http.post(&url)
            .basic_auth(&self.account_sid, Some(&self.auth_token))
            .form(&vec![
                ("From", from.to_string()),
                ("To", to.to_string()),
                ("Body", message.to_string())
            ])
            .send();

        result.map_err(|e| TwilioErr::Http(e))
            .and_then(|mut r: reqwest::Response| {
                r.json()
                    .map_err(|e: reqwest::Error| TwilioErr::Http(e))
                    .and_then(|x: ApiResult<SendSmsResponse>| x.as_result()
                        .map_err(|e| TwilioErr::Api(e)))
            })
    }
}



#[cfg(test)]
mod tests {
    use crate::sms::*;
    use crate::error::*;

    const TEST_MESSAGE: &str = "Hello Rust";
    const VALID_NUMBER: &str = "+15005550010";
    const OUR_NUMBER: &str = "+15005550006 ";


    fn client() -> Client {
        dotenv::dotenv()
            .expect("No .env file found");
        let sid = dotenv::var("TEST_SID")
            .expect("No test SID in .env file");
        let token = dotenv::var("TEST_TOKEN")
            .expect("No test token in .env file");
        Client::new(&sid.to_owned(), &token.to_owned())
    }


    fn test_number(from: &str, to: &str, expected: Option<u32>) {
        let result = client().send_sms(
            &phonenumber::parse(None, from).expect("Can't parse phone number: from"),
            &phonenumber::parse(None, to).expect("Can't parse phone number: to"),
            TEST_MESSAGE
        );


        let got: Option<u32> = match result {
            Err(TwilioErr::Http(e)) => {
                assert!(false, format!("HttpError: {}", e));
                unreachable!()
            },
            Err(TwilioErr::Api(e)) => Some(e),
            _ => None
        }.map(|e| e.code);

        if got != expected {
            assert!(false, format!("Expected {:?}, but got: {:?}", expected, got));
        }
    }



    #[test]
    fn test_1() {
        test_number("+15005550001", VALID_NUMBER, Some(21212));
    }

    #[test]
    fn test_2() {
        test_number("+15005550007", VALID_NUMBER, Some(21606));
    }

    #[test]
    fn test_3() {
        test_number("+15005550008", VALID_NUMBER, Some(21611));
    }

    #[test]
    fn test_4() {
        test_number("+15005550006", VALID_NUMBER, None);
    }

    #[test]
    fn test_5() {
        test_number(VALID_NUMBER, VALID_NUMBER, Some(21606));
    }

    #[test]
    fn test_8() {
        test_number(OUR_NUMBER, "+15005550001", Some(21211));
    }

    #[test]
    fn test_9() {
        test_number(OUR_NUMBER, "+15005550002", Some(21612));
    }

    #[test]
    fn test_10() {
        test_number(OUR_NUMBER, "+15005550003", Some(21408));
    }

    #[test]
    fn test_11() {
        test_number(OUR_NUMBER, "+15005550004", Some(21610));
    }

    #[test]
    fn test_12() {
        test_number(OUR_NUMBER, "+15005550009", Some(21614));
    }

    #[test]
    fn test_13() {
        test_number(OUR_NUMBER, VALID_NUMBER, None);
    }
}
