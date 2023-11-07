use async_trait::async_trait;
use hcloud::{
    apis::{
        configuration::Configuration,
        servers_api::{
            self, CreateServerError, CreateServerParams, DeleteServerError, DeleteServerParams,
            GetServerError, GetServerParams,
        },
    },
    models::{CreateServerResponse, DeleteServerResponse, GetServerResponse},
};

#[async_trait]
pub trait HCloud {
    async fn create_server(
        &self,
        params: CreateServerParams,
    ) -> Result<CreateServerResponse, hcloud::apis::Error<CreateServerError>>;

    async fn get_server(
        &self,
        params: GetServerParams,
    ) -> Result<GetServerResponse, hcloud::apis::Error<GetServerError>>;

    async fn delete_server(
        &self,
        params: DeleteServerParams,
    ) -> Result<DeleteServerResponse, hcloud::apis::Error<DeleteServerError>>;
}

pub struct HCloudClient {
    configuration: Configuration,
}

impl HCloudClient {
    pub fn new(configuration: Configuration) -> Self {
        Self { configuration }
    }
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

    async fn delete_server(
        &self,
        params: DeleteServerParams,
    ) -> Result<DeleteServerResponse, hcloud::apis::Error<DeleteServerError>> {
        servers_api::delete_server(&self.configuration, params).await
    }
}
