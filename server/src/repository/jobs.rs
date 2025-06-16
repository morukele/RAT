use crate::error;
use crate::repository::Repository;
use common::entities::Job;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

impl Repository {
    pub async fn create_job(&self, db: &Pool<Postgres>, job: &Job) -> Result<(), error::Error> {
        const QUERY: &str = "INSERT INTO jobs \
            (id, created_at, executed_at, command, args, output, agent_id) \
            VALUES ($1, $2, $3, $4, $5, $6, $7)";

        match sqlx::query(QUERY)
            .bind(job.id)
            .bind(job.created_at)
            .bind(job.executed_at)
            .bind(&job.command)
            .bind(&job.args)
            .bind(&job.output)
            .bind(job.agent_id)
            .execute(db)
            .await
        {
            Err(err) => {
                log::error!("create_job: Inserting job: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }

    pub async fn update_job(&self, db: &Pool<Postgres>, job: &Job) -> Result<(), error::Error> {
        const QUERY: &str = "UPDATE jobs \
            SET executed_at = $1, output = $2 \
            WHERE id = $3";

        match sqlx::query(QUERY)
            .bind(job.executed_at)
            .bind(&job.output)
            .bind(job.id)
            .execute(db)
            .await
        {
            Err(err) => {
                log::error!("update_job: updating job: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }

    pub async fn find_job_by_id(
        &self,
        db: &Pool<Postgres>,
        job_id: Uuid,
    ) -> Result<Job, error::Error> {
        const QUERY: &str = "SELECT * FROM jobs WHERE id = $1";

        match sqlx::query_as::<_, Job>(QUERY)
            .bind(job_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                log::error!("find_job_by_id: finding job: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(error::Error::NotFound("Job, not found.".to_string())),
            Ok(Some(res)) => Ok(res),
        }
    }

    pub async fn find_job_for_agent(
        &self,
        db: &Pool<Postgres>,
        agent_id: Uuid,
    ) -> Result<Job, error::Error> {
        const QUERY: &str = "SELECT * FROM jobs WHERE agent_id = $1 AND output IS NULL LIMIT 1";
        match sqlx::query_as::<_, Job>(QUERY)
            .bind(agent_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                log::error!("find_job_where_output_is_null: finding job: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(error::Error::NotFound("Job not found".to_string())),
            Ok(Some(res)) => Ok(res),
        }
    }

    pub async fn find_all_jobs(&self, db: &Pool<Postgres>) -> Result<Vec<Job>, error::Error> {
        const QUERY: &str = "SELECT * FROM jobs ORDER BY created_at";

        match sqlx::query_as::<_, Job>(QUERY).fetch_all(db).await {
            Err(err) => {
                log::error!("find_all_jobs: finding all jobs: {}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }
}
