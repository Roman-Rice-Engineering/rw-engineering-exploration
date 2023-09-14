use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum DisplayState{
    Hidden,
    Success{message: String},
    Failure{message: String}
}
