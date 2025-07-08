tonic::include_proto!("api");

use bytes::Bytes;
use grpc_client::endpoint;
use tonic::{
  Request, Status,
  transport::{Channel, Error},
};

use crate::api_client::ApiClient;

type Interceptor = fn(Request<()>) -> Result<Request<()>, Status>;
pub type ApiClientWithHeader =
  ApiClient<tonic::service::interceptor::InterceptedService<Channel, Interceptor>>;

pub async fn conn(addr: impl Into<Bytes>) -> Result<ApiClientWithHeader, Error> {
  let channel = endpoint(addr)?;

  let client = ApiClient::connect(channel).await?;

  Ok(client)
}
