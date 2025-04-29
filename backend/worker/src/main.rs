#![allow(non_local_definitions)]

mod jobs;
mod listener;
mod prelude;
mod scheduler;

use celery::beat::CronSchedule;
use celery::error::TaskError;
use celery::prelude::TaskResult;
use listener::Listenable;
use prelude::Result;
use scheduler::Schedulable;
use tokio::signal;

#[celery::task]
async fn scraper_job() -> TaskResult<()> {
    jobs::patch_scraper::execute_job()
        .await
        .map_err(|e| TaskError::UnexpectedError(e.to_string()))
}

const APP_NAME: &str = "celery";
const QUEUE_NAME: &str = "beat_queue";

#[tokio::main]
async fn main() -> Result<()> {
    // Register logger
    utils::init_logger();

    // Load redis URL from the environment
    let env_vars = utils::get_env_variables();
    let broker_url = if env_vars.redis_url.starts_with("redis://") {
        env_vars.redis_url.clone()
    } else {
        format!("redis://{}", env_vars.redis_url)
    };

    // Get listener and scheduler
    let listener = listener::get_listener(APP_NAME, &broker_url, QUEUE_NAME).await?;
    let mut scheduler = scheduler::get_scheduler(APP_NAME, &broker_url, QUEUE_NAME).await?;

    // Register tasks
    listener.register_task::<scraper_job>().await?;
    scheduler.schedule_task(
        scraper_job::new(),
        CronSchedule::from_string("*/15 * * * *")?, // Run every 15 min
    );

    // Start listener and scheduler
    tokio::select! {
        res = listener.start_listen() => {
            if let Err(error) = res {
                dbg!(error);
            }
            eprintln!("Listener has shutdown");
        }
        res = scheduler.start_schedule() => {
            if let Err(error) = res {
                dbg!(error);
            }
            eprintln!("Scheduler has shutdown");
        }
        _ = signal::ctrl_c() => {
            eprintln!("Ctrl-C received, shutting down");
        }
    }
    Ok(())
}
