use super::BaseObject;
use crate::Store;
use crate::entity::prelude::AttachmentEntity;
use crate::entity::{AttachmentModel, ItemModel};
use crate::error::TodoError;
use sea_orm::{DatabaseConnection, EntityTrait};
use std::fmt;
use tokio::sync::OnceCell;

pub struct Attachment {
    pub model: AttachmentModel,
    base: BaseObject,
    db: DatabaseConnection,
    store: OnceCell<Store>, // 延迟初始化且线程安全
}
impl Attachment {
    pub fn new(db: DatabaseConnection, model: AttachmentModel) -> Attachment {
        let base = BaseObject::default();
        Self {
            model,
            base,
            db,
            store: OnceCell::new(),
        }
    }
    pub async fn store(&self) -> &Store {
        self.store
            .get_or_init(|| async { Store::new(self.db.clone()).await })
            .await
    }
    pub async fn from_db(db: DatabaseConnection, attachment_id: &str) -> Result<Self, TodoError> {
        let attachment = AttachmentEntity::find_by_id(attachment_id)
            .one(&db)
            .await?
            .ok_or_else(|| TodoError::NotFound(format!("Item {} not found", attachment_id)))?;

        Ok(Self::new(db, attachment))
    }

    pub async fn delete(&self) -> Result<u64, TodoError> {
        self.store()
            .await
            .delete_attachment(self.model.id.as_str())
            .await
    }
    pub async fn item(&self) -> Option<ItemModel> {
        self.store().await.get_item(&self.model.item_id).await
    }
    pub fn set_item(&mut self, new_item_id: &str) -> &mut Self {
        self.model.item_id = new_item_id.to_string();
        self
    }
    pub fn duplicate(&self) -> AttachmentModel {
        AttachmentModel {
            file_type: self.model.file_type.clone(),
            file_name: self.model.file_name.clone(),
            file_size: self.model.file_size.clone(),
            file_path: self.model.file_path.clone(),
            ..Default::default()
        }
    }
}
impl fmt::Display for Attachment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "_________________________________\nID: {}\nITEM ID: {}\nFILE TYPE: {}\nFILE NAME: {}\nFILE SIZE: {}\nFILE PATH: {}\n---------------------------------",
            self.model.id.clone(),
            self.model.item_id.clone(),
            self.model.file_type.as_ref().unwrap_or(&"".to_string()),
            self.model.file_name.clone(),
            self.model.file_size.clone(),
            self.model.file_path.clone(),
        )
    }
}
