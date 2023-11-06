use async_trait::async_trait;
use hcloud::{
    apis::{
        configuration::Configuration,
        servers_api::{
            self, CreateServerError, CreateServerParams, GetServerError, GetServerParams,
        },
    },
    models::{CreateServerResponse, GetServerResponse},
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
}

pub struct HCloudClient {
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
