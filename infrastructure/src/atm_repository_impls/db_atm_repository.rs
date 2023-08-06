use crate::{transactions::DbTransaction, InfraError};
use domain::aggregates::atm::{self, Atm, AtmId};
use domain::repositories::{AtmRepository, Repository};

use derive_new::new;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel};

/// データベースを用いたAtmRepository
#[derive(Clone, Debug, new)]
pub struct DbAtmRepository {
    conn: DatabaseConnection,
}

#[async_trait::async_trait]
impl Repository for DbAtmRepository {
    type Error = InfraError;
    type Aggregate = Atm;
    type Transaction = DbTransaction;

    async fn save<'t>(
        &self,
        atm: Atm,
        transaction: Option<&'t Self::Transaction>,
    ) -> Result<(), Self::Error> {
        let active_model = Into::<atm::orm::Model>::into(atm).into_active_model();

        match transaction {
            Some(transaction) => {
                active_model.insert(transaction.inner()).await?;
            }
            None => {
                active_model.insert(&self.conn).await?;
            }
        }
        Ok(())
    }
    async fn edit<'t>(
        &self,
        atm: Atm,
        transaction: Option<&'t Self::Transaction>,
    ) -> Result<(), Self::Error> {
        let active_model = Into::<atm::orm::Model>::into(atm)
            .into_active_model()
            .reset_all();

        match transaction {
            Some(transaction) => {
                active_model.update(transaction.inner()).await?;
            }
            None => {
                active_model.update(&self.conn).await?;
            }
        }

        Ok(())
    }
    async fn find_by_id<'t>(
        &self,
        id: AtmId,
        transaction: Option<&'t Self::Transaction>,
    ) -> Result<Self::Aggregate, Self::Error> {
        let found_atm = {
            let select = atm::orm::Entity::find_by_id(id);

            match transaction {
                Some(transaction) => select.one(transaction.inner()).await?,
                None => select.one(&self.conn).await?,
            }
        };

        match found_atm {
            Some(res) => Ok(res.into()),
            None => Err(InfraError::RecordNotFoundError(format!(
                "Not found id: {}",
                Into::<String>::into(id)
            ))),
        }
    }
    async fn remove<'t>(
        &self,
        id: AtmId,
        transaction: Option<&'t Self::Transaction>,
    ) -> Result<(), Self::Error> {
        let delete = atm::orm::Entity::delete_by_id(id);

        match transaction {
            Some(transaction) => {
                delete.exec(transaction.inner()).await?;
            }
            None => {
                delete.exec(&self.conn).await?;
            }
        }

        Ok(())
    }
}

impl AtmRepository for DbAtmRepository {}
