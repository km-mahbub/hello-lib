#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}

#[napi]
pub fn add(a: u32, b: u32) -> u32 {
  a + b
}

#[napi]
pub async fn get_data(url: String) -> Result<String> {
  let response = reqwest::get(url).await.map_err(|e| {
    Error::new(
      Status::GenericFailure,
      format!("HTTP request failed: {}", e),
    )
  })?;

  let body = response.text().await.map_err(|e| {
    Error::new(
      Status::GenericFailure,
      format!("Failed to read response text: {}", e),
    )
  })?;

  Ok(body)
}
