mod activity;
pub mod client;
pub mod paths;
pub mod provider_hooks;
mod runtime;
mod server;
mod store;

pub fn serve() -> Result<(), String> {
    server::serve()
}
