use dotenvy::dotenv;
use std::{env, collections::HashMap};
use serde_json::Value;

#[derive(Debug, serde::Deserialize)]
pub struct Error{
    message: String,
    code: u32,
    #[serde(default)]
    data: Value,
}
#[derive(Debug, serde::Deserialize)]
pub struct ErrorRes{
    error:Error
}
pub fn get_token() -> String {
    dotenv().expect(".env file not found");
    env::var("token").expect("token not found")
}