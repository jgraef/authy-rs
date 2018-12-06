extern crate reqwest;
extern crate phonenumber;
#[macro_use] extern crate serde_derive;
#[cfg(test)] extern crate dotenv;


pub mod error;
pub mod client;

pub use crate::client::{Client, VerifyResponse, CheckResponse, StatusResponse};
pub use crate::error::AuthyErr;
