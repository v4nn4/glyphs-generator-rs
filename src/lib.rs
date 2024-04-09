mod generate;
mod glyph;
mod intersect;
mod parameters;
mod stroke;

#[cfg(test)]
mod tests;

use generate::GlyphGenerator;
use parameters::Parameters;
use serde::{Deserialize, Serialize};
use stroke::Stroke;
use wasm_bindgen::prelude::*;

use once_cell::sync::OnceCell;

static GENERATOR: OnceCell<GlyphGenerator> = OnceCell::new();

#[wasm_bindgen]
pub fn initialize(parameters_json: String) {
    let parameters: Parameters =
        serde_json::from_str(&parameters_json).expect("Error deserializing parameters JSON");
    let generator = GlyphGenerator::new(parameters);
    GENERATOR.set(generator).ok();
}

#[derive(Serialize, Deserialize)]
pub struct Computable {
    strokes: Vec<Stroke>,
}

#[wasm_bindgen]
pub fn compute(computable_json: String) -> Result<String, JsValue> {
    if GENERATOR.get().is_none() {
        // Returning an error if the JSON data is not loaded
        return Err(JsValue::from_str("Paremters JSON data not loaded"));
    }
    let generator = GENERATOR.get().unwrap();
    let computable: Computable =
        serde_json::from_str(&computable_json).expect("Error deserializing computable JSON");
    let strokes = computable.strokes;
    let result = generator.generate(&strokes, &strokes[0]);
    Ok(serde_json::to_string(&result).unwrap())
}
