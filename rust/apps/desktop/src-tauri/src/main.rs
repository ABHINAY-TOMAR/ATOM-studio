#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use tauri::State;
use tracing::{error, info};

#[tauri::command]
fn ipc_smoke_test(name: String, core: State<'_, atom_core::ApplicationCore>) -> String {
    core.ipc_smoke_response(&name)
}

fn run() -> Result<(), Box<dyn Error>> {
    let core = match atom_core::ApplicationCore::initialize() {
        Ok(core) => core,
        Err(error) => {
            log_initialization_failure(&error);
            return Err(Box::new(error));
        }
    };
    let _logging_guard =
        atom_core::logging::initialize(core.paths().logs_dir(), core.config().log_level)?;

    info!(
        log_level = core.config().log_level.as_str(),
        "ATOM Studio starting"
    );
    let result = tauri::Builder::default()
        .manage(core)
        .invoke_handler(tauri::generate_handler![ipc_smoke_test])
        .run(tauri::generate_context!());

    if let Err(error) = &result {
        error!(error = %error, "ATOM Studio exited with an application error");
    } else {
        info!("ATOM Studio stopped");
    }

    result?;
    Ok(())
}

fn log_initialization_failure(error: &atom_core::error::CoreError) {
    let paths = match atom_core::paths::AppPaths::discover() {
        Ok(paths) => paths,
        Err(_) => {
            eprintln!(
                "ATOM Studio startup diagnostics unavailable ({})",
                error.diagnostic_code()
            );
            return;
        }
    };
    if let Err(path_error) = paths.ensure() {
        eprintln!(
            "ATOM Studio startup diagnostics unavailable ({}; {:?})",
            path_error.diagnostic_code(),
            path_error.io_error_kind()
        );
        return;
    }

    match atom_core::logging::initialize(paths.logs_dir(), atom_core::config::LogLevel::Error) {
        Ok(_guard) => error!(
            error_code = error.diagnostic_code(),
            io_error_kind = ?error.io_error_kind(),
            "ATOM Studio initialization failed"
        ),
        Err(logging_error) => {
            eprintln!("ATOM Studio startup log file unavailable: {logging_error}");
        }
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("ATOM Studio could not start: {error}");
        std::process::exit(1);
    }
}
