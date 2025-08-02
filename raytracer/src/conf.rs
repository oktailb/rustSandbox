use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

use crate::types::common::Drawable;
use crate::types::{circle::Circle, square::Square};

pub fn build_drawables_from_json(rt_file: &str) -> Result<Vec<Box<dyn Drawable>>, String> {
    let file = File::open(rt_file).unwrap();
    let json_reader = BufReader::new(file);

    let values: Vec<Value> =
        serde_json::from_reader(json_reader).expect("JSON was not well-formatted");

    let mut drawables: Vec<Box<dyn Drawable>> = Vec::new();

    // Étape 2: Itérer sur chaque `Value` pour déterminer son type
    for v in values {
        // Étape 3: Extraire le champ "Classification"
        let classification = v["Common"]["Classification"]
            .as_str()
            .ok_or("Champ 'Classification' manquant ou invalide")?;

        // Étape 4: Utiliser une `match` pour créer la bonne struct
        match classification {
            "Circle" => {
                let cercle: Circle =
                    serde_json::from_value(v.clone()).map_err(|e| e.to_string())?;
                drawables.push(Box::new(cercle));
            }
            "Square" => {
                let carre: Square = serde_json::from_value(v.clone()).map_err(|e| e.to_string())?;
                drawables.push(Box::new(carre));
            }
            _ => {
                eprintln!("Type de classification inconnu : {}", classification);
            }
        }
    }

    Ok(drawables)
}
