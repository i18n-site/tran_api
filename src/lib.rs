tonic::include_proto!("api");
use bytes::Bytes;
use grpc_client::endpoint;
use tonic::{
  Request, Status,
  transport::{Channel, Error},
};

use crate::api_client::ApiClient;

#[allow(clippy::result_large_err)]
fn req_interceptor(mut req: Request<()>) -> Result<Request<()>, Status> {
  let meta = req.metadata_mut();
  // meta.remove("user-agent");
  meta.insert(
    "t",
    tonic::metadata::AsciiMetadataValue::from_key::<tonic::metadata::Ascii>(
      "11xvw".parse().unwrap(),
    ),
  );
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
