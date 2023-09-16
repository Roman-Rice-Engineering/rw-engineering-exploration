use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone)]
#[derive(Serialize, Deserialize)]
pub enum DisplayState{
    Hidden,
    Success{message: String},
    Failure{message: String}
}
