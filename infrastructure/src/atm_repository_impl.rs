use crate::InfraError;
use domain::aggregates::atm::orm as atm_mod;
use domain::aggregates::atm::{Atm, AtmId};
use domain::repositories::AtmRepository;

use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, IntoActiveModel};

/// データベースを用いたAtmRepository
pub struct DbAtmRepository<T: ConnectionTrait + Send> {
    conn: T,
}

impl<T: ConnectionTrait + Send> DbAtmRepository<T> {
    pub fn new(conn: T) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl<T: ConnectionTrait + Send> AtmRepository for DbAtmRepository<T> {
    type Error = InfraError;
    async fn save(&self, atm: Atm) -> Result<(), Self::Error> {
        let active_model = Into::<atm_mod::Model>::into(atm).into_active_model();
        active_model.insert(&self.conn).await?;

        Ok(())
    }
    async fn edit(&self, atm: Atm) -> Result<(), Self::Error> {
        let active_model = Into::<atm_mod::Model>::into(atm)
            .into_active_model()
            .reset_all();
        active_model.update(&self.conn).await?;

        Ok(())
    }
    async fn remove(&self, id: AtmId) -> Result<(), Self::Error> {
        atm_mod::Entity::delete_by_id(id).exec(&self.conn).await?;

        Ok(())
    }
}
