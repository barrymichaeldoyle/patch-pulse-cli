pub mod models;
pub mod utils;

use crate::models::package_json::PackageJson;
use crate::utils::read_package_json::read_package_json;
use colored::*;
use patch_pulse::check_and_print_dependency_versions;
use std::env;

#[tokio::main]
async fn main() {
    env::set_var("CLICOLOR_FORCE", "1");

    let mut package_json_path = match env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Failed to get current directory: {}", e);
            return;
        }
    };
    package_json_path.push("package.json");

    match read_package_json(package_json_path) {
        Ok(package_json) => print_package_json(package_json).await,
        Err(error) => eprintln!("{}", error.red()),
    }

    println!();
    println!(
        "{}",
        "Thank you for using Patch Pulse CLI!"
            .bright_yellow()
            .bold()
    );
    println!();
    println!(
        "{} {}",
        "If you find this tool helpful, consider supporting my work:"
            .bright_green()
            .bold(),
        "https://www.buymeacoffee.com/barrycg".bright_white().bold()
    );
    println!();
    println!(
        "{} {}",
        "Link to Patch Pulse Slack Bot for notifications on updated packages:"
            .bright_cyan().bold(),
        "https://slack.com/oauth/v2/authorize?client_id=180374136631.6017466448468&scope=chat:write,commands,incoming-webhook".bright_white().bold()
    );
    println!();
    println!(
        "{} {}",
        "Found an issue or have a feature request? Please report it here:"
            .bright_magenta()
            .bold(),
        "https://github.com/barrymichaeldoyle/patch-pulse-cli/issues"
            .bright_white()
            .bold()
    );
    println!();
}

async fn print_package_json(package_json: PackageJson) {
    check_and_print_dependency_versions("Dependencies", package_json.dependencies).await;
    check_and_print_dependency_versions("Dev Dependencies", package_json.dev_dependencies).await;
    check_and_print_dependency_versions("Peer Dependencies", package_json.peer_dependencies).await;
    check_and_print_dependency_versions(
        "Optional Dependencies",
        package_json.optional_dependencies,
    )
    .await;
    check_and_print_dependency_versions("Bundled Dependencies", package_json.bundled_dependencies)
        .await;
}
