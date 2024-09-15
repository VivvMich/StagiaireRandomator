use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Trainee {
    pub name: String,
    pub skill_value: u8
}
