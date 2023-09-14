use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Clone)]
#[derive(Serialize, Deserialize)]
pub struct UserTransmission{
    username: String,
    email: String,
    password: Option<String>,
}

impl UserTransmission{
    pub fn new(username: String, email: String, password: Option<String>) -> UserTransmission{
        UserTransmission{
            username,
            email,
            password
        }
    }
}

pub trait TransmitUser{
    fn as_user_transmission() -> UserTransmission;
}


