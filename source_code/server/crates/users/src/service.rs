//! User service

use ug_core::{Error, Result, UserId};
use ug_db::{repositories::UserRepository, DbState};

pub struct UserService {
    db: DbState,
}

impl UserService {
    pub fn new(db: DbState) -> Self {
        Self { db }
    }

    pub async fn get_user(&self, id: UserId) -> Result<Option<ug_db::repositories::user_repo::UserRow>> {
        UserRepository::find_by_id(&self.db.pg, id)
            .await
            .map_err(|e| Error::Database(e.to_string()))
    }

    pub async fn get_user_by_wallet(&self, wallet: &str) -> Result<Option<ug_db::repositories::user_repo::UserRow>> {
        UserRepository::find_by_wallet(&self.db.pg, wallet)
            .await
            .map_err(|e| Error::Database(e.to_string()))
    }
}
