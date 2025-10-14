mod patch_scraper;

use database::DbPool;
use patch_scraper::patch_scraper_job;
use queue::{QueueScheduler, ScheduledJob, SendableJob};

use crate::prelude::{QueueError, QueueResult, Result};

/// Register job tasks with the scheduler (in UTC)
pub async fn register_scheduled_jobs(scheduler: &QueueScheduler) -> Result<()> {
    for scheduled_job in get_scheduled_jobs() {
        scheduler.add_scheduled_job(scheduled_job).await?;
    }

    Ok(())
}

/// Get a list of all jobs to schedule
fn get_scheduled_jobs() -> Vec<ScheduledJob> {
    // Patch Scraper - Runs every 15 min
    let patch_schedule = "0 */15 * * * *";

    vec![ScheduledJob::new(patch_schedule, SendableJob::PatchScraper)]
}

/// Handle jobs from the queue
pub async fn job_handler(job_task: &SendableJob) -> QueueResult<bool> {
    log::info!("Starting \"{job_task:?}\" job...");

    match run_job_helper(job_task).await {
        Ok(success) => {
            log::info!("Job \"{job_task:?}\" succeeded with result: {success}");
            Ok(success)
        }
        Err(err) => {
            log::error!("Job \"{job_task:?}\" failed with error: {err:?}");
            Err(QueueError::FailedJobTask(
                format!("{job_task:?}"),
                err.to_string(),
            ))
        }
    }
}

/// Wrapper to run a job
async fn run_job_helper(job_task: &SendableJob) -> Result<bool> {
    let env_vars = utils::get_config_safe()?;
    let db_pool = DbPool::new(&env_vars.database_url).await?;

    match job_task {
        SendableJob::PatchScraper => patch_scraper_job(&db_pool, &env_vars).await,
    }
}

#[cfg(test)]
mod tests {
    use croner::errors::CronError;
    use croner::parser::{CronParser, Seconds};
    use test_log::test;

    use crate::jobs::get_scheduled_jobs;

    #[test]
    fn test_get_scheduled_jobs() -> Result<(), CronError> {
        let cron_parser = CronParser::builder()
            .seconds(Seconds::Required)
            .dom_and_dow(true)
            .build();

        for job in get_scheduled_jobs() {
            if let Err(err) = cron_parser.parse(&job.cron_schedule) {
                log::error!(
                    "Error parsing \"{:?}\" job cron schedule \"{}\": {:?}",
                    job.job_task,
                    job.cron_schedule,
                    err
                );
                return Err(err);
            }
        }

        Ok(())
    }
}
