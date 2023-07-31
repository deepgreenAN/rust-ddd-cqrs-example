use domain::aggregates::{atm, bank_account};
use infrastructure::{
    atm_repository_impl::DbAtmRepository, bank_account_repository_impl::DbBankAccountRepository,
};
use serverside::api_handlers;
use serverside::command_handlers::{atm_command_handlers, bank_account_command_handlers};
use serverside::event_handlers::bank_account_event_handlers;
use serverside::query_handlers::QueryHandler;

use config::CONFIG;
use event_bus::event_bus_from_subscribes;
use migration::{Migrator, MigratorTrait};

use axum::{routing::post, Router};
use lru::LruCache;
use sea_orm::Database;
use sea_orm::JsonValue;
use std::sync::Arc;
use std::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // トレーシング
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL envvar is not set.");
    let db_connection = Database::connect(db_url).await?;
    // マイグレーション
    Migrator::up(&db_connection, None).await?;

    // リポジトリ
    let bank_account_repo = DbBankAccountRepository::new(db_connection.clone());
    let atm_repo = DbAtmRepository::new(db_connection.clone());

    // コマンドハンドラ
    let bank_account_command_handler = bank_account_command_handlers::BankAccountCommandHandler {
        deposit_money_handler: Box::new(
            bank_account_command_handlers::DepositMoneyCommandHandler::new(
                bank_account_repo.clone(),
                db_connection.clone(),
            ),
        ),
        open_account_handler: Box::new(
            bank_account_command_handlers::OpenAccountCommandHandler::new(
                bank_account_repo.clone(),
                db_connection.clone(),
            ),
        ),
        withdraw_money_handler: Box::new(
            bank_account_command_handlers::WithdrawMoneyCommandHandler::new(
                bank_account_repo.clone(),
                db_connection.clone(),
            ),
        ),
        write_check_handler: Box::new(
            bank_account_command_handlers::WriteCheckCommandHandler::new(
                bank_account_repo.clone(),
                db_connection.clone(),
            ),
        ),
        event_bus: bank_account_event_handlers::BankAccountEventBus {
            account_open_event_bus: event_bus_from_subscribes![
                bank_account_event_handlers::SendOpenAccountMailHandler::new()
            ],
            customer_deposited_money_bus: event_bus_from_subscribes![
                bank_account_event_handlers::AtmDepositHandler::new(
                    atm_repo.clone(),
                    db_connection.clone()
                )
            ],
            customer_withdrew_cash_bus: event_bus_from_subscribes![
                bank_account_event_handlers::AtmWithdrawHandler::new(
                    atm_repo.clone(),
                    db_connection.clone()
                )
            ],
            customer_wrote_check_bus: event_bus_from_subscribes![
                bank_account_event_handlers::ExternalWroteCheckHandler::new()
            ],
        },
        command_id_cache: Mutex::new(LruCache::new(10.try_into().unwrap())),
    };

    let atm_command_handler = atm_command_handlers::AtmCommandHandler {
        register_command_handler: Box::new(atm_command_handlers::RegisterAtmCommandHandler::new(
            atm_repo.clone(),
            db_connection.clone(),
        )),
        command_id_cache: Mutex::new(LruCache::new(10.try_into().unwrap())),
    };

    // クエリハンドラ
    let bank_account_query_handler = Arc::new(QueryHandler::<bank_account::orm::Model>::new(
        db_connection.clone(),
    ));

    let atm_query_handler = Arc::new(QueryHandler::<atm::orm::Model>::new(db_connection.clone()));

    let custom_query_handler = Arc::new(QueryHandler::<JsonValue>::new(db_connection.clone()));

    // axumのルーター
    let command_router: Router<()> = Router::new().nest(
        "/command",
        Router::new()
            .route(
                "/bank_account",
                post(
                    api_handlers::command_api_handler::<
                        bank_account_command_handlers::BankAccountCommandHandler,
                    >,
                ),
            )
            .with_state(Arc::new(bank_account_command_handler))
            .route(
                "/atm",
                post(api_handlers::command_api_handler::<atm_command_handlers::AtmCommandHandler>),
            )
            .with_state(Arc::new(atm_command_handler)),
    );

    let query_one_router: Router<()> = Router::new().nest(
        "/query_one",
        Router::new()
            .route(
                "/bank_account",
                post(api_handlers::query_one_api_handler::<bank_account::orm::Model>),
            )
            .with_state(Arc::clone(&bank_account_query_handler))
            .route(
                "/atm",
                post(api_handlers::query_one_api_handler::<atm::orm::Model>),
            )
            .with_state(Arc::clone(&atm_query_handler))
            .route(
                "/custom",
                post(api_handlers::query_one_api_handler::<JsonValue>),
            )
            .with_state(Arc::clone(&custom_query_handler)),
    );

    let query_all_router: Router<()> = Router::new().nest(
        "/query_all",
        Router::new()
            .route(
                "/bank_account",
                post(api_handlers::query_all_api_handler::<bank_account::orm::Model>),
            )
            .with_state(Arc::clone(&bank_account_query_handler))
            .route(
                "/atm",
                post(api_handlers::query_all_api_handler::<atm::orm::Model>),
            )
            .with_state(Arc::clone(&atm_query_handler))
            .route(
                "/custom",
                post(api_handlers::query_all_api_handler::<JsonValue>),
            )
            .with_state(Arc::clone(&custom_query_handler)),
    );

    let cors_layer = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let app_router: Router<()> = Router::new()
        .merge(command_router)
        .merge(query_one_router)
        .merge(query_all_router)
        .layer(cors_layer);

    println!("server started: http://{}", CONFIG.TEST_API_ADDR);

    axum::Server::bind(&CONFIG.TEST_API_ADDR.parse()?)
        .serve(app_router.into_make_service())
        .await?;

    Ok(())
}
