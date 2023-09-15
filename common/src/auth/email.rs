use serde::{Serialize, Deserialize};

const EMAIL_LENGTH_MAX: usize = 254;

#[derive(Debug, PartialEq, Eq, Clone)]
#[derive(Serialize, Deserialize)]
pub enum Email{
    Unconfirmed{email: String},
    Confirmed{email: String},
    AdminVerified{email: String}
}

impl Email {
    
    pub fn new(email: String) -> Email{
        Email::Unconfirmed { email }
    }

    pub fn get_email(self: &Self) -> &str{
        match self{
            Self::Unconfirmed { email } => email,
            Self::Confirmed { email } => email,
            Self::AdminVerified { email } => email
        }
    }

    pub fn is_valid(self: &Self) -> Result<(), &str>{
        if self.get_email().is_ascii() == false {
            return Err("email contains non ascii characters");
        }
        if self.get_email().len() >= EMAIL_LENGTH_MAX{
            return Err("email is too long")
        }
        Ok(())
    }
}

