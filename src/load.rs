use std::collections::HashMap;
use std::fs::{create_dir, File, OpenOptions};

use crate::trainee::Trainee;

pub fn load (mut file: &File) -> Result<Vec<Trainee>, std::io::Error >{

    let trainees : Vec<Trainee> = serde_json::from_reader(file)?;
    Ok(trainees)
}