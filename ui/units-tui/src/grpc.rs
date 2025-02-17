pub mod proto_types {
    tonic::include_proto!("finternet");
}

#[allow(dead_code)]
pub struct Clients {
    pub driver_client: proto_types::driver_client::DriverClient<tonic::transport::Channel>,
    pub bind_client: proto_types::bind_client::BindClient<tonic::transport::Channel>,
    pub exec_client: proto_types::execution_client::ExecutionClient<tonic::transport::Channel>,
    pub driver_detail_client:
        proto_types::driver_details_client::DriverDetailsClient<tonic::transport::Channel>,
    pub user_signup_client:
        proto_types::user_sign_up_client::UserSignUpClient<tonic::transport::Channel>,
    pub user_login_client:
        proto_types::user_login_client::UserLoginClient<tonic::transport::Channel>,
    pub user_check_client:
        proto_types::user_check_client::UserCheckClient<tonic::transport::Channel>,
}

impl Clients {
    pub async fn new(addr: String) -> anyhow::Result<Self> {
        let endpoint = tonic::transport::Endpoint::try_from(addr.clone())?;
        let driver_client = proto_types::driver_client::DriverClient::connect(endpoint).await?;
        let endpoint = tonic::transport::Endpoint::try_from(addr.clone())?;
        let bind_client = proto_types::bind_client::BindClient::connect(endpoint).await?;
        let endpoint = tonic::transport::Endpoint::try_from(addr.clone())?;
        let exec_client = proto_types::execution_client::ExecutionClient::connect(endpoint).await?;
        let endpoint = tonic::transport::Endpoint::try_from(addr.clone())?;
        let driver_detail_client =
            proto_types::driver_details_client::DriverDetailsClient::connect(endpoint).await?;
        let endpoint = tonic::transport::Endpoint::try_from(addr.clone())?;
        let user_signup_client =
            proto_types::user_sign_up_client::UserSignUpClient::connect(endpoint).await?;
        let endpoint = tonic::transport::Endpoint::try_from(addr.clone())?;
        let user_login_client =
            proto_types::user_login_client::UserLoginClient::connect(endpoint).await?;
        let endpoint = tonic::transport::Endpoint::try_from(addr.clone())?;
        let user_check_client =
            proto_types::user_check_client::UserCheckClient::connect(endpoint).await?;

        Ok(Self {
            driver_client,
            bind_client,
            exec_client,
            driver_detail_client,
            user_signup_client,
            user_login_client,
            user_check_client,
        })
    }
}
