use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use jsonwebtoken::errors::Error;
use chrono::{Local, Duration};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaim {
    pub company: String,
    pub sub: String,
    pub exp: i64,
}

impl UserClaim {
    // Creating Token
    pub fn create_token(email_claim: String) -> Result<String, Error> {
        let secret = "my_secret";

        // Defining expiration time
        let expiration: i64 = (Local::now().naive_local() + Duration::hours(1)).timestamp_millis();

        let claim = UserClaim {
            company: "api_rust".to_string(),
            sub: email_claim,
            exp: expiration,
        };

        // Token Codification
        let token = encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?;

        Ok(token)
    }

    // Validação do Token
    pub fn validate_token(token: &str) -> Result<UserClaim, Error> {
        let secret = "my_secret";

        // Try decode token
        let token_data = decode::<UserClaim>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        );

        // if Token is valid, verify if it has expired
        match token_data {
            Ok(data) => {
                // if token is expired, return error
                if data.claims.exp < Local::now().naive_local().timestamp_millis() {
                    return Err(Error::from(jsonwebtoken::errors::ErrorKind::ExpiredSignature));
                }
                Ok(data.claims)
            }
            Err(e) => Err(e), // if token is invalid, return error
        }
    }
}

