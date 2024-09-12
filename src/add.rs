use std::collections::HashMap;
use std::fs::File;
use serde_json;
use std::io::Write;
use crate::Commands::Add;

pub fn add(name: String, skill_value: u8, mut list : &mut HashMap<String, u8>) -> std::io::Result<()> {

    let n = name.clone();
    list.insert(name, skill_value);

    let serial = serde_json::to_string(&list).unwrap();
    let mut file = File::create("data.json").unwrap();
    file.write_all(serial.as_bytes())?;

    println!("{}", serial);

    print!("Ajout de {} avec une compétence de {}/20.", n, skill_value);

    Ok(())
}
