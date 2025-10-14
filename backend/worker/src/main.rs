mod jobs;
mod prelude;

use jobs::job_handler;
use prelude::Result;
use queue::{QueueListener, QueueScheduler};
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    // Register logger
    utils::init_logger();

    // Load environment variables
    let env_vars = utils::get_config_safe()?;
    let broker_url = env_vars.redis_url;

    // Get the job queue scheduler and listener
    let scheduler = QueueScheduler::new(&broker_url).await?;
    let listener = QueueListener::new(&broker_url)?;

    // Register scheduled job tasks and start scheduler
    jobs::register_scheduled_jobs(&scheduler).await?;
    scheduler.start_schedule().await?;

    // Start listener and wait for shutdown signal
    tokio::select! {
        res = listener.start_listen(job_handler) => {
            if let Err(err) = res {
                log::error!("{err:?}");
            }
            log::info!("Listener has shutdown");
        }
        _ = signal::ctrl_c() => {
            log::info!("Ctrl-C received, shutting down");
        }
    }

    Ok(())
}
