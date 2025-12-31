use std::env;

#[derive(Debug, Clone)]
pub enum AppMode {
    Eth,
    Btc,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub mode: AppMode,

    pub clickhouse_url: String,
    pub clickhouse_user: String,
    pub clickhouse_pass: String,

    pub clickhouse_db_eth: String,
    pub clickhouse_db_btc: String,

    pub eth_rpc_url: Option<String>,
    pub btc_api_url: Option<String>,
    //pub btc_api_url: String,

    pub btc_start_block: u64,
    pub eth_start_block: u64,

    pub total_btc_txs: u64,
    pub total_eth_txs: u64,
}

// impl AppConfig {
//     pub fn from_env() -> Self {
//         let mode = match env::var("APP_MODE").as_deref() {
//             Ok("eth") => AppMode::Eth,
//             Ok("btc") => AppMode::Btc,
//             _ => panic!("APP_MODE must be eth or btc"),
//         };

//         Self {
//             mode,
//             clickhouse_url: env::var("CLICKHOUSE_URL").expect("Clickhouse URL Faild"),
//             clickhouse_user: env::var("CLICKHOUSE_USER").expect("Clickhouse Username Faild"),
//             clickhouse_pass: env::var("CLICKHOUSE_PASSWORD").expect("Clickhouse Password Faild"),

//             clickhouse_db_eth: env::var("CLICKHOUSE_DB_ETH").expect("Clickhouse DB ETH Faild").into(),
//             clickhouse_db_btc: env::var("CLICKHOUSE_DB_BTC").expect("Clickhouse DB BTC Faild").into(),

//             eth_rpc_url: env::var("ETH_RPC_HTTP").ok(),
//             btc_api_url: env::var("BTC_API_URL").ok(),

//             btc_start_block: env::var("BTC_START_BLOCK").expect("Clickhouse BTC start block Faild").parse().expect("Cannot Parse String to int"),
//             eth_start_block: env::var("ETH_START_BLOCK").expect("Clickhouse ETH Start block Faild").parse().expect("Cannot Parse U64"),
//             total_btc_txs: env::var("TOTAL_BTC_TXS").expect("Clickhouse totla btc Faild").parse().expect("cannot pase int"),
//             total_eth_txs: env::var("TOTAL_ETH_TXS").expect("Clickhouse total eth Faild").parse().expect("Cannot parse intss"),
//         }
//     }
// }

// Test
impl AppConfig {
    pub fn from_env() -> Self {
        let mode = AppMode::Eth;

        Self {
            mode,
            clickhouse_url: "http://localhost:8123".into(),
            clickhouse_user: "default".into(),
            clickhouse_pass: "".into(),

            clickhouse_db_eth:"eth_db".into(),
            clickhouse_db_btc:"btc_db".into(),
            
            //eth_rpc_url: Some("http://localhost:8545".into()),
            eth_rpc_url: Some("https://rpc.ankr.com/eth/a4ce905377a7aa94ded62bf6efb50b20acde76159d163f8de77a16ec6237137b".into()),
            btc_api_url: Some("https://blockstream.info/api".into()),

            btc_start_block: 831000,
            eth_start_block: 19000000,
            total_btc_txs: 500,
            total_eth_txs: 500,
        }
    }
}