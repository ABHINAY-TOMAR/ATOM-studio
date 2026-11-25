#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use tracing::{error, info};

#[tauri::command]
fn ipc_smoke_test(name: String) -> String {
    atom_core::ipc_smoke_response(&name)
}

fn run() -> Result<(), Box<dyn Error>> {
    let core = atom_core::ApplicationCore::initialize()?;
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

fn main() {
    if let Err(error) = run() {
        eprintln!("ATOM Studio could not start: {error}");
        std::process::exit(1);
    }
}
