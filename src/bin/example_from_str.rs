use cryptolens;

fn main() {
  let license_key = cryptolens::LicenseKey::from_str(
      r#"{"licenseKey":"eyJQcm9kdWN0SWQiOjM2NDYsIklEIjo0LCJLZXkiOiJNUERXWS1QUUFPVy1GS1NDSC1TR0FBVSIsIkNyZWF0ZWQiOjE0OTAzMTM2MDAsIkV4cGlyZXMiOjE0OTI5MDU2MDAsIlBlcmlvZCI6MzAsIkYxIjpmYWxzZSwiRjIiOmZhbHNlLCJGMyI6ZmFsc2UsIkY0IjpmYWxzZSwiRjUiOmZhbHNlLCJGNiI6ZmFsc2UsIkY3IjpmYWxzZSwiRjgiOmZhbHNlLCJOb3RlcyI6bnVsbCwiQmxvY2siOmZhbHNlLCJHbG9iYWxJZCI6MzE4NzYsIkN1c3RvbWVyIjpudWxsLCJBY3RpdmF0ZWRNYWNoaW5lcyI6W3siTWlkIjoiIiwiSVAiOiIxNTUuNC4xMzQuMjciLCJUaW1lIjoxNDkxODk4OTE4fSx7Ik1pZCI6ImxvbCIsIklQIjoiMTU1LjQuMTM0LjI3IiwiVGltZSI6MTQ5MTg5ODk5NX0seyJNaWQiOiIyODlqZjJhZnNmIiwiSVAiOiIxNTUuNC4xMzQuMjciLCJUaW1lIjoxNDkxOTAwNTQ2fSx7Ik1pZCI6IjI4OWpmMmFmczMiLCJJUCI6IjE1NS40LjEzNC4yNyIsIlRpbWUiOjE0OTE5MDA2MzZ9XSwiVHJpYWxBY3RpdmF0aW9uIjpmYWxzZSwiTWF4Tm9PZk1hY2hpbmVzIjoxMCwiQWxsb3dlZE1hY2hpbmVzIjoiIiwiRGF0YU9iamVjdHMiOltdLCJTaWduRGF0ZSI6MTQ5NTAxOTc2Nn0=","signature":"SqPm8dtTdVBrXrmJzXer7qq6dvdQfctJxP8mar+RO9p8QABsgWWaX+uH7aOGMBd42eg+2Omorv7Ks6V7itRhXPeeq5qWoKuefd+pTsFagvqiu2N/E2Np8fpt51aqmiygdHLECo42nJwVD8JzlN67hnvJTgY7iyDWhG7qFK9Slk+kEJjjK/0J1pJYI6nOi+7sgBV7ZRca+7DmiP6OmOjNfySps6PdiB7QbiSis5f24Xmc5OYyRe3fzZmAueqF3eymBK19XhYFroWXeT4tcNsBNJsv+YfItovGbJysLx+K4ppltd2GNwEFQgtE3ILGOUj7EVbeQmQXg9m2c5MTPyk8iA==","result":0,"message":""}"#
    ).unwrap();

  let public_key = r#"<RSAKeyValue><Modulus>khbyu3/vAEBHi339fTuo2nUaQgSTBj0jvpt5xnLTTF35FLkGI+5Z3wiKfnvQiCLf+5s4r8JB/Uic/i6/iNjPMILlFeE0N6XZ+2pkgwRkfMOcx6eoewypTPUoPpzuAINJxJRpHym3V6ZJZ1UfYvzRcQBD/lBeAYrvhpCwukQMkGushKsOS6U+d+2C9ZNeP+U+uwuv/xu8YBCBAgGb8YdNojcGzM4SbCtwvJ0fuOfmCWZvUoiumfE4x7rAhp1pa9OEbUe0a5HL+1v7+JLBgkNZ7Z2biiHaM6za7GjHCXU8rojatEQER+MpgDuQV3ZPx8RKRdiJgPnz9ApBHFYDHLDzDw==</Modulus><Exponent>AQAB</Exponent></RSAKeyValue>"#;

  match license_key.has_valid_signature(public_key) {
    Ok(true) => { }
    _        => { println!("Signature check failed. Aborting!"); return; }
  }

  println!("Successfully activated license key: {}", license_key.Key.unwrap());
}
