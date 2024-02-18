use colored::Colorize;
use reqwest;
use semver::Version;
use serde::Deserialize;
use serde_json::Value;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct PackageJson {
    pub dependencies: Option<BTreeMap<String, String>>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Option<BTreeMap<String, String>>,
    #[serde(rename = "peerDependencies")]
    pub peer_dependencies: Option<BTreeMap<String, String>>,
    #[serde(rename = "optionalDependencies")]
    pub optional_dependencies: Option<BTreeMap<String, String>>,
    #[serde(rename = "bundledDependencies")]
    pub bundled_dependencies: Option<BTreeMap<String, String>>,
}

pub fn read_package_json(path: PathBuf) -> Result<PackageJson, String> {
    println!("\nOpening {:?}", path);
    let mut file: File =
        File::open(&path).map_err(|e| format!("Error opening package.json: {}", e))?;

    println!("Reading package.json\n");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e: std::io::Error| format!("Error reading package.json: {}", e))?;

    serde_json::from_str(&contents).map_err(|e| format!("Error parsing package.json: {}", e))
}

pub async fn fetch_latest_version(
    package_name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let url: String = format!("https://registry.npmjs.org/{}/latest", package_name);
    let response: String = reqwest::get(url).await?.text().await?;
    let package_info: Value = serde_json::from_str(&response)?;

    Ok(package_info["version"]
        .as_str()
        .unwrap_or_default()
        .to_string())
}

pub async fn check_and_print_dependency_versions(
    dependency_type: &str,
    dependencies: Option<BTreeMap<String, String>>,
) {
    if let Some(deps) = dependencies {
        println!("{}", format!("{}:", dependency_type).blue().bold());
        println!("{}", "=".repeat(dependency_type.len() + 1).blue().bold());

        let fixed_width: usize = deps.keys().map(String::len).max().unwrap_or(0) + 10;
        let update_msg_fixed_width: usize = 27;

        for (name, specified_version_str) in deps {
            match fetch_latest_version(&name).await {
                Ok(latest_version_str) => {
                    let name_version: String = format!("{}: {}", name, specified_version_str);

                    match Version::parse(&latest_version_str) {
                        Ok(latest_version) => {
                            let up_to_date_msg: colored::ColoredString =
                                if let Ok(specified_version) = Version::parse(
                                    &specified_version_str
                                        .trim_start_matches('^')
                                        .trim_start_matches('~'),
                                ) {
                                    match (
                                        specified_version.major.cmp(&latest_version.major),
                                        specified_version.minor.cmp(&latest_version.minor),
                                        specified_version.patch.cmp(&latest_version.patch),
                                    ) {
                                        (Ordering::Less, _, _) => {
                                            "new major update available".bright_yellow()
                                        }
                                        (_, Ordering::Less, _) => {
                                            "new minor update available".bright_magenta()
                                        }
                                        (_, _, Ordering::Less) => {
                                            "new patch update available".bright_cyan()
                                        }
                                        _ => "up to date with latest".bright_green(),
                                    }
                                } else {
                                    "version format error".normal()
                                };

                            let padded_up_to_date_msg = format!(
                                "{:width$}",
                                up_to_date_msg,
                                width = update_msg_fixed_width
                            );

                            println!(
                                "{:width$} {} {}",
                                name_version.color("bright_white"),
                                padded_up_to_date_msg,
                                latest_version_str.color("bright_white"),
                                width = fixed_width
                            );
                        }
                        Err(_) => {
                            eprintln!(
                                "{}: Unable to parse latest version for {}.",
                                "Warning".bright_yellow(),
                                name.color("bright_white"),
                            );
                        }
                    }
                }
                Err(error) => eprintln!(
                    "Error fetching latest version for {}: {}",
                    name.color("bright_white"),
                    error
                ),
            }
        }
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
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
