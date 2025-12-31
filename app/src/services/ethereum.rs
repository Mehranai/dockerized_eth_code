use crate::services::loader::LoaderEth;
use crate::models::transaction::Sensivity;
use crate::services::progress::{save_tx, save_wallet};

use ethers::prelude::*;
use clickhouse::Client;
use std::sync::Arc;
use anyhow::Result;

use futures::stream::{FuturesUnordered, StreamExt};


// Main Function

pub async fn fetch_eth(
    loader: Arc<LoaderEth>,
    start_block: u64,
    total_txs: u64,
) -> Result<()> {

    let provider = loader.eth_provider.clone();
    let clickhouse = loader.clickhouse.clone();
    let mut tx_count: u64 = 0;

    for block_number in start_block..start_block + 1000 {
        if tx_count >= total_txs {
            break;
        }

        let Some(block) = provider.get_block_with_txs(block_number).await? else {
            continue;
        };

        let mut tasks = FuturesUnordered::new();

        for tx in block.transactions {
            if tx_count >= total_txs {
                break;
            }

            let provider = provider.clone();
            let clickhouse = clickhouse.clone();

            tasks.push(tokio::spawn(async move {
                process_tx(provider, clickhouse, tx, block_number).await
            }));

            tx_count += 1;
            println!("Added Tx #{:?}", tx_count)
        }

        while let Some(res) = tasks.next().await {
            res??;
        }
    }

    Ok(())
}


// Process Single Tx

async fn process_tx(
    provider: Arc<Provider<Http>>,
    clickhouse: Arc<Client>,
    tx: Transaction,
    block_number: u64,
) -> Result<()> {

    let hash = format!("{:#x}", tx.hash);
    let from = tx.from;
    let to = tx.to;
    let value = tx.value.to_string();
    let sensivity = Sensivity::Green as u8;

    save_tx(
        clickhouse.clone(),
        hash,
        block_number,
        from.to_string(),
        to.unwrap_or_default().to_string(),
        value,
        sensivity,
    )
    .await?;

    save_wallet_eth(provider.clone(), clickhouse.clone(), from).await?;

    if let Some(to_addr) = to {
        save_wallet_eth(provider, clickhouse, to_addr).await?;
    }

    Ok(())
}


// Save Wallet / Contract

async fn save_wallet_eth(
    provider: Arc<Provider<Http>>,
    clickhouse: Arc<Client>,
    addr: Address,
) -> Result<()> {

    if addr == Address::zero() {
        return Ok(());
    }

    let balance = provider.get_balance(addr, None).await?;
    let nonce = provider.get_transaction_count(addr, None).await?;
    let wallet_type = detect_wallet_type(provider, addr).await?;

    save_wallet(
        clickhouse,
        &addr.to_string(),
        balance.to_string(),
        nonce.as_u64(),
        wallet_type,
    )
    .await?;

    Ok(())
}


// Updated
// Detect Wallet Type

/// wallet          -> EOA (no bytecode)
/// smart_contract  -> has bytecode

async fn detect_wallet_type(
    provider: Arc<Provider<Http>>,
    address: Address,
) -> anyhow::Result<String> {

    let code = provider.get_code(address, None).await?;

    if code.0.is_empty() {
        Ok("wallet".to_string())
    } else {
        Ok("smart_contract".to_string())
    }
}