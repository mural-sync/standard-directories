use std::process::ExitCode;

fn main() -> ExitCode {
    if let Err(e) = print_directories() {
        eprintln!("error: {}", e);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn print_directories() -> Result<(), standard_directories::Error> {
    const ORGANIZATION_NAME: &str = "Mural Sync";
    const APP_NAME: &str = "Standard Directories Cli";

    println!(
        "Config directory: {}",
        standard_directories::config_path(ORGANIZATION_NAME.to_string(), APP_NAME.to_string())?
            .display()
    );

    println!(
        "Data directory: {}",
        standard_directories::data_path(ORGANIZATION_NAME.to_string(), APP_NAME.to_string())?
            .display()
    );

    Ok(())
}
