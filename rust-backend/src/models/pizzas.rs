use serde::{Deserialize, Serialize};    
use validator::Validate;

#[derive(Validate, Deserialize,Serialize)]
pub struct Pizza {
    #[validate(length(min = 3, message = "validate_name"))]
    pub name: String,
}