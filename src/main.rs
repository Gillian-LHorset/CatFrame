use crossterm;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers, poll, read};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::time::Duration;
use std::{thread, time};

#[derive(serde::Deserialize, PartialEq)]
struct Anim {
    cols: u8,
    rows: u8,
    frame_count: u32,
    frames: Vec<Vec<String>>,
}

#[derive(serde::Deserialize)]
struct Fichier {
    variants: HashMap<String, Anim>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let contenu: String = fs::read_to_string("src/ressources/LeChat-annimation.json")?;

    if contenu.is_empty() || contenu.len() == 0 {
        println!("The json is empty.");
        return Ok(());
    }

    let fichier: Fichier = serde_json::from_str(&contenu)?;

    let anims: &HashMap<String, Anim> = &fichier.variants;
    let mut anim: &Anim = &fichier.variants["le_chat"]; // set default anim

    loop {
        for frame in anim.frames.iter() {
            for row in frame.iter() {
                println!("{row}")
            }

            thread::sleep(time::Duration::from_millis(300));

            if poll(Duration::from_millis(100))? {
                if let Event::Key(key) = read()? {
                    match key.code {
                        KeyCode::Char('g') => {
                            if anim != anims.get("le_chonk").unwrap() {
                                anim = &anims.get("le_chonk").unwrap();
                                break;
                            }
                        }
                        KeyCode::Char('c') => {
                            if anim != anims.get("le_chat").unwrap() {
                                anim = &anims.get("le_chat").unwrap();
                                break;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(())
}
