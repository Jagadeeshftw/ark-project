use dotenv::dotenv;
use reqwest::Client;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::error::Error;

pub async fn fetch_block(
    client: &Client,
    block_number: u64,
) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
    dotenv().ok();
    let rpc_provider = env::var("RPC_PROVIDER").expect("RPC_PROVIDER must be set");
    let payload = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "starknet_getEvents",
        "params": {
            "filter": {
                "from_block": {
                  "block_number": block_number
                },
                "to_block": {
                  "block_number": block_number
                },
                "chunk_size": 1000,
              }
        }
    });

    let resp = client.post(rpc_provider).json(&payload).send().await?;
    let block: HashMap<String, Value> = resp.json().await?;

    Ok(block)
}

pub async fn call_contract(
    client: &Client,
    contract_address: &str,
    block_id: u64,
    selector: &str,
    calldata: Vec<&str>,
) -> Result<Value, Box<dyn Error>> {
    let payload = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "starknet_call",
        "params": {
            "request": {
                "contract_address": contract_address,
                "entry_point_selector": selector,
                "calldata": calldata,
                "signature": []
            },
            "block_id": {
                "block_number": block_id
            }
        }
    });

    let response = client
        .post("https://starknet-mainnet.infura.io/v3/ccedf270a2a14a418fe9303865844cb7")
        .json(&payload)
        .send()
        .await?;

    let result: Value = response.json().await?;

    Ok(result.get("result").cloned().unwrap_or(Value::Null))
}

pub async fn get_latest_block(client: &Client) -> Result<u64, Box<dyn Error>> {
    let payload: Value = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "starknet_blockNumber",
        "params": {}
    });
    let response = client
        .post("https://starknet-mainnet.infura.io/v3/ccedf270a2a14a418fe9303865844cb7")
        .json(&payload)
        .send()
        .await?;

    let result: Value = response.json().await?;
    let block_number = result
        .get("result")
        .and_then(Value::as_u64)
        .ok_or("Failed to parse block number")?;
    Ok(block_number)
}
