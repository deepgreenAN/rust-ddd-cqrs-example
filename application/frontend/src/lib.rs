mod api_handler;
pub(crate) mod utils;

use config::CONFIG;

// とりあえずテスト用URLを利用
const API_BASE_URL: &'static str = CONFIG.TEST_API_URL;
