pub struct Authorizer {
    api_key: String,
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Error {
    InvalidApiKey { message: String },
}

impl Authorizer {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
        }
    }

    pub fn authorize(&self, subject: &str) -> Result<(), Error> {
        if subject == &self.api_key {
            Ok(())
        } else {
            Err(Error::InvalidApiKey {
                message: "Invalid api key.".to_string(),
            })
        }
    }
}

#[test]
fn test_valid_api_key_then_success() {
    let authorizer = Authorizer::new("valid-api-key");

    let result = authorizer.authorize("valid-api-key");

    assert!(result.is_ok())
}

#[test]
fn test_invalid_api_key_then_fail() {
    let authorizer = Authorizer::new("valid-api-key");

    let result = authorizer.authorize("invalid-api-key");

    assert!(result.is_err())
}
