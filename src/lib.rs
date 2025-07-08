tonic::include_proto!("api");
use bytes::Bytes;
use grpc_client::endpoint;
use tonic::{
  codec::CompressionEncoding::Gzip,
  transport::{Channel, Error},
};

use crate::api_client::ApiClient;

pub fn req_interceptor(mut req: tonic::Request<()>) -> Result<tonic::Request<()>, Error> {
  req
    .metadata_mut()
    .insert("Authorization", "XXXXXXXXXXXX".try_into().unwrap());
  Ok(req)
}
pub async fn conn(addr: impl Into<Bytes>) -> Result<ApiClient<Channel>, Error> {
  let channel = endpoint(addr)?;

  let client = ApiClient::connect(channel)
    .await?
    .send_compressed(Gzip)
    .accept_compressed(Gzip);

  Ok(client)
}
