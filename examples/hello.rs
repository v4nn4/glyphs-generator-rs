use relative_path::RelativePath;

extern crate glyphs_generator;
use glyphs_generator::{compute, initialize};
use std::fs;

fn main() {
    let example_dir = RelativePath::new("examples");

    // Initialize generator
    let parameters_path = example_dir.join("parameters_9ap.json").to_string();
    let parameters_json = fs::read_to_string(parameters_path)
        .expect("Failed to read parameters file")
        .to_string();
    initialize(parameters_json);

    // Compute glyphs
    let computable_path = example_dir.join("computable.json").to_string();
    let computable_json = fs::read_to_string(computable_path)
        .expect("Failed to read computable file")
        .to_string();
    let result = compute(computable_json);

    // Handle result
    if result.is_ok() {
        println!("{}", result.unwrap());
    } else {
        println!("An error occured")
    }
}
