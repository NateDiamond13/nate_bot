use tokio_cron_scheduler::{Job, JobScheduler};

use crate::SendableJob;
use crate::prelude::Result;
use crate::sender::QueueSender;

#[derive(Clone, Debug)]
pub struct ScheduledJob {
    pub cron_schedule: String,
    pub job_task: SendableJob,
}

impl ScheduledJob {
    pub fn new(cron_schedule: impl Into<String>, job_task: SendableJob) -> Self {
        Self {
            cron_schedule: cron_schedule.into(),
            job_task,
        }
    }
}

pub struct QueueScheduler {
    scheduler: JobScheduler,
    broker_url: String,
}

impl QueueScheduler {
    pub async fn new(broker_url: impl Into<String>) -> Result<Self> {
        Ok(Self {
            scheduler: JobScheduler::new().await?,
            broker_url: broker_url.into(),
        })
    }

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

    pub async fn start_schedule(&self) -> Result<()> {
        log::info!("Starting queue scheduler...");

        Ok(self.scheduler.start().await?)
    }
}
