mod dates;

use std::{env, fs::File, io};

use anyhow::Result;

use chrono::DateTime;
use dates::get_last_month_time_box_utc;
use log::{debug, error, info};
use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();
    dotenvy::dotenv()?;
    let url = match dotenvy::var("MEMPOOL_URL") {
        Ok(url) => url,
        Err(_) => {
            info!("MEMPOOL_URL is not set. Please enter the MEMPOOL URL:");
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        }
    };
    let (start_time, end_time) = get_last_month_time_box_utc();

    let start_timestamp = start_time.timestamp();
    debug!("start_timestamp: {}", start_timestamp);
    let start_url = format!("http://{}/api/v1/mining/blocks/timestamp/{}", url, start_timestamp);
    debug!("Start URL: {}", start_url);
    // Query the API endpoint to find the approximate first block of the current month
    let start_response = reqwest::get(&start_url).await?.text().await?;
    debug!("Response: {}", start_response);
    let start: TimeStampResponse = serde_json::from_str(&start_response)?;
    debug!("Starting with block: {:?}", start);

    let end_timestamp = end_time.timestamp();
    debug!("end_timestamp: {}", end_timestamp);
    let end_url = format!("http://{}/api/v1/mining/blocks/timestamp/{}", url, end_timestamp);
    debug!("End URL: {}", end_url);
    // Query the API endpoint to find the approximate last block of the current month
    let end_response = reqwest::get(&end_url).await?.text().await?;
    debug!("Response: {}", end_response);
    let end: TimeStampResponse = serde_json::from_str(&end_response)?;
    debug!("Ending with block: {:?}", end);

    let start_timestamp = DateTime::parse_from_rfc3339(&start.timestamp)?.timestamp();
    let start = if start_timestamp < start_time.timestamp() {
        start.height + 1
    } else {
        start.height
    };

    let end = end.height;
    info!("Querying block data from {} to {}", start, end);
    info!("querying for {} blocks", end - start + 1);
    let bulk_url = format!("http://{}/api/v1/blocks-bulk/{}/{}", url, start, end);
    debug!("Bulk URL: {}", bulk_url);
    let body = reqwest::get(&bulk_url).await?.text().await?;
    let bulk_disabled = body.contains("config.MEMPOOL.MAX_BLOCKS_BULK_QUERY");
    if bulk_disabled {
        error!("Bulk blocks query is disabled, use different mempool instance");
        return Ok(());
    }

    let jd = &mut serde_json::Deserializer::from_str(&body);

    let result: Result<Vec<Block>, _> = serde_path_to_error::deserialize(jd);
    match result {
        Ok(block_data) => {
            let filename = format!("block_report_from_{}_to_{}.csv", start, end);
            info!(
                "writing {} blocks of data to {}",
                block_data.len(),
                &filename
            );
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

#[derive(Deserialize, Debug)]
struct TimeStampResponse {
    height: u64,
    hash: String,
    timestamp: String,
}
