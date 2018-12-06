# twilio-rs

NOTE: There is already a more mature twilio client: https://crates.io/crates/twilio
THerefore I will probably change this into a client for the Authy API only. Expect the repository to be renamed and SMS support to be removed.

Rust client library for sending SMS with [Twilio](https://www.twilio.com).

Disclaimer: This crate is not an official Twilio product.

This crate uses features from Rust nightly. To use Rust nightly in your current project run:

```sh
rustup override set nightly
```

## Features:

 * Sending SMS
 * Phone number verification

## Examples

### Sending a SMS

```rust
extern crate twilio;
extern crate phonenumber;

use twilio::sms::Client;


let client = Client::new("<API SID>", "<API AUTH TOKEN>");

let result = client().send_sms(
            &phonenumber::parse(None, "<FROM PHONE NUMBER>").expect("Can't parse phone number: from"),
            &phonenumber::parse(None, "<TO PHONE NUMBER>").expect("Can't parse phone number: to"),
            "Hello World");

match result {
	Ok(r) => println!("Result:\n{:#?}", r),
	Err(e) => println!("Error: {}", e.description())
};
```

### Verifying a phone number

```rust
use twilio::authy::{Client, Via};
use phonenumber::PhoneNumber;

let client = Client::new("<AUTHY API KEY>");

let verify_response = client.verify("TEST PHONE NUMBER", Via::SMS, 6, "en").expect("verify failed");

let status_response = client.status(verify_response.uuid.into()).expect("status failed");
println!("Status:\n{:#?}", status_response);

// The verification code received as SMS
let code: u32 = 123456;
let check_response = client.check("TEST PHONE NUMBER", code).expect("check failed");
if check_response.success {
	println!("Phone number verified!");
}
else {
	println!("Verification failed");
}
```

## TODO:

 * build in extended error information

