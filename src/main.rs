
use std::{env, fs::File};

use anyhow::Result;

use chrono::DateTime;
use log::{info, error};
use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();
    dotenvy::dotenv()?;
    let url = dotenvy::var("MEMPOOL_URL").expect("MEMPOOL_URL must be set");
    // Find blocks for start and end of month
    let start = 863565;
    let end = 868326;
    info!("querying for {} blocks", end - start + 1);
    let body = reqwest::get(format!(
        "http://{}/api/v1/blocks-bulk/{}/{}",
        url, start, end
    ))
    .await?
    .text()
    .await?;
    let bulk_disabled = body.contains("config.MEMPOOL.MAX_BLOCKS_BULK_QUERY");
    if bulk_disabled {
        error!("Bulk blocks query is disabled, use different mempool instance");
        return Ok(());
    }

    let jd = &mut serde_json::Deserializer::from_str(&body);

    let result: Result<Vec<Block>, _> = serde_path_to_error::deserialize(jd);
    match result {
        Ok(block_data) => {
            let filename = format!("blocks_{}_{}.csv", start, end);
            info!("writing {} blocks of data to {}", block_data.len(), &filename);
            save_to_csv(&block_data, filename)?;
        }
        Err(err) => {
            let path = err.path().to_string();
            error!("Error at path: {}", path);
            error!("{}", err);
            error!("{}", body);
        }
    }
    info!("done");
    Ok(())
}

fn save_to_csv(blocks: &[Block], filename: String) -> Result<()> {
    let file = File::create(filename)?;
    let mut wtr = csv::Writer::from_writer(file);

    // Write headers
    wtr.write_record(&["height", "timestamp", "reward", "fees", "pool"])?;

    // Write records
    for block in blocks {
        let datetime = DateTime::from_timestamp(block.timestamp, 0)
            .unwrap()
            .to_string();
        wtr.write_record(&[
            block.height.to_string(),
            datetime,
            block.reward.to_string(),
            block.total_fee_amt.to_string(),
            block.pool_slug.clone(),
        ])?;
    }

    wtr.flush()?;
    Ok(())
}

#[derive(Debug, Deserialize)]
struct FeeAmtPercentiles {
    min: f64,
    perc_10: f64,
    perc_25: f64,
    perc_50: f64,
    perc_75: f64,
    perc_90: f64,
    max: f64,
}

#[derive(Debug, Deserialize)]
struct FeeRatePercentiles {
    min: f64,
    perc_10: f64,
    perc_25: f64,
    perc_50: f64,
    perc_75: f64,
    perc_90: f64,
    max: f64,
}

#[derive(Debug, Deserialize)]
struct Block {
    height: i64,
    hash: String,
    timestamp: i64,
    median_timestamp: i64,
    previous_block_hash: String,
    difficulty: f64,
    header: String,
    version: u64,
    bits: u64,
    nonce: u64,
    size: usize,
    weight: usize,
    tx_count: i64,
    merkle_root: String,
    reward: i64,
    total_fee_amt: i64,
    avg_fee_amt: i64,
    median_fee_amt: i64,
    fee_amt_percentiles: FeeAmtPercentiles,
    avg_fee_rate: f64,
    median_fee_rate: f64,
    fee_rate_percentiles: FeeRatePercentiles,
    total_inputs: i64,
    total_input_amt: Option<i64>,
    total_outputs: i64,
    total_output_amt: i64,
    segwit_total_txs: i64,
    segwit_total_size: usize,
    segwit_total_weight: usize,
    avg_tx_size: f64,
    utxoset_change: i64,
    utxoset_size: Option<usize>,
    coinbase_raw: String,
    coinbase_address: String,
    coinbase_addresses: Vec<String>,
    coinbase_signature: String,
    coinbase_signature_ascii: String,
    pool_slug: String,
    pool_id: i64,
    orphans: Vec<Orphan>,
}

#[derive(Deserialize, Debug)]
struct Orphan {
    height: i64,
    hash: String,
    status: String,
    prevhash: String,
}
