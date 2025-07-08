tonic::include_proto!("api");

use bytes::Bytes;
use http::HeaderValue;
use tonic::{
  Request, Status,
  transport::{Channel, Endpoint, Error},
};

use crate::api_client::ApiClient;

type Interceptor = fn(Request<()>) -> Result<Request<()>, Status>;
pub type ApiClientWithHeader =
  ApiClient<tonic::service::interceptor::InterceptedService<Channel, Interceptor>>;

pub async fn conn(addr: impl Into<Bytes>) -> Result<ApiClient<Channel>, Error> {
  let endpoint = Endpoint::from_shared(addr.into())?.user_agent(
    HeaderValue::from_bytes(
      const_str::concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")).as_bytes(),
    )
    .unwrap(),
  )?;

  let client = ApiClient::connect(endpoint).await?;

  Ok(client)
}
