mod inner {
    use common::{
        query_statement::{QueryStatement, DEFAULT_DB_BACKEND},
        ApplicationError,
    };
    use domain::aggregates::bank_account::{self, BankAccount, EmailAddress};
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    pub async fn bank_account_all(base_url: &str) -> Result<Vec<BankAccount>, ApplicationError> {
        let query =
            QueryStatement::from_select(DEFAULT_DB_BACKEND, bank_account::orm::Entity::find());

        crate::api_handler::inner::query_all_bank_account(base_url, query).await
    }

    pub async fn bank_account_from_email(
        base_url: &str,
        email_address: &EmailAddress,
    ) -> Result<Option<BankAccount>, ApplicationError> {
        let query = QueryStatement::from_select(
            DEFAULT_DB_BACKEND,
            bank_account::orm::Entity::find()
                .filter(bank_account::orm::Column::EmailAddress.eq(email_address)),
        );
        crate::api_handler::inner::query_one_bank_account(base_url, query).await
    }
}

use crate::API_BASE_URL;
use common::ApplicationError;
use domain::aggregates::{bank_account::EmailAddress, BankAccount};

pub async fn bank_account_all() -> Result<Vec<BankAccount>, ApplicationError> {
    inner::bank_account_all(API_BASE_URL).await
}

pub async fn bank_account_from_email(
    email_address: &EmailAddress,
) -> Result<Option<BankAccount>, ApplicationError> {
    inner::bank_account_from_email(API_BASE_URL, email_address).await
}
