tonic::include_proto!("api");

use bytes::Bytes;
use tonic::{
  service::Interceptor,
  transport::{Channel, Endpoint, Error},
};

use crate::api_client::ApiClient;

pub async fn conn<F>(
  addr: impl Into<Bytes>,
  interceptor: F,
) -> Result<ApiClient<tonic::service::interceptor::InterceptedService<Channel, F>>, Error>
where
  F: Interceptor,
{
  let endpoint = Endpoint::from_shared(addr.into())?;
  let channel = endpoint.connect().await?;

  let client = ApiClient::with_interceptor(channel, interceptor);

  Ok(client)
}
