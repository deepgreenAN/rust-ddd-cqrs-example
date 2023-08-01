use common::ApplicationError;
use domain::events::bank_account_events::{
    AccountOpenedEvent, BankAccountEvent, CustomerDepositedMoneyEvent, CustomerWithdrewCashEvent,
    CustomerWroteCheckEvent,
};
use domain::repositories::{AtmRepository, Transaction};
use infrastructure::InfraError;

use event_bus::{EventBus, Subscribe, Task};

use derive_new::new;
use tracing::info;

// -------------------------------------------------------------------------------------------------
// SendOpenAccountMailHandler

/// アカウントの開設をメールで送信するイベントハンドラ
#[derive(new)]
pub struct SendOpenAccountMailHandler;

#[async_trait::async_trait]
impl Subscribe for SendOpenAccountMailHandler {
    type InputEvent = AccountOpenedEvent;
    type Output = Result<(), ApplicationError>;
    async fn handle_event<'event>(
        &self,
        event: &'event Self::InputEvent,
    ) -> Result<(), ApplicationError> {
        info!("SendOpenAccountMainHandler dispatched.");

        info!("Send email to {:?}", event.email_address);
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------
// AtmDepositHandler

/// Atmに現金を入れるイベントハンドラ
#[derive(new)]
pub struct AtmDepositHandler<AR: AtmRepository<Error = InfraError>> {
    repo: AR,
    pool: <AR::Transaction as Transaction>::Pool,
}

#[async_trait::async_trait]
impl<AR: AtmRepository<Error = InfraError>> Subscribe for AtmDepositHandler<AR> {
    type InputEvent = CustomerDepositedMoneyEvent;
    type Output = Result<(), ApplicationError>;
    async fn handle_event<'event>(
        &self,
        event: &'event Self::InputEvent,
    ) -> Result<(), ApplicationError> {
        info!("AtmDepositHandler dispatched.");

        let transaction = <AR::Transaction as Transaction>::begin(&self.pool).await?;

        let CustomerDepositedMoneyEvent {
            account_id: _,
            amount,
            balance: _,
            atm_id,
        } = event;

        let mut atm = self.repo.find_by_id(*atm_id, Some(&transaction)).await?;

        // 実際はもうアカウントのトランザクションを終了しているため，他の方法でリカバリーする
        atm.charge_cash(*amount)?;

        self.repo.edit(atm, Some(&transaction)).await?;

        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------
// AtmWithdrawHandler

/// Atmからお金を引き出すイベントハンドラ
#[derive(new)]
pub struct AtmWithdrawHandler<AR: AtmRepository<Error = InfraError>> {
    repo: AR,
    pool: <AR::Transaction as Transaction>::Pool,
}

#[async_trait::async_trait]
impl<AR: AtmRepository<Error = InfraError>> Subscribe for AtmWithdrawHandler<AR> {
    type InputEvent = CustomerWithdrewCashEvent;
    type Output = Result<(), ApplicationError>;
    async fn handle_event<'event>(
        &self,
        event: &'event Self::InputEvent,
    ) -> Result<(), ApplicationError> {
        info!("AtmWithdrawHandler dispatched.");

        let transaction = <AR::Transaction as Transaction>::begin(&self.pool).await?;

        let CustomerWithdrewCashEvent {
            account_id: _,
            amount,
            balance: _,
            atm_id,
        } = event;

        let mut atm = self.repo.find_by_id(*atm_id, Some(&transaction)).await?;

        // 実際はもうアカウントのトランザクションを終了しているため，他の方法でリカバリーする
        atm.withdraw(*amount)?;

        self.repo.edit(atm, Some(&transaction)).await?;

        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------
// ExternalWroteCheckHandler

/// 小切手利用の際に外部サービスを利用するイベントハンドラ
#[derive(new)]
pub struct ExternalWroteCheckHandler;

#[async_trait::async_trait]
impl Subscribe for ExternalWroteCheckHandler {
    type InputEvent = CustomerWroteCheckEvent;
    type Output = Result<(), ApplicationError>;
    async fn handle_event<'event>(
        &self,
        event: &'event Self::InputEvent,
    ) -> Result<(), ApplicationError> {
        info!("ExternalWroteCheckHandler dispatched.");

        let CustomerWroteCheckEvent {
            account_id: _,
            check_number,
            amount,
            balance: _,
        } = event;

        info!("Use external api:  check_number: {check_number}, amount: {amount}");
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------
// BankAccountEventBus

/// BankAccountに対するイベントバス
#[derive(derive_new::new)]
pub struct BankAccountEventBus {
    event_bus: EventBus<Result<(), ApplicationError>>,
}

impl BankAccountEventBus {
    pub fn dispatch_event(
        &self,
        event: BankAccountEvent,
    ) -> Vec<Task<Result<(), ApplicationError>>> {
        use BankAccountEvent::*;

        match event {
            AccountOpenedEvent(e) => self.event_bus.dispatch_event(e),
            CustomerDepositedMoneyEvent(e) => self.event_bus.dispatch_event(e),
            CustomerWithdrewCashEvent(e) => self.event_bus.dispatch_event(e),
            CustomerWroteCheckEvent(e) => self.event_bus.dispatch_event(e),
        }
    }
}
