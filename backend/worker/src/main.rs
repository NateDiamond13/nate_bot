mod jobs;
mod listener;
mod prelude;
mod scheduler;

// use celery::beat::CronSchedule;
use celery::error::TaskError;
use celery::prelude::TaskResult;
use env_logger::Env;
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

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Load redis URL from the environment
    let env_vars = utils::get_env_variables();
    let broker_url = if env_vars.redis_url.starts_with("redis://") {
        env_vars.redis_url.clone()
    } else {
        format!("redis://{}", env_vars.redis_url)
    };
    let queue_name = "beat_queue";

    // Get listener and scheduler
    let listener = listener::get_listener(&broker_url, queue_name).await?;
    let mut scheduler = scheduler::get_scheduler(&broker_url, queue_name).await?;

    // Register tasks
    listener.register_task::<scraper_job>().await?;
    // scheduler.schedule_task(
    //     scraper_job::new(),
    //     CronSchedule::from_string("*/5 * * * *")?,
    // );

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
