use crate::entities::Job;
use crate::error;
use crate::repository::Repository;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

impl Repository {
    pub async fn create_job(&self, db: &Pool<Postgres>, job: &Job) -> Result<(), error::Error> {
        const QUERY: &str = "INSERT INTO jobs \
            (id, encrypted_job, ephemeral_public_key, nonce, signature, agent_id) \
            VALUES ($1, $2, $3, $4, $5, $6)";

        match sqlx::query(QUERY)
            .bind(job.id)
            .bind(&job.encrypted_job)
            .bind(&job.ephemeral_public_key)
            .bind(&job.nonce)
            .bind(&job.signature)
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
            SET encrypted_result = $1, result_ephemeral_public_key = $2,\
                result_nonce = $3, result_signature = $4
            WHERE id = $5";

        match sqlx::query(QUERY)
            .bind(&job.encrypted_result)
            .bind(&job.result_ephemeral_public_key)
            .bind(&job.result_nonce)
            .bind(&job.result_signature)
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

    pub async fn delete_job(&self, db: &Pool<Postgres>, job_id: Uuid) -> Result<(), error::Error> {
        const QUERY: &str = "DELETE FROM jobs WHERE id = $1";

        sqlx::query(QUERY).bind(job_id).execute(db).await?;
        Ok(())
    }
}
