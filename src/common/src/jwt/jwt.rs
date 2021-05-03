use chrono::{Local, Duration};


use tide::prelude::*;



#[derive(Deserialize, Debug)]
pub struct AuthUser {
    pub id: u64,
    pub username: String,
    pub email: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    username: String,
    uid: u64,
    exp: usize,
}


impl Into<AuthUser> for Claims {
    fn into(self) -> AuthUser {
        AuthUser {
            id: self.uid,
            username: self.username,
            email: self.sub,
        }
    }
}

impl From<&AuthUser> for Claims {
    fn from(u: &AuthUser) -> Self {
        Self {
            sub: u.email.to_string(),
            username: u.username.to_string(),
            uid: u.id,
            exp: (Local::now() + Duration::hours(24)).timestamp() as usize,
        }
    }
}
