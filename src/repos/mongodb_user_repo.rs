use async_trait::async_trait;
use mongodb::bson::{doc, to_document};
use mongodb::{Collection, Database};

use crate::models::user::User;

use super::traits::UserRepo;

#[derive(Clone)]
pub struct MongoDBUserRepo {
    col: Collection<User>,
}

impl MongoDBUserRepo {
    pub async fn new(db: &Database) -> Self {
        let collection = db.collection::<User>("users");
        Self { col: collection }
    }
}

#[async_trait]
impl UserRepo for MongoDBUserRepo {
    async fn get_user_by_uuid(&self, uuid: &uuid::Uuid) -> Result<Option<User>, ()> {
        let filter = doc! {"uuid": uuid };
        let user = self.col.find_one(Some(filter), None).await.unwrap();
        Ok(user)
    }

    async fn create_user(&self, user: &mut User) -> Result<(), ()> {
        let id = self.col.insert_one(user.clone(), None).await.unwrap();
        user.id = Some(id.inserted_id.as_object_id().unwrap().clone());
        Ok(())
    }

    async fn update_user(&self, user: &User) -> Result<(), ()> {
        let filter = doc! {"uuid": user.uuid };
        self.col
            .update_one(filter, to_document(user).unwrap(), None)
            .await
            .unwrap();
        Ok(())
    }

    async fn get_user_by_login(&self, login: &str) -> Result<Option<User>, ()> {
        let filter = doc! {"$or": [{"username": login}, {"email": login}]};
        let user = self.col.find_one(Some(filter), None).await.unwrap();
        Ok(user)
    }
}
