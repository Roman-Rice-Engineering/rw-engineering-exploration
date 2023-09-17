
#[cfg(feature = "bcrypt")]
use bcrypt::BcryptError;
use serde::{Serialize, Deserialize};
use crate::auth::Email;
use crate::auth::Password;
#[cfg(feature = "bcrypt")]
use crate::auth::password::PasswordHashError;

#[derive(Debug, PartialEq, Eq, Clone)]
#[derive(Serialize, Deserialize)]
pub struct User{
    username: String,
    email: Email,
    password: Password,
}



impl User {
    pub fn get_username(self: &Self) -> &str{
        &self.username
    }

    pub fn new(username: String, email: Email, password: Password) -> User{
        User{
            username,
            email,
            password
        }
    }
    pub fn washout_password(self: Self) -> Self{
        Self { 
            username: self.username,
            email: self.email,
            password: Password::None
        }
    }
    pub fn washout(self: Self) -> Self{
        Self{
            username: self.username,
            email: self.email,
            password: Password::None
        }
        
    }

    pub fn get_plaintext_password(self: &Self) -> Option<String>{
        match &self.password {
            Password::None => None,
            Password::Hash { .. } => None,
            Password::Plaintext { password } => Some(password.to_owned())
        }
    }
    pub fn get_hased_password(self: &Self) -> Option<String>{
        match &self.password {
            Password::None => None,
            Password::Plaintext { .. } => None,
            Password::Hash { hash } => Some(hash.to_owned())
        }
    }


    pub fn is_valid_with_plaintext_password(self: &Self) -> Result<(), &str> {
        if self.username.is_ascii() == false {
            return Err("username contains non ascii characters");
        }
        match self.email.is_valid(){
            Ok(()) => (),
            Err(e) => return Err(e)
        }
        match self.password.is_valid_plaintext() {
            Ok(()) => (),
            Err(e) => return Err(e)
        }
        Ok(())
    }
}

#[cfg(feature = "bcrypt")]
impl User {

    pub fn hash(self: &mut Self) -> Result<(), PasswordHashError>{
        self.password.hash()
    }

    pub fn verify(self: &Self, password: &str) -> Result<bool, BcryptError>{
        self.password.verify(password)
    }
    
}


