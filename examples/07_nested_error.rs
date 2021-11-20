use anyhow::{anyhow, Result};
use env_logger::{Builder, Env};
use tokio::time::{sleep, Duration};
use tokio_graceful_shutdown::{SubsystemHandle, Toplevel};

async fn subsys1(mut subsys: SubsystemHandle) -> Result<()> {
    subsys.start("Subsys2", subsys2);
    log::info!("Subsystem1 started.");
    subsys.on_shutdown_requested().await;
    log::info!("Shutting down Subsystem1 ...");
    sleep(Duration::from_millis(500)).await;
    log::info!("Subsystem1 stopped.");
    Ok(())
}

async fn subsys2(_subsys: SubsystemHandle) -> Result<()> {
    log::info!("Subsystem2 started.");
    sleep(Duration::from_millis(500)).await;

    Err(anyhow!("Subsystem2 threw an error."))
}

#[tokio::main]
async fn main() -> Result<()> {
    // Init logging
    Builder::from_env(Env::default().default_filter_or("debug")).init();

    // Create toplevel
    Toplevel::new()
        .start("Subsys1", subsys1)
        .catch_signals()
        .wait_for_shutdown(Duration::from_millis(1000))
        .await
}
