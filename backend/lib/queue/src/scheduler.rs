use tokio_cron_scheduler::{Job, JobScheduler};

use crate::SendableJob;
use crate::prelude::Result;
use crate::sender::QueueSender;

/// A job that can be added to the task queue on a given schedule
#[derive(Clone, Debug)]
pub struct ScheduledJob {
    /// Cron-formatted schedule for the job task
    pub cron_schedule: String,
    /// Job task to run
    pub job_task: SendableJob,
}

impl ScheduledJob {
    /// Create a new [`ScheduledJob`] that will run based on the `cron_schedule`
    pub fn new(cron_schedule: impl Into<String>, job_task: SendableJob) -> Self {
        Self {
            cron_schedule: cron_schedule.into(),
            job_task,
        }
    }
}

/// Wrapper that adds [`ScheduledJob`] to the task queue on a schedule
pub struct QueueScheduler {
    scheduler: JobScheduler,
    broker_url: String,
}

impl QueueScheduler {
    /// Create a new [`QueueScheduler`] to schedule jobs for the task queue
    pub async fn new(broker_url: impl Into<String>) -> Result<Self> {
        Ok(Self {
            scheduler: JobScheduler::new().await?,
            broker_url: broker_url.into(),
        })
    }

    /// Add a [`ScheduledJob`] to the task queue
    pub async fn add_scheduled_job(&self, scheduled_job: ScheduledJob) -> Result<()> {
        let broker_url = self.broker_url.clone();
        log::info!(
            "Adding job \"{:?}\" with cron schedule \"{}\"",
            scheduled_job.job_task,
            scheduled_job.cron_schedule,
        );

        self.scheduler
            .add(Job::new_async(
                scheduled_job.cron_schedule,
                move |_uuid, _l| {
                    let broker_url = broker_url.clone();
                    let job_task = scheduled_job.job_task.clone();

                    Box::pin(async move {
                        match QueueSender::new(broker_url) {
                            Ok(sender) => match sender.send_job(&job_task).await {
                                Ok(_) => {
                                    log::info!("Job \"{:?}\" sent successfully", &job_task)
                                }
                                Err(err) => {
                                    log::error!("Error occurred while sending job: {err:?}")
                                }
                            },
                            Err(err) => {
                                log::error!(
                                    "Error occurred while connecting to job sender: {err:?}"
                                );
                            }
                        }
                    })
                },
            )?)
            .await?;

        Ok(())
    }

    /// Start the scheduler for the task queue
    pub async fn start_schedule(&self) -> Result<()> {
        log::info!("Starting queue scheduler...");

        Ok(self.scheduler.start().await?)
    }
}
