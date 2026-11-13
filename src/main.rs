use std::fs;
use std::error::Error;
use std::collections::HashMap;

#[derive(serde::Deserialize)]
struct Anim {
    cols: u8,
    rows: u8,
    frames: Vec<Vec<String>>,
}

#[derive(serde::Deserialize)]
struct Fichier {
    variants: HashMap<String, Anim>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let contenu = fs::read_to_string("src/ressources/LeChat-annimation.json")?;    
    let fichier: Fichier = serde_json::from_str(&contenu)?;
    let anim = &fichier.variants["le_chat"];

    Ok(())
}