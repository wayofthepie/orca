use hcloud::apis::configuration::Configuration;
use orca::{hetzner::client::HCloudClient, orca::OrcaHCloud};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut configuration = Configuration::new();
    configuration.bearer_access_token = std::env::var("HCLOUD_TOKEN").ok();
    let hcloud = Box::new(HCloudClient::new(configuration));
    let _orca = OrcaHCloud::new(hcloud);
    // do stuff

    Ok(())
}
