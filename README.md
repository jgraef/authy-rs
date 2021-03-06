# authy-rs

Rust client for the Authy API by [Twilio](https://www.twilio.com). 

Disclaimer: This crate is not an official Twilio product.

## Example

```rust
extern crate authy_rs;

use authy_rs::authy::{Client, Via};
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
