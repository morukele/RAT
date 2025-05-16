use crate::common::UpdateJobResult;
use crate::error;
use crate::repository::Repository;
use sqlx::{Pool, Postgres};

pub mod agents;
pub mod job;

#[derive(Debug)]
pub struct Service {
    repo: Repository,
    db: Pool<Postgres>,
}

impl Service {
    pub async fn update_job_result(&self, input: UpdateJobResult) -> Result<(), error::Error> {
        let mut job = self.repo.find_job_by_id(&self.db, input.job_id).await?;

        job.executed_at = Some(chrono::Utc::now());
        job.output = Some(input.output);
        self.repo.update_job(&self.db, &job).await
    }
}

impl Service {
    pub fn new(db: Pool<Postgres>) -> Service {
        let repo = Repository {};
        Service { db, repo }
    }
}
