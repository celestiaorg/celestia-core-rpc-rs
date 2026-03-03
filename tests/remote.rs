#![cfg(feature = "http-client")]

use std::{cmp, env, str::FromStr};

use celestia_core_rpc::client::CompatMode;
use celestia_core_rpc::{Client, HttpClient, Paging};
use subtle_encoding::base64;
use tendermint::{block::Height, Hash};

fn log_enabled() -> bool {
    env::var("CELESTIA_RPC_LOG")
        .map(|value| value != "0")
        .unwrap_or(false)
}

fn log(message: impl AsRef<str>) {
    if log_enabled() {
        eprintln!("[remote] {}", message.as_ref());
    }
}

fn rpc_client() -> Option<HttpClient> {
    let url = match env::var("CELESTIA_RPC_URL") {
        Ok(url) => url,
        Err(_) => {
            eprintln!("CELESTIA_RPC_URL not set; skipping remote RPC tests");
            return None;
        }
    };

    let mut client = HttpClient::new(url.as_str()).expect("invalid CELESTIA_RPC_URL");

    if let Ok(compat) = env::var("CELESTIA_RPC_COMPAT") {
        let compat = compat
            .parse::<CompatMode>()
            .expect("invalid CELESTIA_RPC_COMPAT (expected v0.34, v0.37, v0.38)");
        client.set_compat_mode(compat);
    }

    Some(client)
}

fn env_u64(name: &str) -> Option<u64> {
    env::var(name).ok().and_then(|value| value.parse().ok())
}

fn range_for(height: u64) -> (u64, u64) {
    let end = height.saturating_add(1);
    let start = cmp::max(1, height.saturating_sub(10));
    (start, end)
}

fn block_range_for(height: Height) -> (Height, Height) {
    let height_value = height.value();
    let min_value = cmp::max(1, height_value.saturating_sub(1));
    let min = Height::try_from(min_value).expect("invalid min height");
    (min, height)
}

async fn remote_height(client: &HttpClient) -> u64 {
    if let Some(height) = env_u64("CELESTIA_RPC_HEIGHT") {
        return height;
    }

    let status = client.status().await.expect("status request failed");
    status.sync_info.latest_block_height.value()
}

async fn remote_height_typed(client: &HttpClient) -> Height {
    let height = remote_height(client).await;
    Height::try_from(height).expect("invalid CELESTIA_RPC_HEIGHT")
}

#[tokio::test]
async fn health_smoke() {
    let Some(client) = rpc_client() else {
        return;
    };

    log("health");
    client.health().await.expect("health request failed");
}

#[tokio::test]
async fn status_smoke() {
    let Some(client) = rpc_client() else {
        return;
    };

    log("status");
    let status = client.status().await.expect("status request failed");
    assert!(status.sync_info.latest_block_height.value() > 0);
}

#[tokio::test]
async fn block_matches_status_height() {
    let Some(client) = rpc_client() else {
        return;
    };

    let height = remote_height_typed(&client).await;
    log(format!("block height={}", height.value()));

    let block = client.block(height).await.expect("block request failed");
    assert_eq!(height, block.block.header.height);
}

#[tokio::test]
async fn signed_block_matches_height() {
    let Some(client) = rpc_client() else {
        return;
    };

    let height = remote_height_typed(&client).await;
    log(format!("signed_block height={}", height.value()));

    let block = client
        .signed_block(height)
        .await
        .expect("signed_block request failed");
    assert_eq!(height, block.header.height);
}

#[tokio::test]
async fn genesis_chunked_smoke() {
    let Some(client) = rpc_client() else {
        return;
    };

    log("genesis_chunked chunk=0");
    let chunk = client
        .genesis_chunked(0)
        .await
        .expect("genesis_chunked request failed");
    assert_eq!(0, chunk.chunk);
    assert!(chunk.total > 0);
    assert!(!chunk.data.is_empty());
}

#[tokio::test]
async fn data_root_inclusion_proof_smoke() {
    let Some(client) = rpc_client() else {
        return;
    };

    let height = remote_height(&client).await;
    let (start, end) = range_for(height);
    log(format!("data_root_inclusion_proof height={height} start={start} end={end}"));
    let resp = client
        .data_root_inclusion_proof(height, start, end)
        .await
        .expect("data_root_inclusion_proof request failed");
    assert!(resp.proof.is_some());
}

#[tokio::test]
async fn data_commitment_smoke() {
    let Some(client) = rpc_client() else {
        return;
    };

    let height = remote_height(&client).await;
    let (start, end) = range_for(height);
    log(format!("data_commitment start={start} end={end}"));
    let resp = client
        .data_commitment(start, end)
        .await
        .expect("data_commitment request failed");
    assert!(!resp.data_commitment.is_empty());
}

#[tokio::test]
async fn prove_shares_v2_smoke() {
    let Some(client) = rpc_client() else {
        return;
    };

    let height = remote_height(&client).await;
    log(format!("prove_shares_v2 height={height} start_share=0 end_share=1"));
    let resp = client
        .prove_shares_v2(height, 0, 1)
        .await
        .expect("prove_shares_v2 request failed");
    assert!(!resp.share_proof.data.is_empty());
}

#[tokio::test]
async fn unconfirmed_txs_smoke() {
    let Some(client) = rpc_client() else {
        return;
    };

    log("unconfirmed_txs");
    let resp = client
        .unconfirmed_txs(None)
        .await
        .expect("unconfirmed_txs request failed");
    assert!(resp.total >= resp.count);
    assert!(resp.total_bytes >= 0);
}

#[tokio::test]
async fn num_unconfirmed_txs_smoke() {
    let Some(client) = rpc_client() else {
        return;
    };

    log("num_unconfirmed_txs");
    let resp = client
        .num_unconfirmed_txs()
        .await
        .expect("num_unconfirmed_txs request failed");
    assert!(resp.total >= resp.count);
    assert!(resp.total_bytes >= 0);
}

#[tokio::test]
async fn tx_status_optional() {
    let Some(client) = rpc_client() else {
        return;
    };

    let hash = match env::var("CELESTIA_RPC_TX_HASH") {
        Ok(value) => Hash::from_str(&value).expect("invalid CELESTIA_RPC_TX_HASH"),
        Err(_) => {
            log("tx_status skipped: CELESTIA_RPC_TX_HASH not set");
            return;
        }
    };

    log(format!("tx_status hash={hash}"));
    let resp = client
        .tx_status(hash)
        .await
        .expect("tx_status request failed");
    assert!(!resp.status.is_empty());
}

#[tokio::test]
async fn check_tx_optional() {
    let Some(client) = rpc_client() else {
        return;
    };

    let tx = match env::var("CELESTIA_RPC_TX_BASE64") {
        Ok(value) => base64::decode(value).expect("invalid CELESTIA_RPC_TX_BASE64"),
        Err(_) => {
            log("check_tx skipped: CELESTIA_RPC_TX_BASE64 not set");
            return;
        }
    };

    log("check_tx");
    let _resp = client
        .check_tx(tx)
        .await
        .expect("check_tx request failed");
}

#[tokio::test]
async fn abci_info_smoke() {
    let Some(client) = rpc_client() else {
        return;
    };

    log("abci_info");
    let resp = client.abci_info().await.expect("abci_info request failed");
    assert!(!resp.data.is_empty());
}

#[tokio::test]
async fn net_info_smoke() {
    let Some(client) = rpc_client() else {
        return;
    };

    log("net_info");
    let resp = client.net_info().await.expect("net_info request failed");
    assert!(resp.n_peers as usize >= resp.peers.len());
}

#[tokio::test]
async fn block_results_smoke() {
    let Some(client) = rpc_client() else {
        return;
    };

    let height = remote_height_typed(&client).await;
    log(format!("block_results height={}", height.value()));
    let resp = client
        .block_results(height)
        .await
        .expect("block_results request failed");
    assert_eq!(height, resp.height);
}

#[tokio::test]
async fn header_smoke() {
    let Some(client) = rpc_client() else {
        return;
    };

    let height = remote_height_typed(&client).await;
    log(format!("header height={}", height.value()));
    let resp = client.header(height).await.expect("header request failed");
    assert_eq!(height, resp.header.height);
}

#[tokio::test]
async fn commit_smoke() {
    let Some(client) = rpc_client() else {
        return;
    };

    let height = remote_height_typed(&client).await;
    log(format!("commit height={}", height.value()));
    let resp = client.commit(height).await.expect("commit request failed");
    assert_eq!(height, resp.signed_header.header.height);
}

#[tokio::test]
async fn validators_smoke() {
    let Some(client) = rpc_client() else {
        return;
    };

    let height = remote_height_typed(&client).await;
    log(format!("validators height={}", height.value()));
    let resp = client
        .validators(height, Paging::Default)
        .await
        .expect("validators request failed");
    assert_eq!(height, resp.block_height);
    assert!(resp.total >= resp.count);
}

#[tokio::test]
async fn consensus_params_smoke() {
    let Some(client) = rpc_client() else {
        return;
    };

    let height = remote_height_typed(&client).await;
    log(format!("consensus_params height={}", height.value()));
    let resp = client
        .consensus_params(height)
        .await
        .expect("consensus_params request failed");
    assert_eq!(height, resp.block_height);
}

#[tokio::test]
async fn blockchain_smoke() {
    let Some(client) = rpc_client() else {
        return;
    };

    let height = remote_height_typed(&client).await;
    let (min, max) = block_range_for(height);
    log(format!(
        "blockchain min={} max={}",
        min.value(),
        max.value()
    ));
    let resp = client
        .blockchain(min, max)
        .await
        .expect("blockchain request failed");
    assert!(resp.last_height >= max);
    assert!(!resp.block_metas.is_empty());
}

#[tokio::test]
async fn block_by_hash_smoke() {
    let Some(client) = rpc_client() else {
        return;
    };

    let height = remote_height_typed(&client).await;
    let block = client.block(height).await.expect("block request failed");
    let hash = block.block_id.hash;
    log(format!("block_by_hash hash={hash}"));
    let resp = client
        .block_by_hash(hash)
        .await
        .expect("block_by_hash request failed");
    assert!(resp.block.is_some());
}

#[tokio::test]
async fn header_by_hash_smoke() {
    let Some(client) = rpc_client() else {
        return;
    };

    let height = remote_height_typed(&client).await;
    let block = client.block(height).await.expect("block request failed");
    let hash = block.block_id.hash;
    log(format!("header_by_hash hash={hash}"));
    let resp = client
        .header_by_hash(hash)
        .await
        .expect("header_by_hash request failed");
    assert!(resp.header.is_some());
}

#[tokio::test]
async fn tx_status_batch_optional() {
    let Some(client) = rpc_client() else {
        return;
    };

    let hashes = match env::var("CELESTIA_RPC_TX_HASHES") {
        Ok(value) => value
            .split(',')
            .map(|part| Hash::from_str(part.trim()))
            .collect::<Result<Vec<_>, _>>()
            .expect("invalid CELESTIA_RPC_TX_HASHES"),
        Err(_) => {
            log("tx_status_batch skipped: CELESTIA_RPC_TX_HASHES not set");
            return;
        }
    };

    log(format!("tx_status_batch hashes={}", hashes.len()));
    let resp = client
        .tx_status_batch(hashes)
        .await
        .expect("tx_status_batch request failed");
    assert!(!resp.statuses.is_empty());
}

#[tokio::test]
async fn prove_shares_smoke() {
    let Some(client) = rpc_client() else {
        return;
    };

    let height = remote_height(&client).await;
    log(format!("prove_shares height={height} start_share=0 end_share=1"));
    let resp = client
        .prove_shares(height, 0, 1)
        .await
        .expect("prove_shares request failed");
    assert!(!resp.0.data.is_empty());
}
