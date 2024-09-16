use std::collections::HashMap;
use std::fs::File;
use serde_json;
use std::io::Write;

use crate::trainee::Trainee;


pub fn add(name: String, skill_value: u8, mut trainees : &mut Vec<Trainee>) -> std::io::Result<()> {

    let n = name.clone();
    let t = Trainee{ name, skill_value };
    trainees.push(t);
    let serial = serde_json::to_string_pretty(&trainees).unwrap();
    let mut file = File::create("data.json")?;
    file.set_len(0)?;
    file.write_all(serial.as_bytes()).expect("error");

    println!("{}", serial);

    print!("Ajout de {} avec une compétence de {}/20.", n, skill_value);

    Ok(())
}
