use celestia_rpc::client::Client;
use celestia_rpc::endpoint::data_root_inclusion_proof::Response as DataRootInclusionProofResponse;
use celestia_rpc::endpoint::prove_shares::Response as ProveSharesResponse;
use celestia_rpc::HttpClient;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rpc_endpoint = "http://consensus-full-mocha-4.celestia-mocha.com:26657";
    let client = HttpClient::new(rpc_endpoint).unwrap();

    let height: u64 = 1593400;
    let start: u64 = 1593300;
    let end: u64 = 1593600;

    let data_root_inclusion_proof_response: DataRootInclusionProofResponse =
        client.data_root_inclusion_proof(height, start, end).await?;

    println!(
        "Data Root Inclusion Proof {:?}",
        data_root_inclusion_proof_response
    );

    let start_share: u64 = 0;
    let end_share: u64 = 1;

    for i in 0..5 {
        let prove_shares_response: ProveSharesResponse = client
            .prove_shares(height + i, start_share, end_share)
            .await?;

        println!("Shares Proof{:?}", prove_shares_response);
    }

    Ok(())
}
