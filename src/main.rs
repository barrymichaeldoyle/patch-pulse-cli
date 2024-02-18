use colored::*;
use patch_pulse_cli::{check_and_print_dependency_versions, read_package_json, PackageJson};
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
