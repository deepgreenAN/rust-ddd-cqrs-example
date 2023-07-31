mod inner {
    use common::{
        query_statement::{QueryStatement, DEFAULT_DB_BACKEND},
        ApplicationError,
    };
    use domain::aggregates::atm::{self, Atm};
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    pub async fn atm_all(base_url: &str) -> Result<Vec<Atm>, ApplicationError> {
        let query = QueryStatement::from_select(DEFAULT_DB_BACKEND, atm::orm::Entity::find());

        crate::api_handler::inner::query_all_atm(base_url, query).await
    }

    pub async fn atm_from_location(
        base_url: &str,
        location: &atm::AtmLocation,
    ) -> Result<Option<Atm>, ApplicationError> {
        let query = QueryStatement::from_select(
            DEFAULT_DB_BACKEND,
            atm::orm::Entity::find().filter(atm::orm::Column::Location.eq(location)),
        );

        crate::api_handler::inner::query_one_atm(base_url, query).await
    }
}

use crate::API_BASE_URL;
use common::ApplicationError;
use domain::aggregates::{atm, Atm};

/// 全てのAtmを取得
pub async fn atm_all() -> Result<Vec<Atm>, ApplicationError> {
    inner::atm_all(API_BASE_URL).await
}

/// `location`を持つAtmを取得
pub async fn atm_from_location(
    location: &atm::AtmLocation,
) -> Result<Option<Atm>, ApplicationError> {
    inner::atm_from_location(API_BASE_URL, location).await
}
