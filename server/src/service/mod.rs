use crate::{config, repository::Repository};
use sqlx::{Pool, Postgres};

pub mod agents;
pub mod job;

pub const ENCRYPTED_JOB_MAX_SIZE: usize = 512_000; // 512k
pub const ENCRYPTED_JOB_RESULT_MAX_SIZE: usize = 2_000_000; // 2MB

#[derive(Debug, Clone)]
pub struct Service {
    repo: Repository,
    db: Pool<Postgres>,
    config: config::Config,
}

impl Service {
    pub fn new(db: Pool<Postgres>, config: config::Config) -> Service {
        let repo = Repository {};
        Service { db, repo, config }
    }
}
