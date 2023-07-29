use common::ApplicationError;
use domain::events::bank_account_events::{
    AccountOpenedEvent, CustomerDepositedMoneyEvent, CustomerWithdrewCashEvent,
    CustomerWroteCheckEvent,
};
use domain::repositories::AtmRepository;

use event_bus::Subscribe;

use tracing::info;

#[derive(Clone)]
pub struct SendOpenAccountMailHandler;

#[async_trait::async_trait]
impl Subscribe for SendOpenAccountMailHandler {
    type InputEvent = AccountOpenedEvent;
    type Error = ApplicationError;
    async fn handle_event<'event>(
        &self,
        event: &'event Self::InputEvent,
    ) -> Result<(), ApplicationError> {
        info!("SendOpenAccountMainHandler dispatched.");

        info!("Send email to {:?}", event.email_address);
        Ok(())
    }
}

#[derive(Clone)]
pub struct AtmDepositHandler<AR: AtmRepository> {
    repo: AR,
}

#[async_trait::async_trait]
impl<AR: AtmRepository> Subscribe for AtmDepositHandler<AR> {
    type InputEvent = CustomerDepositedMoneyEvent;
    type Error = ApplicationError;
    async fn handle_event<'event>(
        &self,
        event: &'event Self::InputEvent,
    ) -> Result<(), ApplicationError> {
        info!("AtmDepositHandler dispatched.");

        Ok(())
    }
}
