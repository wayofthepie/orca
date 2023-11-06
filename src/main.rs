mod hetzner;

use std::time::{Duration, Instant};

use hcloud::{
    apis::{
        configuration::Configuration,
        servers_api::{self, CreateServerParams, GetServerParams},
    },
    models::{server::Status, CreateServerRequest, CreateServerResponse, GetServerResponse},
};
use hetzner::{
    client::HCloud,
    models::{HCloudLocation, HCloudServerType},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut configuration = Configuration::new();
    configuration.bearer_access_token = std::env::var("HCLOUD_TOKEN").ok();
    let servers = servers_api::list_servers(&configuration, Default::default())
        .await?
        .servers;
    for server in servers {
        println!("{} {:?}", server.name, server.status);
    }
    Ok(())
}

pub struct OrcaHCloud {
    hcloud: Box<dyn HCloud>,
}

impl OrcaHCloud {
    pub async fn create_server(
        &self,
        name: &str,
        image_id: &str,
        server_type: HCloudServerType,
        location: HCloudLocation,
    ) -> Result<CreateServerResponse, Box<dyn std::error::Error>> {
        let request = CreateServerRequest {
            location: Some(location.to_string()),
            name: name.to_owned(),
            image: image_id.to_owned(),
            server_type: server_type.to_string(),
            ..Default::default()
        };
        let params = CreateServerParams {
            create_server_request: Some(request),
        };
        Ok(self.hcloud.create_server(params).await?)
    }

    pub async fn get_server(
        &self,
        server_id: i64,
    ) -> Result<GetServerResponse, Box<dyn std::error::Error>> {
        Ok(self
            .hcloud
            .get_server(GetServerParams { id: server_id })
            .await?)
    }

    pub async fn wait_until_running(
        &self,
        server_id: i64,
        timeout: Duration,
        tick: Duration,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut response = self.get_server(server_id).await?;
        let now = Instant::now();
        while response.server.unwrap().status != Status::Running {
            response = self.get_server(server_id).await?;
            tokio::time::sleep(tick).await;
            if now.elapsed() > timeout {
                return Err(format!("Timeout {:?} reached!", timeout).into());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use async_trait::async_trait;
    use hcloud::{
        apis::servers_api::{
            CreateServerError, CreateServerParams, GetServerError, GetServerParams,
        },
        models::{server::Status, CreateServerResponse, GetServerResponse, Server},
    };

    use crate::{HCloud, HCloudLocation, HCloudServerType, OrcaHCloud};

    #[derive(Default)]
    struct FakeClient {
        get_server_response: Option<GetServerResponse>,
    }

    #[async_trait]
    impl HCloud for FakeClient {
        async fn create_server(
            &self,
            _: CreateServerParams,
        ) -> Result<CreateServerResponse, hcloud::apis::Error<CreateServerError>> {
            Ok(CreateServerResponse::default())
        }
        async fn get_server(
            &self,
            _: GetServerParams,
        ) -> Result<GetServerResponse, hcloud::apis::Error<GetServerError>> {
            Ok(self.get_server_response.clone().unwrap_or_default())
        }
    }

    #[tokio::test]
    async fn orca_create_server_should_be_ok() {
        let hcloud = Box::<FakeClient>::default();
        let orca_hcloud = OrcaHCloud { hcloud };
        let response = orca_hcloud
            .create_server(
                "test",
                "test",
                HCloudServerType::CAX11,
                HCloudLocation::FSN1,
            )
            .await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn orca_get_server_should_be_ok() {
        let hcloud = Box::<FakeClient>::default();
        let orca_hcloud = OrcaHCloud { hcloud };
        let response = orca_hcloud.get_server(12345).await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn orca_wait_until_running_should_timeout_after_given_time() {
        let hcloud = Box::new(FakeClient {
            get_server_response: Some(GetServerResponse {
                server: Some(Box::new(Server {
                    status: Status::Initializing,
                    ..Server::default()
                })),
            }),
        });
        let orca_hcloud = OrcaHCloud { hcloud };
        let response = orca_hcloud
            .wait_until_running(12345, Duration::from_millis(10), Duration::from_millis(1))
            .await;
        assert!(response.is_err());
        assert_eq!(response.unwrap_err().to_string(), "Timeout 10ms reached!");
    }
}
