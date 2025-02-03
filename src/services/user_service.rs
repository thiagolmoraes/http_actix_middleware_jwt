use anyhow::Result;
use crate::models::{LoginJson, UserSession};


pub async fn login(credential: LoginJson) -> Result<UserSession> {

    let email = &credential.email;
    let password = &credential.password;

    if email != "test@gmail.com" || password != "123Change" {
        return Err(anyhow::anyhow!("Invalid credentials"))
    }
    
    Ok(
        UserSession {
            email: email.to_string()
        }
    )

}