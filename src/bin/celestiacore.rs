use std::{process::Command as ProcessCommand, str::FromStr};

use celestia_core_rpc::client::CompatMode;
use celestia_core_rpc::dialect::{Dialect, LatestDialect};
use celestia_core_rpc::event::DialectEvent;
use celestia_core_rpc::query::Query;
use celestia_core_rpc::request::{RequestMessage, Wrapper as RequestWrapper};
use celestia_core_rpc::{
    endpoint, Client, Error, HttpClient, Order, Paging, PageNumber, PerPage, Scheme,
    Subscription, SubscriptionClient, Url, WebSocketClient,
};
use futures::StreamExt;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use structopt::StructOpt;
use subtle_encoding::hex;
use tendermint::{block::Height, Hash};
use tokio::task::JoinHandle;
use tokio::time::Duration;

#[derive(Debug, StructOpt)]
#[structopt(name = "celestiacore")]
struct Opt {
    /// RPC endpoint URL
    #[structopt(
        long,
        default_value = "http://127.0.0.1:26657",
        env = "CELESTIA_RPC_URL",
        global = true
    )]
    url: Url,

    /// RPC compatibility mode (v0.34, v0.37, v0.38)
    #[structopt(long, global = true)]
    compat: Option<CompatMode>,

    /// Print the JSON-RPC request payload
    #[structopt(long, global = true)]
    show_request: bool,

    /// Print a curl command for the request
    #[structopt(long, global = true)]
    show_curl: bool,

    /// Run curl and compare results
    #[structopt(long, global = true)]
    run_curl: bool,

    /// JSON pointer to compare (e.g. /sync_info)
    #[structopt(long, global = true)]
    compare_path: Option<String>,

    /// Curl binary to execute (when --run-curl)
    #[structopt(long, default_value = "curl", global = true)]
    curl_bin: String,

    /// Print compact JSON output
    #[structopt(long, global = true)]
    compact: bool,

    /// Print bytes as numeric arrays instead of hex
    #[structopt(long, global = true)]
    bytes_array: bool,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    Health,
    Status,
    NetInfo,
    AbciInfo,
    AbciQuery {
        #[structopt(long)]
        path: Option<String>,
        data: String,
        #[structopt(long)]
        height: Option<u64>,
        #[structopt(long)]
        prove: bool,
    },
    Block { height: u64 },
    BlockByHash { hash: String },
    Blockchain { min: u64, max: u64 },
    BlockResults { height: u64 },
    BlockSearch {
        query: Query,
        #[structopt(long, default_value = "1")]
        page: u32,
        #[structopt(long, default_value = "10")]
        per_page: u8,
        #[structopt(long, default_value = "asc")]
        order: Order,
    },
    BroadcastTxAsync { tx_base64: String },
    BroadcastTxCommit { tx_base64: String },
    BroadcastTxSync { tx_base64: String },
    CheckTx { tx_base64: String },
    Commit { height: u64 },
    ConsensusParams { height: u64 },
    ConsensusState,
    DumpConsensusState,
    DataCommitment { start: u64, end: u64 },
    DataRootInclusionProof { height: u64, start: u64, end: u64 },
    Genesis,
    GenesisChunked { chunk: u32 },
    Header { height: u64 },
    HeaderByHash { hash: String },
    LatestBlock,
    LatestBlockResults,
    LatestCommit,
    LatestConsensusParams,
    NumUnconfirmedTxs,
    ProveShares {
        height: u64,
        start_share: u64,
        end_share: u64,
    },
    ProveSharesV2 {
        height: u64,
        start_share: u64,
        end_share: u64,
    },
    SignedBlock { height: u64 },
    Subscribe {
        query: Query,
        #[structopt(long)]
        max_events: Option<u32>,
        #[structopt(long)]
        max_time: Option<u32>,
    },
    Tx { hash: String, #[structopt(long)] prove: bool },
    TxSearch {
        query: Query,
        #[structopt(long, default_value = "1")]
        page: u32,
        #[structopt(long, default_value = "10")]
        per_page: u8,
        #[structopt(long, default_value = "asc")]
        order: Order,
        #[structopt(long)]
        prove: bool,
    },
    TxStatus { hash: String },
    TxStatusBatch { hashes: Vec<String> },
    UnconfirmedTxs { limit: Option<i64> },
    Validators {
        height: u64,
        #[structopt(long)]
        all: bool,
        #[structopt(long)]
        page: Option<usize>,
        #[structopt(long)]
        per_page: Option<u8>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    match &opt.cmd {
        Command::Subscribe {
            query,
            max_events,
            max_time,
        } => {
            let ws_url = websocket_url(&opt.url)?;
            let (client, driver_hdl) = start_websocket_client(ws_url, opt.compat).await?;
            let result = subscription_client_request(&client, query.clone(), *max_events, *max_time).await;
            stop_websocket_client(client, driver_hdl).await?;
            result?;
        }
        _ => match opt.url.scheme() {
            Scheme::Http | Scheme::Https => {
                let client = prepare_http_client(&opt.url, opt.compat).await?;
                execute_command(&opt, &client, &opt.cmd).await?;
            }
            Scheme::WebSocket | Scheme::SecureWebSocket => {
                let (client, driver_hdl) = start_websocket_client(opt.url.clone(), opt.compat).await?;
                let result = execute_command(&opt, &client, &opt.cmd).await;
                stop_websocket_client(client, driver_hdl).await?;
                result?;
            }
        },
    }

    Ok(())
}

async fn prepare_http_client(url: &Url, compat: Option<CompatMode>) -> Result<HttpClient, Error> {
    let mut client = HttpClient::new(url.clone())?;
    if let Some(compat) = compat {
        client.set_compat_mode(compat);
        return Ok(client);
    }
    let status = client.status().await?;
    let compat_mode = CompatMode::from_version(status.node_info.version)?;
    client.set_compat_mode(compat_mode);
    Ok(client)
}

async fn start_websocket_client(
    url: Url,
    compat: Option<CompatMode>,
) -> Result<(WebSocketClient, JoinHandle<Result<(), Error>>), Error> {
    let (client, driver) = if let Some(compat) = compat {
        WebSocketClient::builder(url.clone().try_into()?)
            .compat_mode(compat)
            .build()
            .await?
    } else {
        WebSocketClient::new(url.clone()).await?
    };
    let driver_hdl = tokio::spawn(async move { driver.run().await });

    if compat.is_none() {
        let status = client.status().await?;
        let compat_mode = CompatMode::from_version(status.node_info.version)?;
        if compat_mode != CompatMode::latest() {
            stop_websocket_client(client, driver_hdl).await?;
            let (client, driver) = WebSocketClient::builder(url.clone().try_into()?)
                .compat_mode(compat_mode)
                .build()
                .await?;
            let driver_hdl = tokio::spawn(async move { driver.run().await });
            return Ok((client, driver_hdl));
        }
    }

    Ok((client, driver_hdl))
}

async fn stop_websocket_client(
    client: WebSocketClient,
    driver_hdl: JoinHandle<Result<(), Error>>,
) -> Result<(), Error> {
    client.close()?;
    driver_hdl.await.map_err(Error::join)?
}

async fn subscription_client_request<C>(
    client: &C,
    query: Query,
    max_events: Option<u32>,
    max_time: Option<u32>,
) -> Result<(), Error>
where
    C: SubscriptionClient,
{
    let subs = client.subscribe(query).await?;
    match max_time {
        Some(secs) => recv_events_with_timeout(subs, max_events, secs).await,
        None => recv_events(subs, max_events).await,
    }
}

async fn recv_events_with_timeout(
    mut subs: Subscription,
    max_events: Option<u32>,
    timeout_secs: u32,
) -> Result<(), Error> {
    let timeout = tokio::time::sleep(Duration::from_secs(timeout_secs as u64));
    let mut event_count = 0u64;
    tokio::pin!(timeout);
    loop {
        tokio::select! {
            result_opt = subs.next() => {
                let result = match result_opt {
                    Some(r) => r,
                    None => return Ok(()),
                };
                let event: DialectEvent<<LatestDialect as Dialect>::Event> = result?.into();
                println!("{}", serde_json::to_string_pretty(&event).map_err(Error::serde)?);
                event_count += 1;
                if let Some(me) = max_events {
                    if event_count >= (me as u64) {
                        return Ok(());
                    }
                }
            }
            _ = &mut timeout => return Ok(()),
        }
    }
}

async fn recv_events(mut subs: Subscription, max_events: Option<u32>) -> Result<(), Error> {
    let mut event_count = 0u64;
    while let Some(result) = subs.next().await {
        let event: DialectEvent<<LatestDialect as Dialect>::Event> = result?.into();
        println!("{}", serde_json::to_string_pretty(&event).map_err(Error::serde)?);
        event_count += 1;
        if let Some(me) = max_events {
            if event_count >= (me as u64) {
                return Ok(());
            }
        }
    }
    Ok(())
}

async fn execute_command<C>(opt: &Opt, client: &C, cmd: &Command) -> Result<(), Error>
where
    C: Client + Sync,
{
    match cmd {
        Command::Health => execute_request(opt, client, endpoint::health::Request).await,
        Command::Status => execute_request(opt, client, endpoint::status::Request).await,
        Command::NetInfo => execute_request(opt, client, endpoint::net_info::Request).await,
        Command::AbciInfo => execute_request(opt, client, endpoint::abci_info::Request).await,
        Command::AbciQuery {
            path,
            data,
            height,
            prove,
        } => {
            let height = height.map(|h| parse_height(h)).transpose()?;
            execute_request(
                opt,
                client,
                endpoint::abci_query::Request::new(path.clone(), data.clone(), height, *prove),
            )
            .await
        }
        Command::Block { height } => {
            let height = parse_height(*height)?;
            execute_request(opt, client, endpoint::block::Request::new(height)).await
        }
        Command::BlockByHash { hash } => {
            let hash = parse_hash(hash.as_str())?;
            execute_request(opt, client, endpoint::block_by_hash::Request::new(hash)).await
        }
        Command::Blockchain { min, max } => {
            let min = parse_height(*min)?;
            let max = parse_height(*max)?;
            execute_request(opt, client, endpoint::blockchain::Request::new(min, max)).await
        }
        Command::BlockResults { height } => {
            let height = parse_height(*height)?;
            execute_request(opt, client, endpoint::block_results::Request::new(height)).await
        }
        Command::BlockSearch {
            query,
            page,
            per_page,
            order,
        } => {
            execute_request(
                opt,
                client,
                endpoint::block_search::Request::new(
                    query.clone(),
                    *page,
                    *per_page,
                    order.clone(),
                ),
            )
            .await
        }
        Command::BroadcastTxAsync { tx_base64 } => {
            let tx = decode_base64(tx_base64)?;
            execute_request(opt, client, endpoint::broadcast::tx_async::Request::new(tx)).await
        }
        Command::BroadcastTxCommit { tx_base64 } => {
            let tx = decode_base64(tx_base64)?;
            execute_request(opt, client, endpoint::broadcast::tx_commit::Request::new(tx)).await
        }
        Command::BroadcastTxSync { tx_base64 } => {
            let tx = decode_base64(tx_base64)?;
            execute_request(opt, client, endpoint::broadcast::tx_sync::Request::new(tx)).await
        }
        Command::CheckTx { tx_base64 } => {
            let tx = decode_base64(tx_base64)?;
            execute_request(opt, client, endpoint::check_tx::Request::new(tx)).await
        }
        Command::Commit { height } => {
            let height = parse_height(*height)?;
            execute_request(opt, client, endpoint::commit::Request::new(height)).await
        }
        Command::ConsensusParams { height } => {
            let height = parse_height(*height)?;
            execute_request(
                opt,
                client,
                endpoint::consensus_params::Request::new(Some(height)),
            )
            .await
        }
        Command::ConsensusState => {
            execute_request(opt, client, endpoint::consensus_state::Request::new()).await
        }
        Command::DumpConsensusState => {
            execute_request(opt, client, endpoint::dump_consensus_state::Request).await
        }
        Command::DataCommitment { start, end } => {
            execute_request(opt, client, endpoint::data_commitment::Request::new(*start, *end))
                .await
        }
        Command::DataRootInclusionProof { height, start, end } => {
            execute_request(
                opt,
                client,
                endpoint::data_root_inclusion_proof::Request::new(*height, *start, *end),
            )
            .await
        }
        Command::Genesis => {
            execute_request(
                opt,
                client,
                endpoint::genesis::Request::<serde_json::Value>::default(),
            )
            .await
        }
        Command::GenesisChunked { chunk } => {
            execute_request(opt, client, endpoint::genesis_chunked::Request::new(*chunk)).await
        }
        Command::Header { height } => {
            let height = parse_height(*height)?;
            execute_request(opt, client, endpoint::header::Request::new(height)).await
        }
        Command::HeaderByHash { hash } => {
            let hash = parse_hash(hash.as_str())?;
            execute_request(opt, client, endpoint::header_by_hash::Request::new(hash)).await
        }
        Command::LatestBlock => execute_request(opt, client, endpoint::block::Request::default()).await,
        Command::LatestBlockResults => {
            execute_request(opt, client, endpoint::block_results::Request::default()).await
        }
        Command::LatestCommit => execute_request(opt, client, endpoint::commit::Request::default()).await,
        Command::LatestConsensusParams => {
            execute_request(opt, client, endpoint::consensus_params::Request::default()).await
        }
        Command::NumUnconfirmedTxs => {
            execute_request(opt, client, endpoint::num_unconfirmed_txs::Request).await
        }
        Command::ProveShares {
            height,
            start_share,
            end_share,
        } => {
            execute_request(
                opt,
                client,
                endpoint::prove_shares::Request::new(*height, *start_share, *end_share),
            )
            .await
        }
        Command::ProveSharesV2 {
            height,
            start_share,
            end_share,
        } => {
            execute_request(
                opt,
                client,
                endpoint::prove_shares_v2::Request::new(*height, *start_share, *end_share),
            )
            .await
        }
        Command::SignedBlock { height } => {
            let height = parse_height(*height)?;
            execute_request(opt, client, endpoint::signed_block::Request::new(height)).await
        }
        Command::Tx { hash, prove } => {
            let hash = parse_hash(hash.as_str())?;
            execute_request_raw(opt, client, endpoint::tx::Request::new(hash, *prove)).await
        }
        Command::TxSearch {
            query,
            page,
            per_page,
            order,
            prove,
        } => {
            execute_request_raw(
                opt,
                client,
                endpoint::tx_search::Request::new(
                    query.clone(),
                    *prove,
                    *page,
                    *per_page,
                    order.clone(),
                ),
            )
            .await
        }
        Command::TxStatus { hash } => {
            let hash = parse_hash(hash.as_str())?;
            execute_request(opt, client, endpoint::tx_status::Request::new(hash)).await
        }
        Command::TxStatusBatch { hashes } => {
            let hashes = hashes
                .iter()
                .map(|hash| parse_hash(hash.as_str()))
                .collect::<Result<Vec<_>, _>>()?;
            execute_request(opt, client, endpoint::tx_status_batch::Request::new(hashes)).await
        }
        Command::UnconfirmedTxs { limit } => {
            execute_request(opt, client, endpoint::unconfirmed_txs::Request::new(*limit)).await
        }
        Command::Validators {
            height,
            all,
            page,
            per_page,
        } => {
            let height = parse_height(*height)?;
            if *all {
                if opt.run_curl {
                    eprintln!("--run-curl is not supported with --all");
                }
                let result = client.validators(height, Paging::All).await?;
                print_output(opt, &result)?;
                Ok(())
            } else {
                let paging = match (page, per_page) {
                    (Some(page), Some(per_page)) => Paging::Specific {
                        page_number: PageNumber::from(*page),
                        per_page: PerPage::from(*per_page),
                    },
                    _ => Paging::Default,
                };
                let req = match paging {
                    Paging::Specific {
                        page_number,
                        per_page,
                    } => endpoint::validators::Request::new(
                        Some(height),
                        Some(page_number),
                        Some(per_page),
                    ),
                    _ => endpoint::validators::Request::new(Some(height), None, None),
                };
                execute_request(opt, client, req).await
            }
        }
        Command::Subscribe { .. } => Err(Error::invalid_params(
            "subscribe requires a WebSocket client".to_string(),
        )),
    }
}

async fn execute_request<C, R>(opt: &Opt, client: &C, request: R) -> Result<(), Error>
where
    C: Client + Sync,
    R: RequestMessage + celestia_core_rpc::SimpleRequest + Clone,
    R::Output: Serialize + DeserializeOwned,
{
    let payload = if opt.show_request || opt.show_curl || opt.run_curl {
        Some(request_payload(request.clone()))
    } else {
        None
    };

    if let Some(payload) = payload.as_ref() {
        if opt.show_request {
            eprintln!("Request payload:\n{payload}\n");
        }
        if opt.show_curl || opt.run_curl {
            let curl_url = curl_url(&opt.url);
            eprintln!(
                "Curl command:\ncurl -sS -H 'Content-Type: application/json' -d {} {}\n",
                shell_escape_single(payload),
                shell_escape_single(curl_url.as_str())
            );
        }
    }

    let result = client.perform(request.clone()).await?;
    print_output(opt, &result)?;

    if opt.run_curl {
        let payload = payload.unwrap_or_else(|| request_payload(request.clone()));
        compare_with_curl(opt, payload, &result)?;
    }

    Ok(())
}

async fn execute_request_raw<C, R>(opt: &Opt, client: &C, request: R) -> Result<(), Error>
where
    C: Client + Sync,
    R: RequestMessage + celestia_core_rpc::SimpleRequest + Clone,
    R::Output: Serialize,
{
    let payload = if opt.show_request || opt.show_curl || opt.run_curl {
        Some(request_payload(request.clone()))
    } else {
        None
    };

    if let Some(payload) = payload.as_ref() {
        if opt.show_request {
            eprintln!("Request payload:\n{payload}\n");
        }
        if opt.show_curl || opt.run_curl {
            let curl_url = curl_url(&opt.url);
            eprintln!(
                "Curl command:\ncurl -sS -H 'Content-Type: application/json' -d {} {}\n",
                shell_escape_single(payload),
                shell_escape_single(curl_url.as_str())
            );
        }
    }

    let result = client.perform(request.clone()).await?;
    print_output(opt, &result)?;

    if opt.run_curl {
        let payload = payload.unwrap_or_else(|| request_payload(request.clone()));
        compare_with_curl_raw(opt, payload, &result)?;
    }

    Ok(())
}

fn request_payload<R>(request: R) -> String
where
    R: RequestMessage,
{
    serde_json::to_string(&RequestWrapper::new(request)).unwrap()
}

fn print_output<T>(opt: &Opt, value: &T) -> Result<(), Error>
where
    T: Serialize,
{
    let mut json = serde_json::to_value(value).map_err(Error::serde)?;
    if !opt.bytes_array {
        json = encode_bytes_as_hex(json);
    }
    let output = if opt.compact {
        serde_json::to_string(&json).map_err(Error::serde)?
    } else {
        serde_json::to_string_pretty(&json).map_err(Error::serde)?
    };
    println!("{output}");
    Ok(())
}

fn encode_bytes_as_hex(value: Value) -> Value {
    match value {
        Value::Array(values) => {
            if !values.is_empty() && values.iter().all(is_byte_value) {
                let bytes = values
                    .into_iter()
                    .filter_map(|value| value.as_u64().map(|num| num as u8))
                    .collect::<Vec<u8>>();
                let encoded = hex::encode_upper(bytes);
                let encoded = String::from_utf8(encoded).unwrap_or_default();
                Value::String(encoded)
            } else {
                Value::Array(values.into_iter().map(encode_bytes_as_hex).collect())
            }
        }
        Value::Object(map) => Value::Object(
            map.into_iter()
                .map(|(key, value)| (key, encode_bytes_as_hex(value)))
                .collect(),
        ),
        other => other,
    }
}

fn is_byte_value(value: &Value) -> bool {
    value
        .as_u64()
        .map(|num| num <= u8::MAX as u64)
        .unwrap_or(false)
}

fn parse_height(value: u64) -> Result<Height, Error> {
    Height::try_from(value).map_err(|e| Error::parse(e.to_string()))
}

fn parse_hash(value: &str) -> Result<Hash, Error> {
    Hash::from_str(value).map_err(|e| Error::parse(e.to_string()))
}

fn decode_base64(value: &str) -> Result<Vec<u8>, Error> {
    subtle_encoding::base64::decode(value).map_err(|e| Error::parse(e.to_string()))
}

fn websocket_url(url: &Url) -> Result<Url, Error> {
    match url.scheme() {
        Scheme::WebSocket | Scheme::SecureWebSocket => Ok(url.clone()),
        Scheme::Http => to_ws_url(url, "ws"),
        Scheme::Https => to_ws_url(url, "wss"),
    }
}

fn to_ws_url(url: &Url, scheme: &str) -> Result<Url, Error> {
    let mut url_str = url.to_string();
    if url_str.starts_with("http://") {
        url_str = url_str.replacen("http://", "ws://", 1);
    } else if url_str.starts_with("https://") {
        url_str = url_str.replacen("https://", "wss://", 1);
    } else if !url_str.starts_with("ws://") && !url_str.starts_with("wss://") {
        url_str = format!("{scheme}://{url_str}");
    }
    if !url_str.ends_with("/websocket") {
        if url_str.ends_with('/') {
            url_str.push_str("websocket");
        } else {
            url_str.push_str("/websocket");
        }
    }
    url_str.parse()
}

fn curl_url(url: &Url) -> String {
    let mut url_str = url.to_string();
    if url_str.starts_with("ws://") {
        url_str = url_str.replacen("ws://", "http://", 1);
    } else if url_str.starts_with("wss://") {
        url_str = url_str.replacen("wss://", "https://", 1);
    }
    if url_str.ends_with("/websocket") {
        url_str.truncate(url_str.len() - "/websocket".len());
    }
    url_str
}

fn compare_with_curl<R>(
    opt: &Opt,
    payload: String,
    result: &R,
) -> Result<(), Error>
where
    R: Serialize + DeserializeOwned,
{
    let curl_url = curl_url(&opt.url);
    let client_value = serde_json::to_value(result).map_err(Error::serde)?;
    let curl_value = run_curl(&opt.curl_bin, curl_url.as_str(), payload.as_str())?;

    let curl_result = curl_value.get("result").cloned().unwrap_or(Value::Null);
    let compare_path = opt.compare_path.as_deref();
    let client_view = select_value(&client_value, compare_path)?;
    let curl_view = select_value(&curl_result, compare_path)?;

    let raw_matches = client_view == curl_view;
    println!("Raw result match: {raw_matches}");

    match serde_json::from_value::<R>(curl_result.clone()) {
        Ok(parsed) => {
            let normalized = serde_json::to_value(parsed).map_err(Error::serde)?;
            let normalized_view = select_value(&normalized, compare_path)?;
            let normalized_matches = client_view == normalized_view;
            println!("Normalized result match: {normalized_matches}");
            if !normalized_matches {
                let mut diffs = Vec::new();
                diff_values(normalized_view, client_view, String::new(), &mut diffs, 10);
                if !diffs.is_empty() {
                    println!("Differences (first 10):");
                    for diff in diffs {
                        println!("- {diff}");
                    }
                }
            }
        }
        Err(err) => {
            println!("Normalized compare skipped: {err}");
        }
    }

    Ok(())
}

fn compare_with_curl_raw<R>(
    opt: &Opt,
    payload: String,
    result: &R,
) -> Result<(), Error>
where
    R: Serialize,
{
    let curl_url = curl_url(&opt.url);
    let client_value = serde_json::to_value(result).map_err(Error::serde)?;
    let curl_value = run_curl(&opt.curl_bin, curl_url.as_str(), payload.as_str())?;

    let curl_result = curl_value.get("result").cloned().unwrap_or(Value::Null);
    let compare_path = opt.compare_path.as_deref();
    let client_view = select_value(&client_value, compare_path)?;
    let curl_view = select_value(&curl_result, compare_path)?;

    let raw_matches = client_view == curl_view;
    println!("Raw result match: {raw_matches}");
    println!("Normalized compare skipped: response type does not support deserialization");

    Ok(())
}

fn run_curl(
    curl_bin: &str,
    url: &str,
    payload: &str,
) -> Result<Value, Error> {
    let output = ProcessCommand::new(curl_bin)
        .args(["-sS", "-H", "Content-Type: application/json", "-d", payload, url])
        .output()
        .map_err(Error::io)?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::client_internal(format!("curl failed: {stderr}")));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let value: Value = serde_json::from_str(stdout.trim()).map_err(Error::serde)?;
    Ok(value)
}

fn select_value<'a>(value: &'a Value, path: Option<&str>) -> Result<&'a Value, Error> {
    let Some(path) = path else {
        return Ok(value);
    };

    let pointer = normalize_pointer(path);
    value
        .pointer(&pointer)
        .ok_or_else(|| Error::invalid_params(format!("compare path not found: {pointer}")))
}

fn normalize_pointer(path: &str) -> String {
    if path.is_empty() {
        return String::from("/");
    }
    if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{path}")
    }
}

fn diff_values(
    left: &Value,
    right: &Value,
    path: String,
    diffs: &mut Vec<String>,
    limit: usize,
) {
    if diffs.len() >= limit || left == right {
        return;
    }

    match (left, right) {
        (Value::Object(left), Value::Object(right)) => {
            let mut keys: Vec<&String> = left.keys().chain(right.keys()).collect();
            keys.sort();
            keys.dedup();
            for key in keys {
                if diffs.len() >= limit {
                    return;
                }
                let next_path = format!("{}/{}", path, key);
                let left_val = left.get(key).unwrap_or(&Value::Null);
                let right_val = right.get(key).unwrap_or(&Value::Null);
                diff_values(left_val, right_val, next_path, diffs, limit);
            }
        }
        (Value::Array(left), Value::Array(right)) => {
            let max_len = left.len().max(right.len());
            for idx in 0..max_len {
                if diffs.len() >= limit {
                    return;
                }
                let next_path = format!("{}/{}", path, idx);
                let left_val = left.get(idx).unwrap_or(&Value::Null);
                let right_val = right.get(idx).unwrap_or(&Value::Null);
                diff_values(left_val, right_val, next_path, diffs, limit);
            }
        }
        _ => {
            diffs.push(format!("{path}: client={} curl={}", left, right));
        }
    }
}

fn shell_escape_single(input: &str) -> String {
    let mut escaped = String::from("'");
    for ch in input.chars() {
        if ch == '\'' {
            escaped.push_str("'\\''");
        } else {
            escaped.push(ch);
        }
    }
    escaped.push('\'');
    escaped
}
