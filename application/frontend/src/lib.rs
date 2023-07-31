mod api_handler;
pub mod queries;
pub(crate) mod utils;

pub use api_handler::*;

// commonからの再エクスポート
pub use common::commands;
pub use common::commands::CommandId;
pub use common::query_statement;
pub use common::ApplicationError;

// domainからの再エクスポート
pub use domain::aggregates;

use config::CONFIG;

// とりあえずテスト用URLを利用
const API_BASE_URL: &'static str = CONFIG.TEST_API_URL;
