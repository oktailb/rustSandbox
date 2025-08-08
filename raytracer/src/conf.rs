use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use crate::types::common::Drawable;

type DrawableFactory = fn(Value) -> Result<Box<dyn Drawable>, String>;

fn build_drawable_registry() -> HashMap<&'static str, DrawableFactory> {
    let mut registry: HashMap<&'static str, DrawableFactory> = HashMap::new();

    // Enregistrement de chaque type
    registry.insert("Circle", crate::types::circle::factory);
    registry.insert("Square", crate::types::square::factory);
    registry.insert("LightSource", crate::types::lightsource::factory);
    registry.insert("Camera", crate::types::camera::factory);

    registry
}

pub fn build_drawables_from_json(rt_file: &str) -> Result<(Vec<Box<dyn Drawable>>, Vec<Box<dyn Drawable>>, Vec<Box<dyn Drawable>>), String> {
    let file = File::open(rt_file).expect("Could not open file.");
    let json_reader = BufReader::new(file);
    
    let registry = build_drawable_registry();
    let values: Vec<Value> =
        serde_json::from_reader(json_reader).expect("JSON was not well-formatted");

    let mut drawables: Vec<Box<dyn Drawable>> = Vec::new();
    let mut cameras: Vec<Box<dyn Drawable>> = Vec::new();
    let mut lightsources: Vec<Box<dyn Drawable>> = Vec::new();

    for v in values {
        let classification = v["Common"]["Classification"]
            .as_str()
            .ok_or("Champ 'Classification' manquant ou invalide")?
            .to_string();
    
        if let Some(factory_fn) = registry.get(classification.as_str()) {
            let drawable = factory_fn(v)?;
            
            if classification == "Camera"{
                cameras.push(drawable);
	    }
	    else if classification == "LightSource" {
                lightsources.push(drawable);
            } else {
                drawables.push(drawable);
            }
        } else {
            eprintln!("Type de classification inconnu : {}", classification);
        }
    }

    Ok((drawables, cameras, lightsources))
}
