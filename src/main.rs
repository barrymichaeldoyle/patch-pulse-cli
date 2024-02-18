use colored::*;
use patch_pulse::{check_and_print_dependency_versions, read_package_json, PackageJson};
use std::env;

#[tokio::main]
async fn main() {
    std::env::set_var("CLICOLOR_FORCE", "1");

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
        "Thank you for using Patch Pulse CLI!".bright_green().bold()
    );
    println!(
        "{}",
        "If you find this tool helpful, consider supporting my work: https://www.buymeacoffee.com/barrycg"
            .bright_green().bold()
    );
    println!(
        "{}",
        "Found an issue or have a feature request? Please report it here: https://github.com/barrymichaeldoyle/patch-pulse-cli/issues"
            .bright_green().bold()
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
