tonic::include_proto!("api");
use bytes::Bytes;
use grpc_client::endpoint;
use tonic::{
  Request, Status,
  metadata::MetadataValue,
  transport::{Channel, Error},
};

use crate::api_client::ApiClient;

fn req_interceptor(mut req: Request<()>) -> Result<Request<()>, Status> {
  req
    .metadata_mut()
    .insert("x", MetadataValue::from_static("1"));
  Ok(req)
}
type Interceptor = fn(Request<()>) -> Result<Request<()>, Status>;
pub type ApiClientWithHeader =
  ApiClient<tonic::service::interceptor::InterceptedService<Channel, Interceptor>>;

pub async fn conn(addr: impl Into<Bytes>) -> Result<ApiClientWithHeader, Error> {
  let channel = endpoint(addr)?.connect().await?;

  let client = ApiClient::with_interceptor(channel, req_interceptor as Interceptor);

  Ok(client)
}

