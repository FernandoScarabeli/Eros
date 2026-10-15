fn main() {
    if std::env::args_os().any(|argument| argument == "--eros-provider-hook") {
        eros_service::provider_hooks::run();
        return;
    }
    let service_mode = std::env::args_os().any(|argument| argument == "--eros-service");
    if service_mode {
        if eros_service::serve().is_err() {
            std::process::exit(1);
        }
        return;
    }
    #[cfg(target_os = "linux")]
    if std::path::Path::new("/proc/driver/nvidia/version").exists()
        && std::env::var_os("WEBKIT_DISABLE_COMPOSITING_MODE").is_none()
    {
        std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
    }
    eros_desktop::run();
}
