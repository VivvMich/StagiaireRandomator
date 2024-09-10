use std::collections::HashMap;

pub fn add(name: String, skill_value: u8, mut list : &mut HashMap<String, u8>) {
    let n = name.clone();
    list.insert(name, skill_value);
    print!("Ajout de {} avec une compétence de {}/20.", n, skill_value);
}