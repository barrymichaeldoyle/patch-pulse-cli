use colored::{ColoredString, Colorize};
use reqwest::get;
use semver::Version;
use serde_json::{from_str, Value};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::error::Error;

pub async fn fetch_latest_version(package_name: &str) -> Result<String, Box<dyn Error>> {
    let url: String = format!("https://registry.npmjs.org/{}/latest", package_name);
    let response: String = get(url).await?.text().await?;
    let package_info: Value = from_str(&response)?;

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
                            let up_to_date_msg: ColoredString = if let Ok(specified_version) =
                                Version::parse(
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
