#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn ipc_smoke_test(name: String) -> String {
    atom_core::ipc_smoke_response(&name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ipc_smoke_test])
        .run(tauri::generate_context!())
        .expect("failed to run ATOM Studio");
}
