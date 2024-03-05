use crate::models::package_json::PackageJson;

use serde_json::from_str;
use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;

pub fn read_package_json(path: PathBuf) -> Result<PackageJson, String> {
    println!("\nOpening {:?}", path);
    let mut file: File =
        File::open(&path).map_err(|e| format!("Error opening package.json: {}", e))?;

    println!("Reading package.json\n");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e: Error| format!("Error reading package.json: {}", e))?;

    from_str(&contents).map_err(|e| format!("Error parsing package.json: {}", e))
}
