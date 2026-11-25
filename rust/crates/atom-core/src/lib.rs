pub fn ipc_smoke_response(name: &str) -> String {
    format!("Hello, {name} — Rust IPC is working.")
}

#[cfg(test)]
mod tests {
    use super::ipc_smoke_response;

    #[test]
    fn response_includes_requested_name() {
        assert_eq!(
            ipc_smoke_response("ATOM Studio"),
            "Hello, ATOM Studio — Rust IPC is working."
        );
    }
}
