use std::fs;
use std::error::Error;
use std::collections::HashMap;
use std::{thread, time};

#[derive(serde::Deserialize)]
struct Anim {
    cols: u8,
    rows: u8,
    frame_count: u32,
    frames: Vec<Vec<String>>
}

#[derive(serde::Deserialize)]
struct Fichier {
    variants: HashMap<String, Anim>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let contenu :String = fs::read_to_string("src/ressources/LeChat-annimation.json")?;
    let fichier :Fichier = serde_json::from_str(&contenu)?;
    let anim :&Anim = &fichier.variants["le_chat"]; // default anim

    while true {
        for frame in anim.frames.iter() {
            for row in frame.iter() {
                println!("{row}")
            }
            thread::sleep(time::Duration::from_millis(300));
        }
    }

    Ok(())
}