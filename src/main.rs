use crossterm;
use crossterm::event::{Event, KeyCode, KeyModifiers, poll, read};
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;
use std::{thread, time};
use std::io::stdout;
use crossterm::{cursor, execute, terminal};
use crossterm::cursor::MoveTo;

#[derive(serde::Deserialize, PartialEq)]
struct Anim {
    cols: u16,
    rows: u16,
    frame_count: u32,
    frames: Vec<Vec<String>>,
}

#[derive(serde::Deserialize)]
struct Fichier {
    variants: HashMap<String, Anim>,
}

fn main() -> Result<(), Box<dyn Error>> {
    terminal::enable_raw_mode()?;
    execute!(stdout(), cursor::Hide)?;

    const ANIMS_JSON: &str = include_str!("ressources/LeChat-annimation.json");

    if ANIMS_JSON.is_empty() || ANIMS_JSON.len() == 0 {
        println!("The json is empty.");
        return Ok(());
    }

    let fichier: Fichier = serde_json::from_str(&ANIMS_JSON)?;

    let anims: &HashMap<String, Anim> = &fichier.variants;
    let mut anim: &Anim = &fichier.variants["le_chat"]; // set default anim

    loop {
        for frame in anim.frames.iter() {
            clearscreen::clear().unwrap();
            let (w, h) = terminal::size()?;
            let mut i = 0;
            for row in frame.iter() {
                execute!(stdout(), MoveTo((w / 2) - (anim.cols / 2), ((h / 2) - (anim.rows / 2)) + i))?;
                println!("{row}");
                i += 1;
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
                            if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                                return Ok(());
                            }
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
}
