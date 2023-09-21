use std::{usize, ops::Deref};
use serde::{Serialize, Deserialize};
use base64::{engine::general_purpose::STANDARD, Engine};


#[derive(Debug, PartialEq, Eq, Clone, Default)]
#[derive(Serialize, Deserialize)]
pub struct Base64File{
    data: String,
    name: String
}

impl Base64File {
    pub fn new_from_base64_string(data: String, name: String) -> Base64File{
        Base64File{
            data,
            name
        }
    }

    pub fn new_from_vec_u8(data: &[u8], name: String) -> Base64File{
        Base64File{
            data: STANDARD.encode(data),
            name
        }
    }

    pub fn is_valid(self: &Self) -> bool{
        match STANDARD.decode(self.data.deref()){
            Ok(_) => true,
            Err(_) => false
        }
    }

    pub fn get_data_base64_str(self: &Self) -> &str{
        &self.data
    }
    pub fn get_data_vec_u8(self: &Self) -> Result<Vec<u8>, base64::DecodeError>{
        STANDARD.decode(self.data.deref())
    }
    pub fn get_name(self: &Self) -> &str{
        &self.name
    }


}

pub fn vec_base_64_file_is_valid(all_data: Vec<Base64File>) -> bool{
    for ele in all_data.iter(){
        if ele.is_valid() == false{
            return false;
        }
    }

    true
}
/*  This was a test version of the code, is no longed used
*
*
#[derive(Debug, PartialEq, Eq, Clone, Default)]
#[derive(Serialize, Deserialize)]
pub struct Base64FileSet{

    // Data - File data as base 64 encoded string
    //
    //        | Data | Mime Type |
    data: Vec<(String, String)>
}

impl Base64FileSet{
    pub fn new() -> Base64FileSet{
        Base64FileSet { data: Vec::new() }
    }

    pub fn verify_integrity(self: &Self) -> bool{
        for (ele, _) in self.data.iter(){
            match STANDARD.decode(ele){
                Ok(_) => (),
                Err(_) => return false
            }
        }

        // Return true if there are no problems detected 
        true
    }
    pub fn push_vec_u8(self: &mut Self, data: Vec<u8>, mime_type: String){
        self.data.push((STANDARD.encode(data), mime_type));
    }
    pub fn push_base64_string(self: &mut Self, data: String, mime_type: String){
        self.data.push((data, mime_type));
    }
    pub fn get_data_base64_str(self: &Self, index: usize) -> &str{
        let (data, _mime_type) = &self.data[index];
        data
    }
    pub fn get_data_vec_u8(self: &Self, index: usize) -> Result<Vec<u8>, base64::DecodeError>{
        let (data, _mime_type) = &self.data[index];
        STANDARD.decode(data)
    }
    pub fn get_name(self: &Self, index: usize) -> &str{
        let (_, name) = &self.data[index];
        name
    }
}
*/
