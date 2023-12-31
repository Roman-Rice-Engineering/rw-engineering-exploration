
#[cfg(feature = "bcrypt")]
use bcrypt::BcryptError;
use serde::{Serialize, Deserialize};
use crate::auth::Email;
use crate::auth::Password;
#[cfg(feature = "bcrypt")]
use crate::auth::password::PasswordHashError;
#[cfg(feature = "database")]
use mongodb::bson::oid::ObjectId;

#[cfg(feature = "database")]
use super::person::PersonBackend;

#[derive(Debug, PartialEq, Eq, Clone)]
#[derive(Serialize, Deserialize)]
pub struct User{
    username: String,
    email: Email,
    password: Password,
}

#[cfg(feature = "database")]
#[derive(Debug, PartialEq, Eq, Clone)]
#[derive(Serialize, Deserialize)]
pub struct UserBackend{
    pub user: User,
    #[serde(rename = "_id")]
    #[serde(skip_serializing)]
    id: Option<ObjectId>, 
    person: Option<ObjectId>
}
#[cfg(feature = "database")]
impl UserBackend {
    pub fn get_id(self: &Self) -> Option<ObjectId> {
        self.id
    }
    pub fn put_id(self: &mut Self, id: ObjectId){
        self.id = Some(id);
    }

    pub fn put_person_backend(self: &mut Self, person: &PersonBackend){
        self.person = person.get_id();
    }
    pub fn to_user(self: Self) -> User{
        User{
            username: self.user.username,
            email: self.user.email,
            password: self.user.password
        }
    }
    pub fn from_user(user: User) -> UserBackend{
        UserBackend{
            user,
            id: None,
            person: None
        }
    }
    pub fn get_person_id(self: &Self) -> Option<ObjectId>{
        self.person
    }
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


