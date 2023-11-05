use std::{
    fmt::Display,
    time::{Duration, Instant},
};

use async_trait::async_trait;
use hcloud::{
    apis::{
        configuration::Configuration,
        servers_api::{
            self, CreateServerError, CreateServerParams, GetServerError, GetServerParams,
        },
    },
    models::{server::Status, CreateServerRequest, CreateServerResponse, GetServerResponse},
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

enum HCloudServerType {
    CAX11,
}

impl Display for HCloudServerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HCloudServerType::CAX11 => "cax11",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
enum HCloudLocation {
    FSN1,
}

impl Display for HCloudLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HCloudLocation::FSN1 => "fsn1",
        };
        write!(f, "{}", s)
    }
}

#[async_trait]
trait HCloud {
    async fn create_server(
        &self,
        params: CreateServerParams,
    ) -> Result<CreateServerResponse, hcloud::apis::Error<CreateServerError>>;

    async fn get_server(
        &self,
        params: GetServerParams,
    ) -> Result<GetServerResponse, hcloud::apis::Error<GetServerError>>;
}

struct HCloudClient {
    configuration: Configuration,
}

#[async_trait]
impl HCloud for HCloudClient {
    async fn create_server(
        &self,
        params: CreateServerParams,
    ) -> Result<CreateServerResponse, hcloud::apis::Error<CreateServerError>> {
        servers_api::create_server(&self.configuration, params).await
    }
    async fn get_server(
        &self,
        params: GetServerParams,
    ) -> Result<GetServerResponse, hcloud::apis::Error<GetServerError>> {
        servers_api::get_server(&self.configuration, params).await
    }
}

struct OrcaHCloud {
    hcloud: Box<dyn HCloud>,
}

impl OrcaHCloud {
    async fn create_server(
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

    async fn get_server(
        &self,
        server_id: i64,
    ) -> Result<GetServerResponse, Box<dyn std::error::Error>> {
        Ok(self
            .hcloud
            .get_server(GetServerParams { id: server_id })
            .await?)
    }

    async fn wait_until_running(
        &self,
        server_id: i64,
        timeout: Duration,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut response = self.get_server(server_id).await?;
        let now = Instant::now();
        while response.server.unwrap().status != Status::Running {
            response = self.get_server(server_id).await?;
            tokio::time::sleep(Duration::from_secs(5)).await;
            if now.elapsed() > timeout {
                return Err(format!("Timeout {:?} reached!", timeout).into());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;
    use hcloud::{
        apis::servers_api::{
            CreateServerError, CreateServerParams, GetServerError, GetServerParams,
        },
        models::{CreateServerResponse, GetServerResponse},
    };

    use crate::{HCloud, HCloudLocation, HCloudServerType, OrcaHCloud};

    struct FakeClient {}

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
            Ok(GetServerResponse::default())
        }
    }

    #[tokio::test]
    async fn orca_create_server_should_be_ok() {
        let hcloud = Box::new(FakeClient {});
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
        let hcloud = Box::new(FakeClient {});
        let orca_hcloud = OrcaHCloud { hcloud };
        let response = orca_hcloud.get_server(12345).await;
        assert!(response.is_ok());
    }
}
