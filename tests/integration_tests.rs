#[cfg(test)]
mod tests {
    use glyphs_generator::{compute, initialize};
    use std::fs;

    use relative_path::RelativePath;

    fn setup() {
        let tests_dir = RelativePath::new("tests");
        let parameters_path = tests_dir.join("parameters_9ap.json").to_string();
        let parameters_json = fs::read_to_string(parameters_path)
            .expect("Failed to read parameters file")
            .to_string();
        initialize(parameters_json);
    }

    #[test]
    fn test_computable_example() {
        setup();
        let tests_dir = RelativePath::new("tests");
        let computable_path = tests_dir.join("computable.json").to_string();
        let computable_json = fs::read_to_string(computable_path)
            .expect("Failed to read computable file")
            .to_string();
        let result = compute(computable_json);
        assert!(result.is_ok());
    }
}
