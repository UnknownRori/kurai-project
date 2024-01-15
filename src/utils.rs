/// Setup the application tracing and setting up environment variable
pub fn setup() -> Result<(), color_eyre::Report> {
    let _ = env("RUST_LIB_BACKTRACE", "1");
    let _ = env("RUST_LOG", "debug");

    color_eyre::install()?;

    tracing_subscriber::fmt::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(true)
        .pretty()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tracing::debug!("Tracing is initialized!");

    Ok(())
}

/// Get the value from [`std::env::var`] variable if, it's doesn't exist set it into default value
/// and return
#[cfg(target_arch = "wasm32")]
pub fn env(_key: &str, default: &str) -> String {
    default.to_string()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn env(key: &str, default: &str) -> String {
    match std::env::var(key) {
        Ok(val) => val,
        Err(_err) => {
            std::env::set_var(key, default);
            default.to_string()
        }
    }
}
