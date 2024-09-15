use std::collections::HashMap;
use std::fs::File;

use crate::trainee::Trainee;


pub fn load (file: &File) -> Result<Vec<Trainee>, std::io::Error >{

    let trainees : Vec<Trainee> = serde_json::from_reader(file)?;
    Ok(trainees)
}