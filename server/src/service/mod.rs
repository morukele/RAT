use crate::repository::Repository;
use sqlx::{Pool, Postgres};

pub mod agents;
pub mod job;

#[derive(Debug, Clone)]
pub struct Service {
    repo: Repository,
    db: Pool<Postgres>,
}

impl Service {
    pub fn new(db: Pool<Postgres>) -> Service {
        let repo = Repository {};
        Service { db, repo }
    }
}
