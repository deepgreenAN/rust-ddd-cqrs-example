use ddd_cqrs_core::{Aggregate, HandleCommand};

use common::ApplicationError;
use domain::aggregates::BankAccount;
use domain::commands::bank_account_commands::BankAccountCommand;
use domain::commands::bank_account_commands::{
    DepositMoneyCommand, OpenAccountCommand, WithdrawMoneyCommand, WriteCheckCommand,
};
use domain::repositories::{BankAccountRepository, Transaction};
use infrastructure::InfraError;

// -------------------------------------------------------------------------------------------------
// OpenAccountCommandHandler

#[derive(Clone)]
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
        &mut self,
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

#[derive(Clone)]
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
        &mut self,
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

#[derive(Clone)]
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
        &mut self,
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

        transaction.commit();

        Ok(events)
    }
}

// -------------------------------------------------------------------------------------------------
// WriteCheckCommandHandler

#[derive(Clone)]
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
        &mut self,
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
