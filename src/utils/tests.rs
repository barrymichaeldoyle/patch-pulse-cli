#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn test_read_package_json_valid() {
        let test_path = Path::new("tests/data/valid_package.json"); // Adjust the path as necessary
        let result = read_package_json(test_path.to_path_buf());
        assert!(result.is_ok());

        let package_json = result.unwrap();
        assert!(package_json.dependencies.is_some());
    }
}
