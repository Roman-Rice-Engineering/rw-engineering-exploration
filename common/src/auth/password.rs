use serde::{Serialize, Deserialize};
#[cfg(feature = "bcrypt")]
use std::fmt;
#[cfg(feature = "bcrypt")]
use bcrypt::{hash, verify, DEFAULT_COST, BcryptError};

const PASSWORD_LENGTH_MIN: usize = 8;
const PASSWORD_LENGTH_MAX: usize = 15;

#[cfg(feature = "bcrypt")]
#[derive(Debug)]
pub enum PasswordHashError{
    AlreadyHashed,
    CannotHashBecauseNone,
    BcryptError{bcrypt_error: BcryptError}
}

#[cfg(feature = "bcrypt")]
impl fmt::Display for PasswordHashError{
    fn fmt(self: &Self, f: &mut fmt::Formatter) -> fmt::Result{
        match self {
            Self::AlreadyHashed => write!(f, "unable to hash password as it is already hashed"),
            Self::CannotHashBecauseNone => write!(f, "password is none, cannot hash"),
            Self::BcryptError { bcrypt_error } => write!(f, "{}", bcrypt_error)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
#[derive(Serialize, Deserialize)]
pub enum Password {
    #[default]
    None,
    Hash{hash: String},
    Plaintext{password: String} 
}


impl Password {
    pub fn new_plaintext(password: String) -> Password{
        Password::Plaintext { password }
    }

    pub fn is_valid_plaintext(self: &Self) -> Result<(), &'static str>{
        match self {
            Self::None => Err("password is none"),
            Self::Hash { hash: _ } => Err("password is hashed"),
            Self::Plaintext { password } => plaintext_password_is_valid(password.to_owned())
        }
    }

}
#[cfg(feature = "bcrypt")]
impl Password {
    pub fn hash(self: &mut Self) -> Result<(), PasswordHashError>{
        match self{
            Self::None => Err(PasswordHashError::CannotHashBecauseNone),
            Self::Hash { .. } => Err(PasswordHashError::AlreadyHashed),
            Self::Plaintext { password } => {
                let hashed_password = hash(password, DEFAULT_COST);
                match hashed_password {
                    Ok(resulting_hash) => {
                        *self = Password::Hash { hash: resulting_hash };
                        Ok(())
                    }
                    Err(e) => Err(PasswordHashError::BcryptError { bcrypt_error: e })
                }
            }
        }

    }

    pub fn verify(self: &Self, password: &str) -> Result<bool, BcryptError>{
        match self {
            Self::None => Ok(false),
            Self::Plaintext { .. } => Ok(false),
            Self::Hash { hash } => verify(password, hash)
        }
    }
    
}

fn plaintext_password_is_valid(password: String) -> Result<(), &'static str>{
    // Check that string is ascii only
    if password.is_ascii() == false {
        return Err("password contains non ascii characters");
    }

    // Check length of password
    if password.len() <= PASSWORD_LENGTH_MIN {
        return Err("password is too short");
    }
    if password.len() >= PASSWORD_LENGTH_MAX {
        return Err("password is too long");
    }

    // Check that password contains numbers
    {
        let mut contains_numbers: bool = false;

        for i in 0..=9 {
            if password.contains(&i.to_string()) {
                contains_numbers = true;
                break; 
            }
        }

        if contains_numbers == false {
            return Err("password does not contain numbers");
        }
    }

    // Check that password contains special characters
    {
        let mut contains_special = false;
        let special_characters = ['~', '!','@', '#', 
            '$', '%', '^', '&', '*', '(', ')', '-',
            '_', '=', '+', '<', '>', ',', '.', '?',
            '/', '|'];

        for i in special_characters{
            if password.contains(i){
                contains_special = true;
                break;
            }
        }

        if contains_special == false{
            return Err("password does not contain special characters");
        }
    }

    // Check that password contains uppercase and lowercase letters
    if password == password.to_lowercase() {
        return Err("password does not contain uppercase letters");
    }

    if password == password.to_uppercase() {
        return Err("password does not contain lowercase letters");
    }

    // Check password does not include banned characters
    {
        let disallowed_characters = [' ', '\t', '\n', '\0'];
        for i in disallowed_characters{
            if password.contains(i){
                return Err("password contains disallowed characters")
            }
        }
    }


    Ok(()) 
}

#[cfg(test)]
mod tests{
    use super::plaintext_password_is_valid;
    
    macro_rules! pw_test{
        ($func_name:ident, $good_pw:literal, $bad_pw:literal) => {
            #[test]
            fn $func_name(){
                let password: String = $bad_pw.to_owned();
                let check_result = plaintext_password_is_valid(password);
                assert_ne!(check_result, Ok(()));
                let password: String = $good_pw.to_owned();
                let check_result = plaintext_password_is_valid(password);
                assert_eq!(check_result, Ok(()));
            }
        };
    }
    
    // Test format | NAME | CONTROL | TEST |
    // CONTROL - valid password
    // TEST - similar to CONTROL but invalid
    pw_test!(password_non_ascii_is_not_valid, "helO!1oke", "helO!1oké");
    pw_test!(password_non_ascii_is_not_valid2, "veRy1!cooL", "veЯy1!cooL");
    pw_test!(password_should_have_special_characters, "helLoWod12!", "helLoWod124");
    pw_test!(password_should_have_special_characters2, "joE1998i$cool", "joE1998iScool");
    pw_test!(password_should_have_uppercase, "bestThing12!", "bestthing12!");
    pw_test!(password_with_spaces_is_not_valid, "$tupiD_Thing2", "$tupiD Thing2");
    pw_test!(password_with_null_is_not_valid, "$tupiD_Thing2", "$tupiD\0Thing2");
    pw_test!(password_with_newline_is_not_valid, "$tupiD_Thing2", "$tupiD\nThing2");
    pw_test!(password_with_tab_is_not_valid, "$tupiD_Thing2", "$tupiD\tThing2");
}
