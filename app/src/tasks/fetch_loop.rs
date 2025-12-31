use std::sync::Arc;
use anyhow::Result;
use clickhouse::Client;

use crate::services::{
    loader::{LoaderEth, LoaderBtc},
    bitcoin,
    ethereum,
};

use crate::db::init_eth::init_eth_db;
use crate::db::init_btc::init_btc_db;
use crate::config::AppConfig;

pub async fn run_btc_loop(config: AppConfig) -> Result<()> {
    //  Loader 
    let loader = Arc::new(LoaderBtc::new(&config).await?);

    // client موقت بدون database برای init
    let admin_client = Client::default()
        .with_url(&config.clickhouse_url)
        .with_user(&config.clickhouse_user)
        .with_password(&config.clickhouse_pass);

    // init دیتابیس و جدول‌ها
    init_btc_db(&admin_client).await?;

    // حالا fetch امن
    bitcoin::fetch_btc(
        loader,
        config.btc_start_block,
        config.total_btc_txs,
        config.btc_api_url
        .as_ref()
        .expect("Canngot get api url!"),
    ).await?;

    Ok(())
}

pub async fn run_eth_loop(config: AppConfig) -> Result<()> {
    let loader = Arc::new(LoaderEth::new(&config).await?);

    let admin_client = Client::default()
        .with_url(&config.clickhouse_url)
        .with_user(&config.clickhouse_user)
        .with_password(&config.clickhouse_pass);

    init_eth_db(&admin_client).await?;

    ethereum::fetch_eth(
        loader,
        config.eth_start_block,
        config.total_eth_txs,
    ).await?;

    Ok(())
}
