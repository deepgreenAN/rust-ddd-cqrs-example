use ddd_cqrs_core::{Aggregate, HandleCommand};

use common::commands::atm_commands::AtmCommand;
use common::commands::atm_commands::RegisterAtmCommand;
use common::commands::CommandId;
use common::ApplicationError;
use domain::aggregates::Atm;
use domain::repositories::{AtmRepository, Transaction};
use infrastructure::InfraError;

use lru::LruCache;
use std::sync::Mutex;

// -------------------------------------------------------------------------------------------------
// RegisterAtmCommandHandler

pub struct RegisterAtmCommandHandler<R: AtmRepository<Error = InfraError>> {
    repo: R,
    pool: <R::Transaction as Transaction>::Pool,
}

#[async_trait::async_trait]
impl<R: AtmRepository<Error = InfraError>> HandleCommand for RegisterAtmCommandHandler<R> {
    type Aggregate = Atm;
    type Command = RegisterAtmCommand;
    type Error = ApplicationError;

    async fn handle_command(
        &self,
        command: Self::Command,
    ) -> Result<Vec<<Self::Aggregate as Aggregate>::Event>, Self::Error> {
        let transaction = <R::Transaction as Transaction>::begin(&self.pool).await?;

        let RegisterAtmCommand {
            location,
            total_cash,
        } = command;

        let mut atm = Atm::from_domains(location, total_cash);
        let events = atm.domain_events_mut().take();
        self.repo.save(atm, Some(&transaction)).await?;

        transaction.commit().await?;

        Ok(events)
    }
}

// -------------------------------------------------------------------------------------------------
// AtmCommandHandler

/// Atmの統合コマンドハンドラー
pub struct AtmCommandHandler {
    pub register_command_handler: Box<
        dyn HandleCommand<Command = RegisterAtmCommand, Aggregate = Atm, Error = ApplicationError>,
    >,
    pub command_id_cache: Mutex<LruCache<CommandId, ()>>,
}

impl AtmCommandHandler {
    fn check_command_duplicate(&self, id: CommandId) -> bool {
        let mut cache_lock = self.command_id_cache.lock().unwrap();
        if cache_lock.contains(&id) {
            true
        } else {
            cache_lock.put(id, ());
            false
        }
    }

    pub async fn handle_command(&self, command: AtmCommand) -> Result<(), ApplicationError> {
        let _ = match command {
            AtmCommand::RegisterAtmCommand(cmd, id) => {
                if !self.check_command_duplicate(id) {
                    self.register_command_handler.handle_command(cmd).await?
                } else {
                    if self.register_command_handler.allow_duplicate() {
                        self.register_command_handler.handle_command(cmd).await?
                    } else {
                        Vec::new()
                    }
                }
            }
        };

        Ok(())
    }
}
