use crate::entities::Job;
use crate::service::Service;
use crate::{common, error};
use chrono::Utc;
use sqlx::types::Json;
use uuid::Uuid;

impl Service {
    pub async fn find_job(&self, job_id: Uuid) -> Result<Job, error::Error> {
        self.repo.find_job_by_id(&self.db, job_id).await
    }

    pub async fn list_jobs(&self) -> Result<Vec<Job>, error::Error> {
        self.repo.find_all_jobs(&self.db).await
    }

    pub async fn get_agent_job(&self, agent_id: Uuid) -> Result<Option<Job>, error::Error> {
        let mut agent = self.repo.find_agent_by_id(&self.db, agent_id).await?;
        agent.last_seen_at = Utc::now();
        // Ignore results as an error is not important
        let _ = self.repo.update_agent(&self.db, &agent).await;

        match self.repo.find_job_for_agent(&self.db, agent_id).await {
            Ok(job) => Ok(Some(job)),
            Err(error::Error::NotFound(_)) => Ok(None),
            Err(err) => Err(err),
        }
    }

    pub async fn update_job_result(
        &self,
        input: common::UpdateJobResult,
    ) -> Result<(), error::Error> {
        let mut job = self.repo.find_job_by_id(&self.db, input.job_id).await?;

        job.executed_at = Some(Utc::now());
        job.output = Some(input.output);
        self.repo.update_job(&self.db, &job).await
    }

    pub async fn create_job(&self, input: common::CreateJob) -> Result<Job, error::Error> {
        println!("creating job: {:?}", input);
        let command = input.command.trim();
        let mut command_with_args: Vec<String> =
            command.split_whitespace().map(|s| s.to_owned()).collect();

        if command_with_args.is_empty() {
            return Err(error::Error::InvalidArgument(
                "Command is not valid".to_string(),
            ));
        }

        let command = command_with_args.remove(0);

        let now = Utc::now();
        let new_job = Job {
            id: Uuid::new_v4(),
            created_at: now,
            executed_at: None,
            command,
            args: Json(command_with_args),
            output: None,
            agent_id: input.agent_id,
        };

        self.repo.create_job(&self.db, &new_job).await?;

        Ok(new_job)
    }
}
