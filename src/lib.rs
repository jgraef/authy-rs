extern crate reqwest;
extern crate phonenumber;
#[macro_use] extern crate serde_derive;
#[cfg(test)] extern crate dotenv;

mod sms;
mod error;


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