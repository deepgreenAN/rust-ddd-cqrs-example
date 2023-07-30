use crate::event_handlers::bank_account_event_handlers::BankAccountEventBus;

use ddd_cqrs_core::{Aggregate, HandleCommand};

use common::commands::bank_account_commands::BankAccountCommand;
use common::commands::bank_account_commands::{
    DepositMoneyCommand, OpenAccountCommand, WithdrawMoneyCommand, WriteCheckCommand,
};
use common::commands::CommandId;
use common::ApplicationError;
use domain::aggregates::BankAccount;
use domain::repositories::{BankAccountRepository, Transaction};
use infrastructure::InfraError;

use lru::LruCache;
use std::sync::Mutex;

// -------------------------------------------------------------------------------------------------
// OpenAccountCommandHandler

pub struct OpenAccountCommandHandler<R: BankAccountRepository<Error = InfraError>> {
    repo: R,
    pool: <R::Transaction as Transaction>::Pool,
}

#[async_trait::async_trait]
impl<R: BankAccountRepository<Error = InfraError>> HandleCommand for OpenAccountCommandHandler<R> {
    type Command = OpenAccountCommand;
    type Aggregate = BankAccount;
    type Error = ApplicationError;

    async fn handle_command(
        &self,
        command: Self::Command,
    ) -> Result<Vec<<BankAccount as Aggregate>::Event>, Self::Error> {
        let transaction = R::Transaction::begin(&self.pool).await?;

        let OpenAccountCommand {
            account_name,
            email_address,
        } = command;

        let mut bank_account = BankAccount::from_domains(email_address, account_name);
        bank_account.open_account();

        let events = bank_account.domain_events_mut().take();

        self.repo.save(bank_account, Some(&transaction)).await?;

        transaction.commit().await?;

        Ok(events)
    }
}

// -------------------------------------------------------------------------------------------------
// DepositMoneyCommandHandler

pub struct DepositMoneyCommandHandler<R: BankAccountRepository<Error = InfraError>> {
    repo: R,
    pool: <R::Transaction as Transaction>::Pool,
}

#[async_trait::async_trait]
impl<R: BankAccountRepository<Error = InfraError>> HandleCommand for DepositMoneyCommandHandler<R> {
    type Command = DepositMoneyCommand;
    type Aggregate = BankAccount;
    type Error = ApplicationError;

    async fn handle_command(
        &self,
        command: Self::Command,
    ) -> Result<Vec<<BankAccount as Aggregate>::Event>, Self::Error> {
        use domain::events::bank_account_events::CustomerDepositedMoneyEvent;

        let transaction = R::Transaction::begin(&self.pool).await?;

        let DepositMoneyCommand {
            account_id,
            amount,
            atm_id,
        } = command;

        let mut bank_account = self.repo.find_by_id(account_id, Some(&transaction)).await?;
        if let Ok(_) = bank_account.deposit_money(amount) {
            let balance = bank_account.balance();

            bank_account.domain_events_mut().push(
                CustomerDepositedMoneyEvent {
                    account_id,
                    amount,
                    balance,
                    atm_id,
                }
                .into(),
            );
        }

        let events = bank_account.domain_events_mut().take();
        self.repo.edit(bank_account, Some(&transaction)).await?;

        transaction.commit().await?;

        Ok(events)
    }
}

// -------------------------------------------------------------------------------------------------
// WithdrawMoneyCommand

pub struct WithdrawMoneyCommandHandler<R: BankAccountRepository<Error = InfraError>> {
    repo: R,
    pool: <R::Transaction as Transaction>::Pool,
}

#[async_trait::async_trait]
impl<R: BankAccountRepository<Error = InfraError>> HandleCommand
    for WithdrawMoneyCommandHandler<R>
{
    type Command = WithdrawMoneyCommand;
    type Aggregate = BankAccount;
    type Error = ApplicationError;

    async fn handle_command(
        &self,
        command: Self::Command,
    ) -> Result<Vec<<BankAccount as Aggregate>::Event>, Self::Error> {
        use domain::events::bank_account_events::CustomerWithdrewCashEvent;

        let transaction = R::Transaction::begin(&self.pool).await?;

        let WithdrawMoneyCommand {
            account_id,
            amount,
            atm_id,
        } = command;

        let mut bank_account = self.repo.find_by_id(account_id, Some(&transaction)).await?;
        if let Ok(_) = bank_account.withdraw_money(amount) {
            let balance = bank_account.balance();

            bank_account.domain_events_mut().push(
                CustomerWithdrewCashEvent {
                    account_id,
                    amount,
                    balance,
                    atm_id,
                }
                .into(),
            );
        }

        let events = bank_account.domain_events_mut().take();
        self.repo.edit(bank_account, Some(&transaction)).await?;

        transaction.commit().await?;

        Ok(events)
    }
}

// -------------------------------------------------------------------------------------------------
// WriteCheckCommandHandler

pub struct WriteCheckCommandHandler<R: BankAccountRepository<Error = InfraError>> {
    repo: R,
    pool: <R::Transaction as Transaction>::Pool,
}

#[async_trait::async_trait]
impl<R: BankAccountRepository<Error = InfraError>> HandleCommand for WriteCheckCommandHandler<R> {
    type Command = WriteCheckCommand;
    type Aggregate = BankAccount;
    type Error = ApplicationError;

    async fn handle_command(
        &self,
        command: Self::Command,
    ) -> Result<Vec<<BankAccount as Aggregate>::Event>, Self::Error> {
        let transaction = R::Transaction::begin(&self.pool).await?;

        let WriteCheckCommand {
            account_id,
            amount,
            check_number,
        } = command;

        let mut bank_account = self.repo.find_by_id(account_id, Some(&transaction)).await?;
        bank_account.write_check(amount, check_number)?;

        let events = bank_account.domain_events_mut().take();
        self.repo.edit(bank_account, Some(&transaction)).await?;

        transaction.commit().await?;

        Ok(events)
    }
}

// -------------------------------------------------------------------------------------------------
// BankAccountに関する統合コマンド

/// BankAccountに関する統合コマンド．コマンドを増やした場合のみ変更されるため，具象型とする
pub struct BankAccountCommandHandler {
    pub deposit_money_handler: Box<
        dyn HandleCommand<
            Command = DepositMoneyCommand,
            Aggregate = BankAccount,
            Error = ApplicationError,
        >,
    >,
    pub open_account_handler: Box<
        dyn HandleCommand<
            Command = OpenAccountCommand,
            Aggregate = BankAccount,
            Error = ApplicationError,
        >,
    >,
    pub withdraw_money_handler: Box<
        dyn HandleCommand<
            Command = WithdrawMoneyCommand,
            Aggregate = BankAccount,
            Error = ApplicationError,
        >,
    >,
    pub write_check_handler: Box<
        dyn HandleCommand<
            Command = WriteCheckCommand,
            Aggregate = BankAccount,
            Error = ApplicationError,
        >,
    >,
    /// BankAccountEventに関するイベントバス
    pub event_bus: BankAccountEventBus,
    /// リトライなどにより重複したコマンドじゃないかどうかを判定するキャッシュ．場合によってはリポジトリとする．
    pub command_id_cache: Mutex<LruCache<CommandId, ()>>,
}

impl BankAccountCommandHandler {
    fn check_command_duplicate(&self, id: CommandId) -> bool {
        let mut cache_lock = self.command_id_cache.lock().unwrap();
        if cache_lock.contains(&id) {
            true
        } else {
            cache_lock.put(id, ());
            false
        }
    }

    pub async fn handle_command(
        &self,
        command: BankAccountCommand,
    ) -> Result<(), ApplicationError> {
        let events = match command {
            BankAccountCommand::OpenAccountCommand(cmd, id) => {
                if !self.check_command_duplicate(id) {
                    self.open_account_handler.handle_command(cmd).await?
                } else {
                    if self.open_account_handler.allow_duplicate() {
                        self.open_account_handler.handle_command(cmd).await?
                    } else {
                        Vec::new()
                    }
                }
            }
            BankAccountCommand::DepositMoneyCommand(cmd, id) => {
                if !self.check_command_duplicate(id) {
                    self.deposit_money_handler.handle_command(cmd).await?
                } else {
                    if self.deposit_money_handler.allow_duplicate() {
                        self.deposit_money_handler.handle_command(cmd).await?
                    } else {
                        Vec::new()
                    }
                }
            }
            BankAccountCommand::WithdrawMoneyCommand(cmd, id) => {
                if !self.check_command_duplicate(id) {
                    self.withdraw_money_handler.handle_command(cmd).await?
                } else {
                    if self.withdraw_money_handler.allow_duplicate() {
                        self.withdraw_money_handler.handle_command(cmd).await?
                    } else {
                        Vec::new()
                    }
                }
            }
            BankAccountCommand::WriteCheckCommand(cmd, id) => {
                if !self.check_command_duplicate(id) {
                    self.write_check_handler.handle_command(cmd).await?
                } else {
                    if self.write_check_handler.allow_duplicate() {
                        self.write_check_handler.handle_command(cmd).await?
                    } else {
                        Vec::new()
                    }
                }
            }
        };

        // イベントのディスパッチ
        for event in events.into_iter() {
            self.event_bus.dispatch_event(event);
        }

        Ok(())
    }
}
