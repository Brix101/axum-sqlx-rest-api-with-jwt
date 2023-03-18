use std::{sync::Arc, time::SystemTime};

use async_trait::async_trait;
// use mockall::automock;
use sqlx::{types::time::OffsetDateTime, FromRow};

use super::user_repository::UserEntity;

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynSessionsRepository = Arc<dyn SessionsRepository + Send + Sync>;

#[async_trait]
pub trait SessionsRepository {
    async fn create_session(
        &self,
        user_id: &i64,
        user_agent: &str,
        exp: &OffsetDateTime,
    ) -> anyhow::Result<SessionEntity>;

    async fn get_user_session_by_id(&self, id: i64) -> anyhow::Result<UserEntity>;
}

#[derive(FromRow)]
pub struct SessionEntity {
    pub id: i64,
    pub user_id: i64,
    pub exp: OffsetDateTime,
    pub user_agent: String,
}

impl Default for SessionEntity {
    fn default() -> Self {
        SessionEntity {
            id: 1,
            user_id: 1,
            exp: OffsetDateTime::from(SystemTime::now()),
            user_agent: String::from("stub user_agent"),
        }
    }
}