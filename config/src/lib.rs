#[allow(non_snake_case)]
pub struct Config {
    pub BALANCE_UPPER_LIM: f64,
    pub TEST_API_ADDR: &'static str,
    pub TEST_API_URL: &'static str,
}

impl Config {
    const fn init() -> Self {
        Self {
            BALANCE_UPPER_LIM: 100_000_000.0,
            TEST_API_ADDR: "127.0.0.1:8000",
            TEST_API_URL: "http://127.0.0.1:8000",
        }
    }
}

pub const CONFIG: Config = Config::init();
